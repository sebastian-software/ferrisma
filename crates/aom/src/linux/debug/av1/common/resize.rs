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

// Mechanically translated from upstream/aom/av1/common/resize.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_convolve_horiz_rs_c(
        src: *const uint8_t,
        src_stride: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        x_filters: *const int16_t,
        x0_qn: core::ffi::c_int,
        x_step_qn: core::ffi::c_int,
    );
    fn av1_highbd_convolve_horiz_rs_c(
        src: *const uint16_t,
        src_stride: core::ffi::c_int,
        dst: *mut uint16_t,
        dst_stride: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        x_filters: *const int16_t,
        x0_qn: core::ffi::c_int,
        x_step_qn: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_alloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        border: core::ffi::c_int,
        byte_alignment: core::ffi::c_int,
        alloc_pyramid: bool,
        alloc_y_plane_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
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
    fn aom_free_frame_buffer(ybf: *mut YV12_BUFFER_CONFIG) -> core::ffi::c_int;
    fn aom_copy_metadata_to_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        arr: *const aom_metadata_array_t,
    ) -> core::ffi::c_int;
    fn aom_extend_frame_borders_c(
        ybf: *mut yv12_buffer_config,
        num_planes: core::ffi::c_int,
    );
    fn aom_yv12_copy_frame_c(
        src_bc: *const yv12_buffer_config,
        dst_bc: *mut yv12_buffer_config,
        num_planes: core::ffi::c_int,
    );
    fn av1_tile_set_col(
        tile: *mut TileInfo,
        cm: *const AV1Common,
        col: core::ffi::c_int,
    );
    fn aom_scaled_2d_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        filter: *const InterpKernel,
        x0_q4: core::ffi::c_int,
        x_step_q4: core::ffi::c_int,
        y0_q4: core::ffi::c_int,
        y_step_q4: core::ffi::c_int,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
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
pub struct InterpFilterParams {
    pub filter_ptr: *const int16_t,
    pub taps: uint16_t,
    pub interp_filter: InterpFilter,
}
pub type AV1_COMMON = AV1Common;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut av1_down2_symeven_half_filter: [int16_t; 4] = [
    56 as core::ffi::c_int as int16_t,
    12 as core::ffi::c_int as int16_t,
    -(3 as core::ffi::c_int) as int16_t,
    -(1 as core::ffi::c_int) as int16_t,
];
static mut av1_down2_symodd_half_filter: [int16_t; 4] = [
    64 as core::ffi::c_int as int16_t,
    35 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    -(3 as core::ffi::c_int) as int16_t,
];
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
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
unsafe extern "C-unwind" fn lock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_lock(&raw mut (*pool).pool_mutex);
}
unsafe extern "C-unwind" fn unlock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_unlock(&raw mut (*pool).pool_mutex);
}
#[inline]
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
static mut av1_bilinear_filters: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        104 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        104 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_8: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        122 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        110 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        102 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        94 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        76 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        66 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        102 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        122 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_8sharp: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        124 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        120 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        108 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        100 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        90 as core::ffi::c_int as int16_t,
        70 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(4 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        80 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        70 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        60 as core::ffi::c_int as int16_t,
        100 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        108 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(22 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        26 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        124 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
]);
static mut av1_sub_pel_filters_8smooth: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        14 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_12sharp: crate::platform::Aligned256<[[int16_t; 12]; 16]> = crate::platform::Aligned256([
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        127 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        124 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        120 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        115 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        108 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        100 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(26 as core::ffi::c_int) as int16_t,
        91 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        81 as core::ffi::c_int as int16_t,
        81 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        71 as core::ffi::c_int as int16_t,
        91 as core::ffi::c_int as int16_t,
        -(26 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        60 as core::ffi::c_int as int16_t,
        100 as core::ffi::c_int as int16_t,
        -(25 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        49 as core::ffi::c_int as int16_t,
        108 as core::ffi::c_int as int16_t,
        -(24 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        115 as core::ffi::c_int as int16_t,
        -(21 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        124 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        127 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_interp_filter_params_list: [InterpFilterParams; 5] = unsafe {
    [
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_8.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_REGULAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_8smooth.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_SMOOTH,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_8sharp.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: MULTITAP_SHARP,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_bilinear_filters.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: BILINEAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_12sharp.0).cast::<[int16_t; 12]>() as *const int16_t,
                taps: 12 as uint16_t,
                interp_filter: MULTITAP_SHARP2,
            };
            init
        },
    ]
};
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
static mut filteredinterp_filters500: [InterpKernel; 64] = [
    [
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        35 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        35 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(3 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        34 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(3 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        33 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        37 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(3 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        32 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(3 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        31 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        39 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(3 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        31 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        39 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        30 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        29 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        41 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        29 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        41 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        27 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        27 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        26 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        25 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        45 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        25 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        45 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        24 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        23 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        47 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        23 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        47 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        22 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        21 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        20 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        20 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        50 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        19 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        51 as core::ffi::c_int as int16_t,
        9 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        19 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        51 as core::ffi::c_int as int16_t,
        9 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        17 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        14 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        14 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        17 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        9 as core::ffi::c_int as int16_t,
        51 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        19 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        9 as core::ffi::c_int as int16_t,
        51 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        19 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        50 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        21 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        47 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        23 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        47 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        23 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        45 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        25 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        45 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        25 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        27 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        27 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        41 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        29 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        41 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        29 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        30 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        39 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        31 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        39 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        31 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        37 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        33 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
    ],
];
static mut filteredinterp_filters625: [InterpKernel; 64] = [
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        33 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        33 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        31 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        30 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        35 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        29 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        37 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        27 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        26 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        39 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        25 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        24 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        41 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        23 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        22 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        21 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        20 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        45 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        19 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        47 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        17 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        16 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        50 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        51 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        75 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        14 as core::ffi::c_int as int16_t,
        74 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        74 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        73 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        73 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        9 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        70 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        69 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        68 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        68 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        67 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        65 as core::ffi::c_int as int16_t,
        65 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        67 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        68 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        62 as core::ffi::c_int as int16_t,
        68 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        69 as core::ffi::c_int as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        70 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        9 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        57 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        56 as core::ffi::c_int as int16_t,
        73 as core::ffi::c_int as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        55 as core::ffi::c_int as int16_t,
        73 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        54 as core::ffi::c_int as int16_t,
        74 as core::ffi::c_int as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        53 as core::ffi::c_int as int16_t,
        74 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        52 as core::ffi::c_int as int16_t,
        75 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        51 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        50 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        16 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        49 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        17 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        47 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        19 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        45 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        44 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        21 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        43 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        42 as core::ffi::c_int as int16_t,
        78 as core::ffi::c_int as int16_t,
        23 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        41 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        40 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        25 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        39 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        27 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        37 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        36 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        29 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        35 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        30 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        34 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        31 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
];
static mut filteredinterp_filters750: [InterpKernel; 64] = [
    [
        2 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        25 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        25 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        24 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        22 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        21 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        29 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        19 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        31 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        17 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        35 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        14 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        37 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        93 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        93 as core::ffi::c_int as int16_t,
        41 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        9 as core::ffi::c_int as int16_t,
        92 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        92 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        91 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        47 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        49 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        89 as core::ffi::c_int as int16_t,
        50 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        87 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        86 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        85 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        83 as core::ffi::c_int as int16_t,
        60 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        82 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        81 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        80 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        79 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        77 as core::ffi::c_int as int16_t,
        67 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        76 as core::ffi::c_int as int16_t,
        69 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        75 as core::ffi::c_int as int16_t,
        70 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        74 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        72 as core::ffi::c_int as int16_t,
        72 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        71 as core::ffi::c_int as int16_t,
        74 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        70 as core::ffi::c_int as int16_t,
        75 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        69 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        67 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        66 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        64 as core::ffi::c_int as int16_t,
        80 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        63 as core::ffi::c_int as int16_t,
        81 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        61 as core::ffi::c_int as int16_t,
        82 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        60 as core::ffi::c_int as int16_t,
        83 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        57 as core::ffi::c_int as int16_t,
        85 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        55 as core::ffi::c_int as int16_t,
        86 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        54 as core::ffi::c_int as int16_t,
        87 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        52 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        50 as core::ffi::c_int as int16_t,
        89 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        49 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        47 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        46 as core::ffi::c_int as int16_t,
        91 as core::ffi::c_int as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        44 as core::ffi::c_int as int16_t,
        92 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        43 as core::ffi::c_int as int16_t,
        92 as core::ffi::c_int as int16_t,
        9 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        41 as core::ffi::c_int as int16_t,
        93 as core::ffi::c_int as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        40 as core::ffi::c_int as int16_t,
        93 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        37 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        35 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        34 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        17 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        32 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        31 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        19 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        29 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        21 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        26 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
];
static mut filteredinterp_filters875: [InterpKernel; 64] = [
    [
        3 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        12 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        3 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        10 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        17 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        19 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        21 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        111 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        111 as core::ffi::c_int as int16_t,
        24 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        26 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        2 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        109 as core::ffi::c_int as int16_t,
        30 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        108 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        108 as core::ffi::c_int as int16_t,
        34 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        107 as core::ffi::c_int as int16_t,
        36 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        106 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        105 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        104 as core::ffi::c_int as int16_t,
        42 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        103 as core::ffi::c_int as int16_t,
        44 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        101 as core::ffi::c_int as int16_t,
        46 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        100 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        99 as core::ffi::c_int as int16_t,
        50 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        97 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        96 as core::ffi::c_int as int16_t,
        55 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        95 as core::ffi::c_int as int16_t,
        57 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        93 as core::ffi::c_int as int16_t,
        59 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        91 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        90 as core::ffi::c_int as int16_t,
        63 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        88 as core::ffi::c_int as int16_t,
        65 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        86 as core::ffi::c_int as int16_t,
        67 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        69 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        83 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        81 as core::ffi::c_int as int16_t,
        73 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        79 as core::ffi::c_int as int16_t,
        75 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        77 as core::ffi::c_int as int16_t,
        77 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        75 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        73 as core::ffi::c_int as int16_t,
        81 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        71 as core::ffi::c_int as int16_t,
        83 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        69 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        67 as core::ffi::c_int as int16_t,
        86 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        65 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        63 as core::ffi::c_int as int16_t,
        90 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        61 as core::ffi::c_int as int16_t,
        91 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        59 as core::ffi::c_int as int16_t,
        93 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        57 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        55 as core::ffi::c_int as int16_t,
        96 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        53 as core::ffi::c_int as int16_t,
        97 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        50 as core::ffi::c_int as int16_t,
        99 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        100 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        46 as core::ffi::c_int as int16_t,
        101 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        44 as core::ffi::c_int as int16_t,
        103 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        42 as core::ffi::c_int as int16_t,
        104 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        40 as core::ffi::c_int as int16_t,
        105 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        106 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        36 as core::ffi::c_int as int16_t,
        107 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        34 as core::ffi::c_int as int16_t,
        108 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        32 as core::ffi::c_int as int16_t,
        108 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        30 as core::ffi::c_int as int16_t,
        109 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        26 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        24 as core::ffi::c_int as int16_t,
        111 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        22 as core::ffi::c_int as int16_t,
        111 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        21 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        19 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        17 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        10 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
    ],
];
#[no_mangle]
pub static mut av1_resize_filter_normative: [[int16_t; 8]; 64] = [
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        128 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        127 as core::ffi::c_int as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        127 as core::ffi::c_int as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        125 as core::ffi::c_int as int16_t,
        11 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        125 as core::ffi::c_int as int16_t,
        13 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        124 as core::ffi::c_int as int16_t,
        15 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        123 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        122 as core::ffi::c_int as int16_t,
        20 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        121 as core::ffi::c_int as int16_t,
        22 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        120 as core::ffi::c_int as int16_t,
        25 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        118 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        117 as core::ffi::c_int as int16_t,
        30 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        32 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        114 as core::ffi::c_int as int16_t,
        35 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        112 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        111 as core::ffi::c_int as int16_t,
        40 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        109 as core::ffi::c_int as int16_t,
        43 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        107 as core::ffi::c_int as int16_t,
        45 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        105 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        103 as core::ffi::c_int as int16_t,
        51 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        101 as core::ffi::c_int as int16_t,
        53 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        99 as core::ffi::c_int as int16_t,
        56 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        97 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        95 as core::ffi::c_int as int16_t,
        61 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        93 as core::ffi::c_int as int16_t,
        64 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        91 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        88 as core::ffi::c_int as int16_t,
        69 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        86 as core::ffi::c_int as int16_t,
        71 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        74 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        81 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        79 as core::ffi::c_int as int16_t,
        79 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        76 as core::ffi::c_int as int16_t,
        81 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        74 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        71 as core::ffi::c_int as int16_t,
        86 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        69 as core::ffi::c_int as int16_t,
        88 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        66 as core::ffi::c_int as int16_t,
        91 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(2 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        64 as core::ffi::c_int as int16_t,
        93 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        7 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        61 as core::ffi::c_int as int16_t,
        95 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        97 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        56 as core::ffi::c_int as int16_t,
        99 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        53 as core::ffi::c_int as int16_t,
        101 as core::ffi::c_int as int16_t,
        -(20 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        51 as core::ffi::c_int as int16_t,
        103 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        105 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        45 as core::ffi::c_int as int16_t,
        107 as core::ffi::c_int as int16_t,
        -(19 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        43 as core::ffi::c_int as int16_t,
        109 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        40 as core::ffi::c_int as int16_t,
        111 as core::ffi::c_int as int16_t,
        -(18 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        112 as core::ffi::c_int as int16_t,
        -(17 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        35 as core::ffi::c_int as int16_t,
        114 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        32 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(16 as core::ffi::c_int) as int16_t,
        5 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        30 as core::ffi::c_int as int16_t,
        117 as core::ffi::c_int as int16_t,
        -(15 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        118 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        25 as core::ffi::c_int as int16_t,
        120 as core::ffi::c_int as int16_t,
        -(13 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        22 as core::ffi::c_int as int16_t,
        121 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        20 as core::ffi::c_int as int16_t,
        122 as core::ffi::c_int as int16_t,
        -(11 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        123 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        15 as core::ffi::c_int as int16_t,
        124 as core::ffi::c_int as int16_t,
        -(9 as core::ffi::c_int) as int16_t,
        3 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(5 as core::ffi::c_int) as int16_t,
        13 as core::ffi::c_int as int16_t,
        125 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        11 as core::ffi::c_int as int16_t,
        125 as core::ffi::c_int as int16_t,
        -(7 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        6 as core::ffi::c_int as int16_t,
        127 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        4 as core::ffi::c_int as int16_t,
        127 as core::ffi::c_int as int16_t,
        -(3 as core::ffi::c_int) as int16_t,
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        2 as core::ffi::c_int as int16_t,
        128 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
];
unsafe extern "C-unwind" fn choose_interp_filter(
    mut in_length: core::ffi::c_int,
    mut out_length: core::ffi::c_int,
) -> *const InterpKernel {
    let mut out_length16: core::ffi::c_int = out_length * 16 as core::ffi::c_int;
    if out_length16 >= in_length * 16 as core::ffi::c_int {
        return av1_resize_filter_normative.as_ptr() as *const InterpKernel
    } else if out_length16 >= in_length * 13 as core::ffi::c_int {
        return core::ptr::addr_of!(filteredinterp_filters875).cast::<InterpKernel>()
    } else if out_length16 >= in_length * 11 as core::ffi::c_int {
        return core::ptr::addr_of!(filteredinterp_filters750).cast::<InterpKernel>()
    } else if out_length16 >= in_length * 9 as core::ffi::c_int {
        return core::ptr::addr_of!(filteredinterp_filters625).cast::<InterpKernel>()
    } else {
        return core::ptr::addr_of!(filteredinterp_filters500).cast::<InterpKernel>()
    };
}
unsafe extern "C-unwind" fn interpolate_core(
    input: *const uint8_t,
    mut in_length: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut out_length: core::ffi::c_int,
    mut interp_filters: *const int16_t,
    mut interp_taps: core::ffi::c_int,
) {
    let delta: int32_t = ((in_length as uint32_t) << RS_SCALE_SUBPEL_BITS)
        .wrapping_add((out_length / 2 as core::ffi::c_int) as uint32_t)
        .wrapping_div(out_length as uint32_t) as int32_t;
    let offset: int32_t = if in_length > out_length {
        ((((in_length - out_length) as int32_t)
            << RS_SCALE_SUBPEL_BITS - 1 as core::ffi::c_int)
            + out_length as int32_t / 2 as int32_t) / out_length as int32_t
    } else {
        -((((out_length - in_length) as int32_t)
            << RS_SCALE_SUBPEL_BITS - 1 as core::ffi::c_int)
            + out_length as int32_t / 2 as int32_t) / out_length as int32_t
    };
    let mut optr: *mut uint8_t = output;
    let mut x: core::ffi::c_int = 0;
    let mut x1: core::ffi::c_int = 0;
    let mut x2: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut int_pel: core::ffi::c_int = 0;
    let mut sub_pel: core::ffi::c_int = 0;
    let mut y: int32_t = 0;
    x = 0 as core::ffi::c_int;
    y = offset + RS_SCALE_EXTRA_OFF as int32_t;
    while y >> RS_SCALE_SUBPEL_BITS
        < interp_taps as int32_t / 2 as int32_t - 1 as int32_t
    {
        x += 1;
        y += delta;
    }
    x1 = x;
    x = out_length - 1 as core::ffi::c_int;
    y = delta * x as int32_t + offset + RS_SCALE_EXTRA_OFF as int32_t;
    while (y >> RS_SCALE_SUBPEL_BITS) + (interp_taps / 2 as core::ffi::c_int) as int32_t
        >= in_length as int32_t
    {
        x -= 1;
        y -= delta;
    }
    x2 = x;
    if x1 > x2 {
        x = 0 as core::ffi::c_int;
        y = offset + RS_SCALE_EXTRA_OFF as int32_t;
        while x < out_length {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                let pk: core::ffi::c_int = int_pel - interp_taps / 2 as core::ffi::c_int
                    + 1 as core::ffi::c_int + k;
                sum
                    += *filter.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                clamp(
                                    pk,
                                    0 as core::ffi::c_int,
                                    in_length - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh7 = optr;
            optr = optr.offset(1);
            *fresh7 = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x += 1;
            y += delta;
        }
    } else {
        x = 0 as core::ffi::c_int;
        y = offset + RS_SCALE_EXTRA_OFF as int32_t;
        while x < x1 {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter_0: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                sum
                    += *filter_0.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                (if int_pel - interp_taps / 2 as core::ffi::c_int
                                    + 1 as core::ffi::c_int + k > 0 as core::ffi::c_int
                                {
                                    int_pel - interp_taps / 2 as core::ffi::c_int
                                        + 1 as core::ffi::c_int + k
                                } else {
                                    0 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh8 = optr;
            optr = optr.offset(1);
            *fresh8 = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x += 1;
            y += delta;
        }
        while x <= x2 {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter_1: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                sum
                    += *filter_1.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                (int_pel - interp_taps / 2 as core::ffi::c_int
                                    + 1 as core::ffi::c_int + k) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh9 = optr;
            optr = optr.offset(1);
            *fresh9 = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x += 1;
            y += delta;
        }
        while x < out_length {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter_2: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                sum
                    += *filter_2.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                (if int_pel - interp_taps / 2 as core::ffi::c_int
                                    + 1 as core::ffi::c_int + k
                                    < in_length - 1 as core::ffi::c_int
                                {
                                    int_pel - interp_taps / 2 as core::ffi::c_int
                                        + 1 as core::ffi::c_int + k
                                } else {
                                    in_length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh10 = optr;
            optr = optr.offset(1);
            *fresh10 = clip_pixel(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
            );
            x += 1;
            y += delta;
        }
    };
}
unsafe extern "C-unwind" fn interpolate(
    input: *const uint8_t,
    mut in_length: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut out_length: core::ffi::c_int,
) {
    let mut interp_filters: *const InterpKernel = choose_interp_filter(
        in_length,
        out_length,
    );
    interpolate_core(
        input,
        in_length,
        output,
        out_length,
        &*(*interp_filters.offset(0 as core::ffi::c_int as isize))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize),
        SUBPEL_TAPS,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_upscale_convolve_step(
    mut in_length: core::ffi::c_int,
    mut out_length: core::ffi::c_int,
) -> int32_t {
    return (((in_length as int32_t) << RS_SCALE_SUBPEL_BITS)
        + out_length as int32_t / 2 as int32_t) / out_length as int32_t;
}
unsafe extern "C-unwind" fn get_upscale_convolve_x0(
    mut in_length: core::ffi::c_int,
    mut out_length: core::ffi::c_int,
    mut x_step_qn: int32_t,
) -> int32_t {
    let err: core::ffi::c_int = out_length * x_step_qn as core::ffi::c_int
        - (in_length << RS_SCALE_SUBPEL_BITS);
    let x0: int32_t = (-((out_length as int32_t - in_length as int32_t)
        << RS_SCALE_SUBPEL_BITS - 1 as core::ffi::c_int)
        + out_length as int32_t / 2 as int32_t) / out_length as int32_t
        + RS_SCALE_EXTRA_OFF as int32_t - err as int32_t / 2 as int32_t;
    return (x0 as uint32_t & RS_SCALE_SUBPEL_MASK as uint32_t) as int32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn down2_symeven(
    input: *const uint8_t,
    mut length: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut start_offset: core::ffi::c_int,
) {
    let mut filter: *const int16_t = core::ptr::addr_of!(av1_down2_symeven_half_filter).cast::<int16_t>();
    let filter_len_half: core::ffi::c_int = (::core::mem::size_of::<[int16_t; 4]>()
        as usize)
        .wrapping_div(2 as usize) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut optr: *mut uint8_t = output;
    let mut l1: core::ffi::c_int = filter_len_half;
    let mut l2: core::ffi::c_int = length - filter_len_half;
    l1 += l1 & 1 as core::ffi::c_int;
    l2 += l2 & 1 as core::ffi::c_int;
    if l1 > l2 {
        i = start_offset;
        while i < length {
            let mut sum: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum
                    += (*input
                        .offset(
                            (if i - j > 0 as core::ffi::c_int {
                                i - j
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + 1 as core::ffi::c_int + j
                                    < length - 1 as core::ffi::c_int
                                {
                                    i + 1 as core::ffi::c_int + j
                                } else {
                                    length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum >>= FILTER_BITS;
            let fresh1 = optr;
            optr = optr.offset(1);
            *fresh1 = clip_pixel(sum);
            i += 2 as core::ffi::c_int;
        }
    } else {
        i = start_offset;
        while i < l1 {
            let mut sum_0: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum_0
                    += (*input
                        .offset(
                            (if i - j > 0 as core::ffi::c_int {
                                i - j
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input.offset((i + 1 as core::ffi::c_int + j) as isize)
                            as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_0 >>= FILTER_BITS;
            let fresh2 = optr;
            optr = optr.offset(1);
            *fresh2 = clip_pixel(sum_0);
            i += 2 as core::ffi::c_int;
        }
        while i < l2 {
            let mut sum_1: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum_1
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input.offset((i + 1 as core::ffi::c_int + j) as isize)
                            as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_1 >>= FILTER_BITS;
            let fresh3 = optr;
            optr = optr.offset(1);
            *fresh3 = clip_pixel(sum_1);
            i += 2 as core::ffi::c_int;
        }
        while i < length {
            let mut sum_2: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum_2
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + 1 as core::ffi::c_int + j
                                    < length - 1 as core::ffi::c_int
                                {
                                    i + 1 as core::ffi::c_int + j
                                } else {
                                    length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_2 >>= FILTER_BITS;
            let fresh4 = optr;
            optr = optr.offset(1);
            *fresh4 = clip_pixel(sum_2);
            i += 2 as core::ffi::c_int;
        }
    };
}
unsafe extern "C-unwind" fn down2_symodd(
    input: *const uint8_t,
    mut length: core::ffi::c_int,
    mut output: *mut uint8_t,
) {
    let mut filter: *const int16_t = core::ptr::addr_of!(av1_down2_symodd_half_filter).cast::<int16_t>();
    let filter_len_half: core::ffi::c_int = (::core::mem::size_of::<[int16_t; 4]>()
        as usize)
        .wrapping_div(2 as usize) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut optr: *mut uint8_t = output;
    let mut l1: core::ffi::c_int = filter_len_half - 1 as core::ffi::c_int;
    let mut l2: core::ffi::c_int = length - filter_len_half + 1 as core::ffi::c_int;
    l1 += l1 & 1 as core::ffi::c_int;
    l2 += l2 & 1 as core::ffi::c_int;
    if l1 > l2 {
        i = 0 as core::ffi::c_int;
        while i < length {
            let mut sum: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum
                    += (*input
                        .offset(
                            (if i - j < 0 as core::ffi::c_int {
                                0 as core::ffi::c_int
                            } else {
                                i - j
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + j >= length {
                                    length - 1 as core::ffi::c_int
                                } else {
                                    i + j
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum >>= FILTER_BITS;
            let fresh11 = optr;
            optr = optr.offset(1);
            *fresh11 = clip_pixel(sum);
            i += 2 as core::ffi::c_int;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < l1 {
            let mut sum_0: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum_0
                    += (*input
                        .offset(
                            (if i - j < 0 as core::ffi::c_int {
                                0 as core::ffi::c_int
                            } else {
                                i - j
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input.offset((i + j) as isize) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_0 >>= FILTER_BITS;
            let fresh12 = optr;
            optr = optr.offset(1);
            *fresh12 = clip_pixel(sum_0);
            i += 2 as core::ffi::c_int;
        }
        while i < l2 {
            let mut sum_1: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum_1
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input.offset((i + j) as isize) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_1 >>= FILTER_BITS;
            let fresh13 = optr;
            optr = optr.offset(1);
            *fresh13 = clip_pixel(sum_1);
            i += 2 as core::ffi::c_int;
        }
        while i < length {
            let mut sum_2: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum_2
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + j >= length {
                                    length - 1 as core::ffi::c_int
                                } else {
                                    i + j
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_2 >>= FILTER_BITS;
            let fresh14 = optr;
            optr = optr.offset(1);
            *fresh14 = clip_pixel(sum_2);
            i += 2 as core::ffi::c_int;
        }
    };
}
unsafe extern "C-unwind" fn get_down2_length(
    mut length: core::ffi::c_int,
    mut steps: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
    while s < steps {
        length = length + 1 as core::ffi::c_int >> 1 as core::ffi::c_int;
        s += 1;
    }
    return length;
}
unsafe extern "C-unwind" fn get_down2_steps(
    mut in_length: core::ffi::c_int,
    mut out_length: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut steps: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut proj_in_length: core::ffi::c_int = 0;
    loop {
        proj_in_length = get_down2_length(in_length, 1 as core::ffi::c_int);
        if !(proj_in_length >= out_length) {
            break;
        }
        steps += 1;
        in_length = proj_in_length;
        if in_length == 1 as core::ffi::c_int {
            break;
        }
    }
    return steps;
}
unsafe extern "C-unwind" fn resize_multistep(
    input: *const uint8_t,
    mut length: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut olength: core::ffi::c_int,
    mut otmp: *mut uint8_t,
) {
    if length == olength {
        memcpy(
            output as *mut core::ffi::c_void,
            input as *const core::ffi::c_void,
            (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(length as size_t),
        );
        return;
    }
    let steps: core::ffi::c_int = get_down2_steps(length, olength) as core::ffi::c_int;
    if steps > 0 as core::ffi::c_int {
        let mut out: *mut uint8_t = 0 as *mut uint8_t;
        let mut filteredlength: core::ffi::c_int = length;
        if !otmp.is_null() {} else {
            __assert_fail(
                b"otmp != NULL\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/resize.c\0" as *const u8
                    as *const core::ffi::c_char,
                482 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 76],
                    [core::ffi::c_char; 76],
                >(
                    *b"void resize_multistep(const uint8_t *const, int, uint8_t *, int, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        };
        let mut otmp2: *mut uint8_t = otmp
            .offset(get_down2_length(length, 1 as core::ffi::c_int) as isize);
        let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
        while s < steps {
            let proj_filteredlength: core::ffi::c_int = get_down2_length(
                filteredlength,
                1 as core::ffi::c_int,
            ) as core::ffi::c_int;
            let in_0: *const uint8_t = if s == 0 as core::ffi::c_int {
                input
            } else {
                out as *const uint8_t
            };
            if s == steps - 1 as core::ffi::c_int && proj_filteredlength == olength {
                out = output;
            } else {
                out = if s & 1 as core::ffi::c_int != 0 { otmp2 } else { otmp };
            }
            if filteredlength & 1 as core::ffi::c_int != 0 {
                down2_symodd(in_0, filteredlength, out);
            } else {
                down2_symeven(in_0, filteredlength, out, 0 as core::ffi::c_int);
            }
            filteredlength = proj_filteredlength;
            s += 1;
        }
        if filteredlength != olength {
            interpolate(out, filteredlength, output, olength);
        }
    } else {
        interpolate(input, length, output, olength);
    };
}
unsafe extern "C-unwind" fn fill_col_to_arr(
    mut img: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut len: core::ffi::c_int,
    mut arr: *mut uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut iptr: *mut uint8_t = img;
    let mut aptr: *mut uint8_t = arr;
    i = 0 as core::ffi::c_int;
    while i < len {
        let fresh6 = aptr;
        aptr = aptr.offset(1);
        *fresh6 = *iptr;
        i += 1;
        iptr = iptr.offset(stride as isize);
    }
}
unsafe extern "C-unwind" fn fill_arr_to_col(
    mut img: *mut uint8_t,
    mut stride: core::ffi::c_int,
    mut len: core::ffi::c_int,
    mut arr: *mut uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut iptr: *mut uint8_t = img;
    let mut aptr: *mut uint8_t = arr;
    i = 0 as core::ffi::c_int;
    while i < len {
        let fresh5 = aptr;
        aptr = aptr.offset(1);
        *iptr = *fresh5;
        i += 1;
        iptr = iptr.offset(stride as isize);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_resize_vert_dir_c(
    mut intbuf: *mut uint8_t,
    mut output: *mut uint8_t,
    mut out_stride: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
    mut start_col: core::ffi::c_int,
) -> bool {
    let mut mem_status: bool = true_0 != 0;
    let mut arrbuf: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(height as size_t),
    ) as *mut uint8_t;
    let mut arrbuf2: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(height2 as size_t),
    ) as *mut uint8_t;
    if arrbuf.is_null() || arrbuf2.is_null() {
        mem_status = false_0 != 0;
    } else {
        let mut i: core::ffi::c_int = start_col;
        while i < width2 {
            fill_col_to_arr(intbuf.offset(i as isize), width2, height, arrbuf);
            down2_symeven(arrbuf, height, arrbuf2, 0 as core::ffi::c_int);
            fill_arr_to_col(output.offset(i as isize), out_stride, height2, arrbuf2);
            i += 1;
        }
    }
    aom_free(arrbuf as *mut core::ffi::c_void);
    aom_free(arrbuf2 as *mut core::ffi::c_void);
    return mem_status;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_resize_horz_dir_c(
    input: *const uint8_t,
    mut in_stride: core::ffi::c_int,
    mut intbuf: *mut uint8_t,
    mut height: core::ffi::c_int,
    mut filtered_length: core::ffi::c_int,
    mut width2: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        down2_symeven(
            input.offset((in_stride * i) as isize),
            filtered_length,
            intbuf.offset((width2 * i) as isize),
            0 as core::ffi::c_int,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_resize_plane_to_half(
    input: *const uint8_t,
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
    mut out_stride: core::ffi::c_int,
) -> bool {
    let mut intbuf: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(width2 as size_t)
            .wrapping_mul(height as size_t),
    ) as *mut uint8_t;
    if intbuf.is_null() {
        return false_0 != 0;
    }
    av1_resize_horz_dir_c(input, in_stride, intbuf, height, width, width2);
    let mut mem_status: bool = av1_resize_vert_dir_c(
        intbuf,
        output,
        out_stride,
        height,
        height2,
        width2,
        0 as core::ffi::c_int,
    );
    aom_free(intbuf as *mut core::ffi::c_void);
    return mem_status;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn should_resize_by_half(
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
) -> bool {
    let is_width_by_2: bool = get_down2_length(width, 1 as core::ffi::c_int) == width2;
    let is_height_by_2: bool = get_down2_length(height, 1 as core::ffi::c_int)
        == height2;
    return is_width_by_2 as core::ffi::c_int != 0
        && is_height_by_2 as core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_resize_plane(
    mut input: *const uint8_t,
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
    mut out_stride: core::ffi::c_int,
) -> bool {
    let mut i: core::ffi::c_int = 0;
    let mut mem_status: bool = true_0 != 0;
    let mut intbuf: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul(width2 as size_t)
            .wrapping_mul(height as size_t),
    ) as *mut uint8_t;
    let mut tmpbuf: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t)
            .wrapping_mul((if width > height { width } else { height }) as size_t),
    ) as *mut uint8_t;
    let mut arrbuf: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(height as size_t),
    ) as *mut uint8_t;
    let mut arrbuf2: *mut uint8_t = aom_malloc(
        (::core::mem::size_of::<uint8_t>() as size_t).wrapping_mul(height2 as size_t),
    ) as *mut uint8_t;
    if intbuf.is_null() || tmpbuf.is_null() || arrbuf.is_null() || arrbuf2.is_null() {
        mem_status = false_0 != 0;
    } else {
        if width > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"width > 0\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/resize.c\0" as *const u8
                    as *const core::ffi::c_char,
                592 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 81],
                    [core::ffi::c_char; 81],
                >(
                    *b"_Bool av1_resize_plane(const uint8_t *, int, int, int, uint8_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        if height > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"height > 0\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/resize.c\0" as *const u8
                    as *const core::ffi::c_char,
                593 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 81],
                    [core::ffi::c_char; 81],
                >(
                    *b"_Bool av1_resize_plane(const uint8_t *, int, int, int, uint8_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        if width2 > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"width2 > 0\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/resize.c\0" as *const u8
                    as *const core::ffi::c_char,
                594 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 81],
                    [core::ffi::c_char; 81],
                >(
                    *b"_Bool av1_resize_plane(const uint8_t *, int, int, int, uint8_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        if height2 > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"height2 > 0\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/resize.c\0" as *const u8
                    as *const core::ffi::c_char,
                595 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 81],
                    [core::ffi::c_char; 81],
                >(
                    *b"_Bool av1_resize_plane(const uint8_t *, int, int, int, uint8_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        i = 0 as core::ffi::c_int;
        while i < height {
            resize_multistep(
                input.offset((in_stride * i) as isize),
                width,
                intbuf.offset((width2 * i) as isize),
                width2,
                tmpbuf,
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int;
        while i < width2 {
            fill_col_to_arr(intbuf.offset(i as isize), width2, height, arrbuf);
            resize_multistep(arrbuf, height, arrbuf2, height2, tmpbuf);
            fill_arr_to_col(output.offset(i as isize), out_stride, height2, arrbuf2);
            i += 1;
        }
    }
    aom_free(intbuf as *mut core::ffi::c_void);
    aom_free(tmpbuf as *mut core::ffi::c_void);
    aom_free(arrbuf as *mut core::ffi::c_void);
    aom_free(arrbuf2 as *mut core::ffi::c_void);
    return mem_status;
}
unsafe extern "C-unwind" fn upscale_normative_rect(
    input: *const uint8_t,
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
    mut out_stride: core::ffi::c_int,
    mut x_step_qn: core::ffi::c_int,
    mut x0_qn: core::ffi::c_int,
    mut pad_left: core::ffi::c_int,
    mut pad_right: core::ffi::c_int,
) -> bool {
    if width > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"width > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            618 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 112],
                [core::ffi::c_char; 112],
            >(
                *b"_Bool upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if height > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"height > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            619 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 112],
                [core::ffi::c_char; 112],
            >(
                *b"_Bool upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if width2 > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"width2 > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            620 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 112],
                [core::ffi::c_char; 112],
            >(
                *b"_Bool upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if height2 > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"height2 > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            621 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 112],
                [core::ffi::c_char; 112],
            >(
                *b"_Bool upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if height2 == height {} else {
        __assert_fail(
            b"height2 == height\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            622 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 112],
                [core::ffi::c_char; 112],
            >(
                *b"_Bool upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let border_cols: core::ffi::c_int = UPSCALE_NORMATIVE_TAPS / 2 as core::ffi::c_int
        + 1 as core::ffi::c_int;
    let mut tmp_left: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp_right: *mut uint8_t = 0 as *mut uint8_t;
    let in_tl: *mut uint8_t = input.offset(-(border_cols as isize)) as *mut uint8_t;
    let in_tr: *mut uint8_t = input.offset(width as isize) as *mut uint8_t;
    if pad_left != 0 {
        tmp_left = aom_malloc(
            (::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul(border_cols as size_t)
                .wrapping_mul(height as size_t),
        ) as *mut uint8_t;
        if tmp_left.is_null() {
            return false_0 != 0;
        }
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < height {
            memcpy(
                tmp_left.offset((i * border_cols) as isize) as *mut core::ffi::c_void,
                in_tl.offset((i * in_stride) as isize) as *const core::ffi::c_void,
                border_cols as size_t,
            );
            memset(
                in_tl.offset((i * in_stride) as isize) as *mut core::ffi::c_void,
                *input.offset((i * in_stride) as isize) as core::ffi::c_int,
                border_cols as size_t,
            );
            i += 1;
        }
    }
    if pad_right != 0 {
        tmp_right = aom_malloc(
            (::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul(border_cols as size_t)
                .wrapping_mul(height as size_t),
        ) as *mut uint8_t;
        if tmp_right.is_null() {
            aom_free(tmp_left as *mut core::ffi::c_void);
            return false_0 != 0;
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < height {
            memcpy(
                tmp_right.offset((i_0 * border_cols) as isize) as *mut core::ffi::c_void,
                in_tr.offset((i_0 * in_stride) as isize) as *const core::ffi::c_void,
                border_cols as size_t,
            );
            memset(
                in_tr.offset((i_0 * in_stride) as isize) as *mut core::ffi::c_void,
                *input.offset((i_0 * in_stride + width - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                border_cols as size_t,
            );
            i_0 += 1;
        }
    }
    av1_convolve_horiz_rs_c(
        input.offset(-(1 as core::ffi::c_int as isize)),
        in_stride,
        output,
        out_stride,
        width2,
        height2,
        &*(*av1_resize_filter_normative.as_ptr().offset(0 as core::ffi::c_int as isize))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize),
        x0_qn,
        x_step_qn,
    );
    if pad_left != 0 {
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < height {
            memcpy(
                in_tl.offset((i_1 * in_stride) as isize) as *mut core::ffi::c_void,
                tmp_left.offset((i_1 * border_cols) as isize)
                    as *const core::ffi::c_void,
                border_cols as size_t,
            );
            i_1 += 1;
        }
        aom_free(tmp_left as *mut core::ffi::c_void);
    }
    if pad_right != 0 {
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < height {
            memcpy(
                in_tr.offset((i_2 * in_stride) as isize) as *mut core::ffi::c_void,
                tmp_right.offset((i_2 * border_cols) as isize)
                    as *const core::ffi::c_void,
                border_cols as size_t,
            );
            i_2 += 1;
        }
        aom_free(tmp_right as *mut core::ffi::c_void);
    }
    return true_0 != 0;
}
unsafe extern "C-unwind" fn highbd_interpolate_core(
    input: *const uint16_t,
    mut in_length: core::ffi::c_int,
    mut output: *mut uint16_t,
    mut out_length: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut interp_filters: *const int16_t,
    mut interp_taps: core::ffi::c_int,
) {
    let delta: int32_t = ((in_length as uint32_t) << RS_SCALE_SUBPEL_BITS)
        .wrapping_add((out_length / 2 as core::ffi::c_int) as uint32_t)
        .wrapping_div(out_length as uint32_t) as int32_t;
    let offset: int32_t = if in_length > out_length {
        ((((in_length - out_length) as int32_t)
            << RS_SCALE_SUBPEL_BITS - 1 as core::ffi::c_int)
            + out_length as int32_t / 2 as int32_t) / out_length as int32_t
    } else {
        -((((out_length - in_length) as int32_t)
            << RS_SCALE_SUBPEL_BITS - 1 as core::ffi::c_int)
            + out_length as int32_t / 2 as int32_t) / out_length as int32_t
    };
    let mut optr: *mut uint16_t = output;
    let mut x: core::ffi::c_int = 0;
    let mut x1: core::ffi::c_int = 0;
    let mut x2: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut int_pel: core::ffi::c_int = 0;
    let mut sub_pel: core::ffi::c_int = 0;
    let mut y: int32_t = 0;
    x = 0 as core::ffi::c_int;
    y = offset + RS_SCALE_EXTRA_OFF as int32_t;
    while y >> RS_SCALE_SUBPEL_BITS
        < interp_taps as int32_t / 2 as int32_t - 1 as int32_t
    {
        x += 1;
        y += delta;
    }
    x1 = x;
    x = out_length - 1 as core::ffi::c_int;
    y = delta * x as int32_t + offset + RS_SCALE_EXTRA_OFF as int32_t;
    while (y >> RS_SCALE_SUBPEL_BITS) + (interp_taps / 2 as core::ffi::c_int) as int32_t
        >= in_length as int32_t
    {
        x -= 1;
        y -= delta;
    }
    x2 = x;
    if x1 > x2 {
        x = 0 as core::ffi::c_int;
        y = offset + RS_SCALE_EXTRA_OFF as int32_t;
        while x < out_length {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                let pk: core::ffi::c_int = int_pel - interp_taps / 2 as core::ffi::c_int
                    + 1 as core::ffi::c_int + k;
                sum
                    += *filter.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                clamp(
                                    pk,
                                    0 as core::ffi::c_int,
                                    in_length - 1 as core::ffi::c_int,
                                ) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh16 = optr;
            optr = optr.offset(1);
            *fresh16 = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x += 1;
            y += delta;
        }
    } else {
        x = 0 as core::ffi::c_int;
        y = offset + RS_SCALE_EXTRA_OFF as int32_t;
        while x < x1 {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter_0: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                sum
                    += *filter_0.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                (if int_pel - interp_taps / 2 as core::ffi::c_int
                                    + 1 as core::ffi::c_int + k > 0 as core::ffi::c_int
                                {
                                    int_pel - interp_taps / 2 as core::ffi::c_int
                                        + 1 as core::ffi::c_int + k
                                } else {
                                    0 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh17 = optr;
            optr = optr.offset(1);
            *fresh17 = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x += 1;
            y += delta;
        }
        while x <= x2 {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter_1: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                sum
                    += *filter_1.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                (int_pel - interp_taps / 2 as core::ffi::c_int
                                    + 1 as core::ffi::c_int + k) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh18 = optr;
            optr = optr.offset(1);
            *fresh18 = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x += 1;
            y += delta;
        }
        while x < out_length {
            int_pel = (y >> RS_SCALE_SUBPEL_BITS) as core::ffi::c_int;
            sub_pel = (y >> RS_SCALE_EXTRA_BITS & RS_SUBPEL_MASK as int32_t)
                as core::ffi::c_int;
            let mut filter_2: *const int16_t = &*interp_filters
                .offset((sub_pel * interp_taps) as isize) as *const int16_t;
            sum = 0 as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < interp_taps {
                sum
                    += *filter_2.offset(k as isize) as core::ffi::c_int
                        * *input
                            .offset(
                                (if int_pel - interp_taps / 2 as core::ffi::c_int
                                    + 1 as core::ffi::c_int + k
                                    < in_length - 1 as core::ffi::c_int
                                {
                                    int_pel - interp_taps / 2 as core::ffi::c_int
                                        + 1 as core::ffi::c_int + k
                                } else {
                                    in_length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            let fresh19 = optr;
            optr = optr.offset(1);
            *fresh19 = clip_pixel_highbd(
                sum
                    + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int,
                bd,
            );
            x += 1;
            y += delta;
        }
    };
}
unsafe extern "C-unwind" fn highbd_interpolate(
    input: *const uint16_t,
    mut in_length: core::ffi::c_int,
    mut output: *mut uint16_t,
    mut out_length: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut interp_filters: *const InterpKernel = choose_interp_filter(
        in_length,
        out_length,
    );
    highbd_interpolate_core(
        input,
        in_length,
        output,
        out_length,
        bd,
        &*(*interp_filters.offset(0 as core::ffi::c_int as isize))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize),
        SUBPEL_TAPS,
    );
}
unsafe extern "C-unwind" fn highbd_down2_symeven(
    input: *const uint16_t,
    mut length: core::ffi::c_int,
    mut output: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    static mut filter: *const int16_t = unsafe {
        core::ptr::addr_of!(av1_down2_symeven_half_filter).cast::<int16_t>()
    };
    let filter_len_half: core::ffi::c_int = (::core::mem::size_of::<[int16_t; 4]>()
        as usize)
        .wrapping_div(2 as usize) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut optr: *mut uint16_t = output;
    let mut l1: core::ffi::c_int = filter_len_half;
    let mut l2: core::ffi::c_int = length - filter_len_half;
    l1 += l1 & 1 as core::ffi::c_int;
    l2 += l2 & 1 as core::ffi::c_int;
    if l1 > l2 {
        i = 0 as core::ffi::c_int;
        while i < length {
            let mut sum: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum
                    += (*input
                        .offset(
                            (if 0 as core::ffi::c_int > i - j {
                                0 as core::ffi::c_int
                            } else {
                                i - j
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + 1 as core::ffi::c_int + j
                                    < length - 1 as core::ffi::c_int
                                {
                                    i + 1 as core::ffi::c_int + j
                                } else {
                                    length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum >>= FILTER_BITS;
            let fresh20 = optr;
            optr = optr.offset(1);
            *fresh20 = clip_pixel_highbd(sum, bd);
            i += 2 as core::ffi::c_int;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < l1 {
            let mut sum_0: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum_0
                    += (*input
                        .offset(
                            (if 0 as core::ffi::c_int > i - j {
                                0 as core::ffi::c_int
                            } else {
                                i - j
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input.offset((i + 1 as core::ffi::c_int + j) as isize)
                            as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_0 >>= FILTER_BITS;
            let fresh21 = optr;
            optr = optr.offset(1);
            *fresh21 = clip_pixel_highbd(sum_0, bd);
            i += 2 as core::ffi::c_int;
        }
        while i < l2 {
            let mut sum_1: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum_1
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input.offset((i + 1 as core::ffi::c_int + j) as isize)
                            as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_1 >>= FILTER_BITS;
            let fresh22 = optr;
            optr = optr.offset(1);
            *fresh22 = clip_pixel_highbd(sum_1, bd);
            i += 2 as core::ffi::c_int;
        }
        while i < length {
            let mut sum_2: core::ffi::c_int = (1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int;
            j = 0 as core::ffi::c_int;
            while j < filter_len_half {
                sum_2
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + 1 as core::ffi::c_int + j
                                    < length - 1 as core::ffi::c_int
                                {
                                    i + 1 as core::ffi::c_int + j
                                } else {
                                    length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_2 >>= FILTER_BITS;
            let fresh23 = optr;
            optr = optr.offset(1);
            *fresh23 = clip_pixel_highbd(sum_2, bd);
            i += 2 as core::ffi::c_int;
        }
    };
}
unsafe extern "C-unwind" fn highbd_down2_symodd(
    input: *const uint16_t,
    mut length: core::ffi::c_int,
    mut output: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    static mut filter: *const int16_t = unsafe { core::ptr::addr_of!(av1_down2_symodd_half_filter).cast::<int16_t>() };
    let filter_len_half: core::ffi::c_int = (::core::mem::size_of::<[int16_t; 4]>()
        as usize)
        .wrapping_div(2 as usize) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut optr: *mut uint16_t = output;
    let mut l1: core::ffi::c_int = filter_len_half - 1 as core::ffi::c_int;
    let mut l2: core::ffi::c_int = length - filter_len_half + 1 as core::ffi::c_int;
    l1 += l1 & 1 as core::ffi::c_int;
    l2 += l2 & 1 as core::ffi::c_int;
    if l1 > l2 {
        i = 0 as core::ffi::c_int;
        while i < length {
            let mut sum: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum
                    += (*input
                        .offset(
                            (if i - j > 0 as core::ffi::c_int {
                                i - j
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + j < length - 1 as core::ffi::c_int {
                                    i + j
                                } else {
                                    length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum >>= FILTER_BITS;
            let fresh24 = optr;
            optr = optr.offset(1);
            *fresh24 = clip_pixel_highbd(sum, bd);
            i += 2 as core::ffi::c_int;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < l1 {
            let mut sum_0: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum_0
                    += (*input
                        .offset(
                            (if i - j > 0 as core::ffi::c_int {
                                i - j
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ) as core::ffi::c_int
                        + *input.offset((i + j) as isize) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_0 >>= FILTER_BITS;
            let fresh25 = optr;
            optr = optr.offset(1);
            *fresh25 = clip_pixel_highbd(sum_0, bd);
            i += 2 as core::ffi::c_int;
        }
        while i < l2 {
            let mut sum_1: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum_1
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input.offset((i + j) as isize) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_1 >>= FILTER_BITS;
            let fresh26 = optr;
            optr = optr.offset(1);
            *fresh26 = clip_pixel_highbd(sum_1, bd);
            i += 2 as core::ffi::c_int;
        }
        while i < length {
            let mut sum_2: core::ffi::c_int = ((1 as core::ffi::c_int)
                << FILTER_BITS - 1 as core::ffi::c_int)
                + *input.offset(i as isize) as core::ffi::c_int
                    * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
            j = 1 as core::ffi::c_int;
            while j < filter_len_half {
                sum_2
                    += (*input.offset((i - j) as isize) as core::ffi::c_int
                        + *input
                            .offset(
                                (if i + j < length - 1 as core::ffi::c_int {
                                    i + j
                                } else {
                                    length - 1 as core::ffi::c_int
                                }) as isize,
                            ) as core::ffi::c_int)
                        * *filter.offset(j as isize) as core::ffi::c_int;
                j += 1;
            }
            sum_2 >>= FILTER_BITS;
            let fresh27 = optr;
            optr = optr.offset(1);
            *fresh27 = clip_pixel_highbd(sum_2, bd);
            i += 2 as core::ffi::c_int;
        }
    };
}
unsafe extern "C-unwind" fn highbd_resize_multistep(
    input: *const uint16_t,
    mut length: core::ffi::c_int,
    mut output: *mut uint16_t,
    mut olength: core::ffi::c_int,
    mut otmp: *mut uint16_t,
    mut bd: core::ffi::c_int,
) {
    if length == olength {
        memcpy(
            output as *mut core::ffi::c_void,
            input as *const core::ffi::c_void,
            (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(length as size_t),
        );
        return;
    }
    let steps: core::ffi::c_int = get_down2_steps(length, olength) as core::ffi::c_int;
    if steps > 0 as core::ffi::c_int {
        let mut out: *mut uint16_t = 0 as *mut uint16_t;
        let mut filteredlength: core::ffi::c_int = length;
        if !otmp.is_null() {} else {
            __assert_fail(
                b"otmp != NULL\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/resize.c\0" as *const u8
                    as *const core::ffi::c_char,
                892 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 91],
                    [core::ffi::c_char; 91],
                >(
                    *b"void highbd_resize_multistep(const uint16_t *const, int, uint16_t *, int, uint16_t *, int)\0",
                ))
                    .as_ptr(),
            );
        };
        let mut otmp2: *mut uint16_t = otmp
            .offset(get_down2_length(length, 1 as core::ffi::c_int) as isize);
        let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
        while s < steps {
            let proj_filteredlength: core::ffi::c_int = get_down2_length(
                filteredlength,
                1 as core::ffi::c_int,
            ) as core::ffi::c_int;
            let in_0: *const uint16_t = if s == 0 as core::ffi::c_int {
                input
            } else {
                out as *const uint16_t
            };
            if s == steps - 1 as core::ffi::c_int && proj_filteredlength == olength {
                out = output;
            } else {
                out = if s & 1 as core::ffi::c_int != 0 { otmp2 } else { otmp };
            }
            if filteredlength & 1 as core::ffi::c_int != 0 {
                highbd_down2_symodd(in_0, filteredlength, out, bd);
            } else {
                highbd_down2_symeven(in_0, filteredlength, out, bd);
            }
            filteredlength = proj_filteredlength;
            s += 1;
        }
        if filteredlength != olength {
            highbd_interpolate(out, filteredlength, output, olength, bd);
        }
    } else {
        highbd_interpolate(input, length, output, olength, bd);
    };
}
unsafe extern "C-unwind" fn highbd_fill_col_to_arr(
    mut img: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut len: core::ffi::c_int,
    mut arr: *mut uint16_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut iptr: *mut uint16_t = img;
    let mut aptr: *mut uint16_t = arr;
    i = 0 as core::ffi::c_int;
    while i < len {
        let fresh28 = aptr;
        aptr = aptr.offset(1);
        *fresh28 = *iptr;
        i += 1;
        iptr = iptr.offset(stride as isize);
    }
}
unsafe extern "C-unwind" fn highbd_fill_arr_to_col(
    mut img: *mut uint16_t,
    mut stride: core::ffi::c_int,
    mut len: core::ffi::c_int,
    mut arr: *mut uint16_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut iptr: *mut uint16_t = img;
    let mut aptr: *mut uint16_t = arr;
    i = 0 as core::ffi::c_int;
    while i < len {
        let fresh15 = aptr;
        aptr = aptr.offset(1);
        *iptr = *fresh15;
        i += 1;
        iptr = iptr.offset(stride as isize);
    }
}
unsafe extern "C-unwind" fn highbd_resize_plane(
    mut input: *const uint8_t,
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
    mut out_stride: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut intbuf: *mut uint16_t = aom_malloc(
        (::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_mul(width2 as size_t)
            .wrapping_mul(height as size_t),
    ) as *mut uint16_t;
    let mut tmpbuf: *mut uint16_t = aom_malloc(
        (::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_mul((if width > height { width } else { height }) as size_t),
    ) as *mut uint16_t;
    let mut arrbuf: *mut uint16_t = aom_malloc(
        (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(height as size_t),
    ) as *mut uint16_t;
    let mut arrbuf2: *mut uint16_t = aom_malloc(
        (::core::mem::size_of::<uint16_t>() as size_t).wrapping_mul(height2 as size_t),
    ) as *mut uint16_t;
    if !(intbuf.is_null() || tmpbuf.is_null() || arrbuf.is_null() || arrbuf2.is_null()) {
        i = 0 as core::ffi::c_int;
        while i < height {
            highbd_resize_multistep(
                ((input.offset((in_stride * i) as isize) as uintptr_t)
                    << 1 as core::ffi::c_int) as *mut uint16_t,
                width,
                intbuf.offset((width2 * i) as isize),
                width2,
                tmpbuf,
                bd,
            );
            i += 1;
        }
        i = 0 as core::ffi::c_int;
        while i < width2 {
            highbd_fill_col_to_arr(intbuf.offset(i as isize), width2, height, arrbuf);
            highbd_resize_multistep(arrbuf, height, arrbuf2, height2, tmpbuf, bd);
            highbd_fill_arr_to_col(
                ((output.offset(i as isize) as uintptr_t) << 1 as core::ffi::c_int)
                    as *mut uint16_t,
                out_stride,
                height2,
                arrbuf2,
            );
            i += 1;
        }
    }
    aom_free(intbuf as *mut core::ffi::c_void);
    aom_free(tmpbuf as *mut core::ffi::c_void);
    aom_free(arrbuf as *mut core::ffi::c_void);
    aom_free(arrbuf2 as *mut core::ffi::c_void);
}
unsafe extern "C-unwind" fn highbd_upscale_normative_rect(
    input: *const uint8_t,
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut output: *mut uint8_t,
    mut height2: core::ffi::c_int,
    mut width2: core::ffi::c_int,
    mut out_stride: core::ffi::c_int,
    mut x_step_qn: core::ffi::c_int,
    mut x0_qn: core::ffi::c_int,
    mut pad_left: core::ffi::c_int,
    mut pad_right: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> bool {
    if width > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"width > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            970 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 124],
                [core::ffi::c_char; 124],
            >(
                *b"_Bool highbd_upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if height > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"height > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            971 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 124],
                [core::ffi::c_char; 124],
            >(
                *b"_Bool highbd_upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if width2 > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"width2 > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            972 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 124],
                [core::ffi::c_char; 124],
            >(
                *b"_Bool highbd_upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if height2 > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"height2 > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            973 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 124],
                [core::ffi::c_char; 124],
            >(
                *b"_Bool highbd_upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if height2 == height {} else {
        __assert_fail(
            b"height2 == height\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            974 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 124],
                [core::ffi::c_char; 124],
            >(
                *b"_Bool highbd_upscale_normative_rect(const uint8_t *const, int, int, int, uint8_t *, int, int, int, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let border_cols: core::ffi::c_int = UPSCALE_NORMATIVE_TAPS / 2 as core::ffi::c_int
        + 1 as core::ffi::c_int;
    let border_size: core::ffi::c_int = (border_cols as usize)
        .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as core::ffi::c_int;
    let mut tmp_left: *mut uint16_t = 0 as *mut uint16_t;
    let mut tmp_right: *mut uint16_t = 0 as *mut uint16_t;
    let input16: *mut uint16_t = ((input as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let in_tl: *mut uint16_t = input16.offset(-(border_cols as isize));
    let in_tr: *mut uint16_t = input16.offset(width as isize);
    if pad_left != 0 {
        tmp_left = aom_malloc(
            (::core::mem::size_of::<uint16_t>() as size_t)
                .wrapping_mul(border_cols as size_t)
                .wrapping_mul(height as size_t),
        ) as *mut uint16_t;
        if tmp_left.is_null() {
            return false_0 != 0;
        }
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < height {
            memcpy(
                tmp_left.offset((i * border_cols) as isize) as *mut core::ffi::c_void,
                in_tl.offset((i * in_stride) as isize) as *const core::ffi::c_void,
                border_size as size_t,
            );
            aom_memset16(
                in_tl.offset((i * in_stride) as isize) as *mut core::ffi::c_void,
                *input16.offset((i * in_stride) as isize) as core::ffi::c_int,
                border_cols as size_t,
            );
            i += 1;
        }
    }
    if pad_right != 0 {
        tmp_right = aom_malloc(
            (::core::mem::size_of::<uint16_t>() as size_t)
                .wrapping_mul(border_cols as size_t)
                .wrapping_mul(height as size_t),
        ) as *mut uint16_t;
        if tmp_right.is_null() {
            aom_free(tmp_left as *mut core::ffi::c_void);
            return false_0 != 0;
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < height {
            memcpy(
                tmp_right.offset((i_0 * border_cols) as isize) as *mut core::ffi::c_void,
                in_tr.offset((i_0 * in_stride) as isize) as *const core::ffi::c_void,
                border_size as size_t,
            );
            aom_memset16(
                in_tr.offset((i_0 * in_stride) as isize) as *mut core::ffi::c_void,
                *input16
                    .offset((i_0 * in_stride + width - 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
                border_cols as size_t,
            );
            i_0 += 1;
        }
    }
    av1_highbd_convolve_horiz_rs_c(
        ((input.offset(-(1 as core::ffi::c_int as isize)) as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t,
        in_stride,
        ((output as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
        out_stride,
        width2,
        height2,
        &*(*av1_resize_filter_normative.as_ptr().offset(0 as core::ffi::c_int as isize))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize),
        x0_qn,
        x_step_qn,
        bd,
    );
    if pad_left != 0 {
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < height {
            memcpy(
                in_tl.offset((i_1 * in_stride) as isize) as *mut core::ffi::c_void,
                tmp_left.offset((i_1 * border_cols) as isize)
                    as *const core::ffi::c_void,
                border_size as size_t,
            );
            i_1 += 1;
        }
        aom_free(tmp_left as *mut core::ffi::c_void);
    }
    if pad_right != 0 {
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < height {
            memcpy(
                in_tr.offset((i_2 * in_stride) as isize) as *mut core::ffi::c_void,
                tmp_right.offset((i_2 * border_cols) as isize)
                    as *const core::ffi::c_void,
                border_size as size_t,
            );
            i_2 += 1;
        }
        aom_free(tmp_right as *mut core::ffi::c_void);
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_resize_and_extend_frame_c(
    mut src: *const YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
    filter: InterpFilter,
    phase_scaler: core::ffi::c_int,
    num_planes: core::ffi::c_int,
) {
    if filter as core::ffi::c_int == BILINEAR as core::ffi::c_int
        || filter as core::ffi::c_int == EIGHTTAP_SMOOTH as core::ffi::c_int
        || filter as core::ffi::c_int == EIGHTTAP_REGULAR as core::ffi::c_int
    {} else {
        __assert_fail(
            b"filter == BILINEAR || filter == EIGHTTAP_SMOOTH || filter == EIGHTTAP_REGULAR\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            1040 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 127],
                [core::ffi::c_char; 127],
            >(
                *b"void av1_resize_and_extend_frame_c(const YV12_BUFFER_CONFIG *, YV12_BUFFER_CONFIG *, const InterpFilter, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let kernel: *const InterpKernel = av1_interp_filter_params_list[filter as usize]
        .filter_ptr as *const InterpKernel;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i
        < (if num_planes < 3 as core::ffi::c_int {
            num_planes
        } else {
            3 as core::ffi::c_int
        })
    {
        let is_uv: core::ffi::c_int = (i > 0 as core::ffi::c_int) as core::ffi::c_int;
        let src_w: core::ffi::c_int = (*src)
            .c2rust_unnamed_1
            .crop_widths[is_uv as usize];
        let src_h: core::ffi::c_int = (*src)
            .c2rust_unnamed_2
            .crop_heights[is_uv as usize];
        let mut src_buffer: *const uint8_t = (*src).c2rust_unnamed_4.buffers[i as usize];
        let src_stride: core::ffi::c_int = (*src)
            .c2rust_unnamed_3
            .strides[is_uv as usize];
        let dst_w: core::ffi::c_int = (*dst)
            .c2rust_unnamed_1
            .crop_widths[is_uv as usize];
        let dst_h: core::ffi::c_int = (*dst)
            .c2rust_unnamed_2
            .crop_heights[is_uv as usize];
        let mut dst_buffer: *mut uint8_t = (*dst).c2rust_unnamed_4.buffers[i as usize];
        let dst_stride: core::ffi::c_int = (*dst)
            .c2rust_unnamed_3
            .strides[is_uv as usize];
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < dst_h {
            let y_q4: core::ffi::c_int = if src_h == dst_h {
                0 as core::ffi::c_int
            } else {
                y * 16 as core::ffi::c_int * src_h / dst_h + phase_scaler
            };
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < dst_w {
                let x_q4: core::ffi::c_int = if src_w == dst_w {
                    0 as core::ffi::c_int
                } else {
                    x * 16 as core::ffi::c_int * src_w / dst_w + phase_scaler
                };
                let mut src_ptr: *const uint8_t = src_buffer
                    .offset((y * src_h / dst_h * src_stride) as isize)
                    .offset((x * src_w / dst_w) as isize);
                let mut dst_ptr: *mut uint8_t = dst_buffer
                    .offset((y * dst_stride) as isize)
                    .offset(x as isize);
                let work_w: core::ffi::c_int = if (16 as core::ffi::c_int) < dst_w - x {
                    16 as core::ffi::c_int
                } else {
                    dst_w - x
                };
                let work_h: core::ffi::c_int = if (16 as core::ffi::c_int) < dst_h - y {
                    16 as core::ffi::c_int
                } else {
                    dst_h - y
                };
                if work_w != 16 as core::ffi::c_int || work_h != 16 as core::ffi::c_int {
                    aom_scaled_2d_c(
                        src_ptr,
                        src_stride as ptrdiff_t,
                        dst_ptr,
                        dst_stride as ptrdiff_t,
                        kernel,
                        x_q4 & 0xf as core::ffi::c_int,
                        16 as core::ffi::c_int * src_w / dst_w,
                        y_q4 & 0xf as core::ffi::c_int,
                        16 as core::ffi::c_int * src_h / dst_h,
                        work_w,
                        work_h,
                    );
                } else {
                    aom_scaled_2d_c(
                        src_ptr,
                        src_stride as ptrdiff_t,
                        dst_ptr,
                        dst_stride as ptrdiff_t,
                        kernel,
                        x_q4 & 0xf as core::ffi::c_int,
                        16 as core::ffi::c_int * src_w / dst_w,
                        y_q4 & 0xf as core::ffi::c_int,
                        16 as core::ffi::c_int * src_h / dst_h,
                        16 as core::ffi::c_int,
                        16 as core::ffi::c_int,
                    );
                }
                x += 16 as core::ffi::c_int;
            }
            y += 16 as core::ffi::c_int;
        }
        i += 1;
    }
    aom_extend_frame_borders_c(dst as *mut yv12_buffer_config, num_planes);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_resize_and_extend_frame_nonnormative(
    mut src: *const YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
    mut bd: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
) -> bool {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i
        < (if num_planes < 3 as core::ffi::c_int {
            num_planes
        } else {
            3 as core::ffi::c_int
        })
    {
        let is_uv: core::ffi::c_int = (i > 0 as core::ffi::c_int) as core::ffi::c_int;
        if (*src).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
            highbd_resize_plane(
                (*src).c2rust_unnamed_4.buffers[i as usize],
                (*src).c2rust_unnamed_2.crop_heights[is_uv as usize],
                (*src).c2rust_unnamed_1.crop_widths[is_uv as usize],
                (*src).c2rust_unnamed_3.strides[is_uv as usize],
                (*dst).c2rust_unnamed_4.buffers[i as usize],
                (*dst).c2rust_unnamed_2.crop_heights[is_uv as usize],
                (*dst).c2rust_unnamed_1.crop_widths[is_uv as usize],
                (*dst).c2rust_unnamed_3.strides[is_uv as usize],
                bd,
            );
        } else if !av1_resize_plane(
            (*src).c2rust_unnamed_4.buffers[i as usize],
            (*src).c2rust_unnamed_2.crop_heights[is_uv as usize],
            (*src).c2rust_unnamed_1.crop_widths[is_uv as usize],
            (*src).c2rust_unnamed_3.strides[is_uv as usize],
            (*dst).c2rust_unnamed_4.buffers[i as usize],
            (*dst).c2rust_unnamed_2.crop_heights[is_uv as usize],
            (*dst).c2rust_unnamed_1.crop_widths[is_uv as usize],
            (*dst).c2rust_unnamed_3.strides[is_uv as usize],
        ) {
            return false_0 != 0
        }
        i += 1;
    }
    aom_extend_frame_borders_c(dst as *mut yv12_buffer_config, num_planes);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_upscale_normative_rows(
    mut cm: *const AV1_COMMON,
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut rows: core::ffi::c_int,
) {
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let ss_x: core::ffi::c_int = (is_uv != 0 && (*(*cm).seq_params).subsampling_x != 0)
        as core::ffi::c_int;
    let downscaled_plane_width: core::ffi::c_int = (*cm).width
        + ((1 as core::ffi::c_int) << ss_x >> 1 as core::ffi::c_int) >> ss_x;
    let upscaled_plane_width: core::ffi::c_int = (*cm).superres_upscaled_width
        + ((1 as core::ffi::c_int) << ss_x >> 1 as core::ffi::c_int) >> ss_x;
    let superres_denom: core::ffi::c_int = (*cm).superres_scale_denominator
        as core::ffi::c_int;
    let mut tile_col: TileInfo = TileInfo {
        mi_row_start: 0,
        mi_row_end: 0,
        mi_col_start: 0,
        mi_col_end: 0,
        tile_row: 0,
        tile_col: 0,
    };
    let x_step_qn: int32_t = av1_get_upscale_convolve_step(
        downscaled_plane_width,
        upscaled_plane_width,
    ) as int32_t;
    let mut x0_qn: int32_t = get_upscale_convolve_x0(
        downscaled_plane_width,
        upscaled_plane_width,
        x_step_qn,
    );
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < (*cm).tiles.cols {
        av1_tile_set_col(&raw mut tile_col, cm as *const AV1Common, j);
        let downscaled_x0: core::ffi::c_int = tile_col.mi_col_start
            << MI_SIZE_LOG2 - ss_x;
        let downscaled_x1: core::ffi::c_int = tile_col.mi_col_end << MI_SIZE_LOG2 - ss_x;
        let src_width: core::ffi::c_int = downscaled_x1 - downscaled_x0;
        let upscaled_x0: core::ffi::c_int = downscaled_x0 * superres_denom
            / SCALE_NUMERATOR;
        let mut upscaled_x1: core::ffi::c_int = 0;
        if j == (*cm).tiles.cols - 1 as core::ffi::c_int {
            upscaled_x1 = upscaled_plane_width;
        } else {
            upscaled_x1 = downscaled_x1 * superres_denom / SCALE_NUMERATOR;
        }
        let src_ptr: *const uint8_t = src.offset(downscaled_x0 as isize);
        let dst_ptr: *mut uint8_t = dst.offset(upscaled_x0 as isize);
        let dst_width: core::ffi::c_int = upscaled_x1 - upscaled_x0;
        let pad_left: core::ffi::c_int = (j == 0 as core::ffi::c_int)
            as core::ffi::c_int;
        let pad_right: core::ffi::c_int = (j == (*cm).tiles.cols - 1 as core::ffi::c_int)
            as core::ffi::c_int;
        let mut success: bool = false;
        if (*(*cm).seq_params).use_highbitdepth != 0 {
            success = highbd_upscale_normative_rect(
                src_ptr,
                rows,
                src_width,
                src_stride,
                dst_ptr,
                rows,
                dst_width,
                dst_stride,
                x_step_qn as core::ffi::c_int,
                x0_qn as core::ffi::c_int,
                pad_left,
                pad_right,
                (*(*cm).seq_params).bit_depth as core::ffi::c_int,
            );
        } else {
            success = upscale_normative_rect(
                src_ptr,
                rows,
                src_width,
                src_stride,
                dst_ptr,
                rows,
                dst_width,
                dst_stride,
                x_step_qn as core::ffi::c_int,
                x0_qn as core::ffi::c_int,
                pad_left,
                pad_right,
            );
        }
        if !success {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Error upscaling frame\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        x0_qn = (x0_qn as core::ffi::c_int
            + (dst_width as int32_t * x_step_qn
                - ((src_width as int32_t) << RS_SCALE_SUBPEL_BITS)) as core::ffi::c_int)
            as int32_t;
        j += 1;
    }
}
unsafe extern "C-unwind" fn upscale_normative_and_extend_frame(
    mut cm: *const AV1_COMMON,
    mut src: *const YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_planes {
        let is_uv: core::ffi::c_int = (i > 0 as core::ffi::c_int) as core::ffi::c_int;
        av1_upscale_normative_rows(
            cm,
            (*src).c2rust_unnamed_4.buffers[i as usize],
            (*src).c2rust_unnamed_3.strides[is_uv as usize],
            (*dst).c2rust_unnamed_4.buffers[i as usize],
            (*dst).c2rust_unnamed_3.strides[is_uv as usize],
            i,
            (*src).c2rust_unnamed_2.crop_heights[is_uv as usize],
        );
        i += 1;
    }
    aom_extend_frame_borders_c(dst as *mut yv12_buffer_config, num_planes);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_realloc_and_scale_if_required(
    mut cm: *mut AV1_COMMON,
    mut unscaled: *mut YV12_BUFFER_CONFIG,
    mut scaled: *mut YV12_BUFFER_CONFIG,
    filter: InterpFilter,
    phase: core::ffi::c_int,
    use_optimized_scaler: bool,
    for_psnr: bool,
    border_in_pixels: core::ffi::c_int,
    alloc_pyramid: bool,
) -> *mut YV12_BUFFER_CONFIG {
    let scaled_width: core::ffi::c_int = if for_psnr as core::ffi::c_int != 0 {
        (*cm).superres_upscaled_width
    } else {
        (*cm).width
    };
    let scaled_height: core::ffi::c_int = if for_psnr as core::ffi::c_int != 0 {
        (*cm).superres_upscaled_height
    } else {
        (*cm).height
    };
    let scaling_required: bool = scaled_width
        != (*unscaled).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        || scaled_height != (*unscaled).c2rust_unnamed_2.c2rust_unnamed.y_crop_height;
    if scaling_required {
        let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
        let mut seq_params: *const SequenceHeader = (*cm).seq_params;
        if aom_realloc_frame_buffer(
            scaled,
            scaled_width,
            scaled_height,
            (*seq_params).subsampling_x,
            (*seq_params).subsampling_y,
            (*seq_params).use_highbitdepth as core::ffi::c_int,
            border_in_pixels,
            (*cm).features.byte_alignment,
            0 as *mut aom_codec_frame_buffer_t,
            None,
            NULL,
            alloc_pyramid,
            0 as core::ffi::c_int,
        ) != 0
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate scaled buffer\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut has_optimized_scaler: bool = av1_has_optimized_scaler(
            (*unscaled).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
            (*unscaled).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
            scaled_width,
            scaled_height,
        );
        if num_planes > 1 as core::ffi::c_int {
            has_optimized_scaler = has_optimized_scaler as core::ffi::c_int != 0
                && av1_has_optimized_scaler(
                    (*unscaled).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
                    (*unscaled).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
                    (*scaled).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
                    (*scaled).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
                ) as core::ffi::c_int != 0;
        }
        if use_optimized_scaler as core::ffi::c_int != 0
            && has_optimized_scaler as core::ffi::c_int != 0
            && (*(*cm).seq_params).bit_depth as core::ffi::c_uint
                == AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint
        {
            av1_resize_and_extend_frame_c(unscaled, scaled, filter, phase, num_planes);
        } else if !av1_resize_and_extend_frame_nonnormative(
            unscaled,
            scaled,
            (*(*cm).seq_params).bit_depth as core::ffi::c_int,
            num_planes,
        ) {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate buffers during resize\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if !((*unscaled).metadata).is_null()
            && aom_copy_metadata_to_frame_buffer(scaled, (*unscaled).metadata) != 0
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to copy source metadata to scaled frame\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        return scaled;
    }
    return unscaled;
}
unsafe extern "C-unwind" fn calculate_scaled_size_helper(
    mut dim: *mut core::ffi::c_int,
    mut denom: core::ffi::c_int,
) {
    if denom != SCALE_NUMERATOR {
        let min_dim: core::ffi::c_int = if (16 as core::ffi::c_int) < *dim {
            16 as core::ffi::c_int
        } else {
            *dim
        };
        *dim = (*dim * SCALE_NUMERATOR + denom / 2 as core::ffi::c_int) / denom;
        *dim = if *dim > min_dim { *dim } else { min_dim };
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calculate_scaled_size(
    mut width: *mut core::ffi::c_int,
    mut height: *mut core::ffi::c_int,
    mut resize_denom: core::ffi::c_int,
) {
    calculate_scaled_size_helper(width, resize_denom);
    calculate_scaled_size_helper(height, resize_denom);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calculate_scaled_superres_size(
    mut width: *mut core::ffi::c_int,
    mut height: *mut core::ffi::c_int,
    mut superres_denom: core::ffi::c_int,
) {
    calculate_scaled_size_helper(width, superres_denom);
}
unsafe extern "C-unwind" fn copy_buffer_config(
    src: *const YV12_BUFFER_CONFIG,
    dst: *mut YV12_BUFFER_CONFIG,
) {
    (*dst).bit_depth = (*src).bit_depth;
    (*dst).color_primaries = (*src).color_primaries;
    (*dst).transfer_characteristics = (*src).transfer_characteristics;
    (*dst).matrix_coefficients = (*src).matrix_coefficients;
    (*dst).monochrome = (*src).monochrome;
    (*dst).chroma_sample_position = (*src).chroma_sample_position;
    (*dst).color_range = (*src).color_range;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_superres_upscale(
    mut cm: *mut AV1_COMMON,
    pool: *mut BufferPool,
    mut alloc_pyramid: bool,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if av1_superres_scaled(cm) == 0 {
        return;
    }
    let seq_params: *const SequenceHeader = (*cm).seq_params;
    let byte_alignment: core::ffi::c_int = (*cm).features.byte_alignment;
    let mut copy_buffer: YV12_BUFFER_CONFIG = yv12_buffer_config {
        c2rust_unnamed: C2RustUnnamed_9 {
            c2rust_unnamed: C2RustUnnamed_10 {
                y_width: 0,
                uv_width: 0,
            },
        },
        c2rust_unnamed_0: C2RustUnnamed_7 {
            c2rust_unnamed: C2RustUnnamed_8 {
                y_height: 0,
                uv_height: 0,
            },
        },
        c2rust_unnamed_1: C2RustUnnamed_5 {
            c2rust_unnamed: C2RustUnnamed_6 {
                y_crop_width: 0,
                uv_crop_width: 0,
            },
        },
        c2rust_unnamed_2: C2RustUnnamed_3 {
            c2rust_unnamed: C2RustUnnamed_4 {
                y_crop_height: 0,
                uv_crop_height: 0,
            },
        },
        c2rust_unnamed_3: C2RustUnnamed_1 {
            c2rust_unnamed: C2RustUnnamed_2 {
                y_stride: 0,
                uv_stride: 0,
            },
        },
        c2rust_unnamed_4: C2RustUnnamed {
            c2rust_unnamed: C2RustUnnamed_0 {
                y_buffer: 0 as *mut uint8_t,
                u_buffer: 0 as *mut uint8_t,
                v_buffer: 0 as *mut uint8_t,
            },
        },
        use_external_reference_buffers: 0,
        store_buf_adr: [0 as *mut uint8_t; 3],
        y_pyramid: 0 as *mut image_pyramid,
        corners: 0 as *mut corner_list,
        buffer_alloc: 0 as *mut uint8_t,
        buffer_alloc_sz: 0,
        border: 0,
        frame_size: 0,
        subsampling_x: 0,
        subsampling_y: 0,
        bit_depth: 0,
        color_primaries: AOM_CICP_CP_RESERVED_0,
        transfer_characteristics: AOM_CICP_TC_RESERVED_0,
        matrix_coefficients: AOM_CICP_MC_IDENTITY,
        monochrome: 0,
        chroma_sample_position: AOM_CSP_UNKNOWN,
        color_range: AOM_CR_STUDIO_RANGE,
        render_width: 0,
        render_height: 0,
        corrupted: 0,
        flags: 0,
        metadata: 0 as *mut aom_metadata_array_t,
    };
    memset(
        &raw mut copy_buffer as *mut YV12_BUFFER_CONFIG as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<YV12_BUFFER_CONFIG>() as size_t,
    );
    let frame_to_show: *mut YV12_BUFFER_CONFIG = &raw mut (*(*cm).cur_frame).buf;
    let aligned_width: core::ffi::c_int = (*cm).width
        + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int);
    if aom_alloc_frame_buffer(
        &raw mut copy_buffer,
        aligned_width,
        (*cm).height,
        (*seq_params).subsampling_x,
        (*seq_params).subsampling_y,
        (*seq_params).use_highbitdepth as core::ffi::c_int,
        AOM_BORDER_IN_PIXELS,
        byte_alignment,
        false_0 != 0,
        0 as core::ffi::c_int,
    ) != 0
    {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate copy buffer for superres upscaling\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    aom_yv12_copy_frame_c(frame_to_show, &raw mut copy_buffer, num_planes);
    if copy_buffer.c2rust_unnamed_1.c2rust_unnamed.y_crop_width == aligned_width
    {} else {
        __assert_fail(
            b"copy_buffer.y_crop_width == aligned_width\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            1341 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"void av1_superres_upscale(AV1_COMMON *, BufferPool *const, _Bool)\0"))
                .as_ptr(),
        );
    };
    if copy_buffer.c2rust_unnamed_2.c2rust_unnamed.y_crop_height == (*cm).height
    {} else {
        __assert_fail(
            b"copy_buffer.y_crop_height == cm->height\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            1342 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"void av1_superres_upscale(AV1_COMMON *, BufferPool *const, _Bool)\0"))
                .as_ptr(),
        );
    };
    if !pool.is_null() {
        let mut fb: *mut aom_codec_frame_buffer_t = &raw mut (*(*cm).cur_frame)
            .raw_frame_buffer;
        let mut release_fb_cb: aom_release_frame_buffer_cb_fn_t = (*pool).release_fb_cb;
        let mut cb: aom_get_frame_buffer_cb_fn_t = (*pool).get_fb_cb;
        let mut cb_priv: *mut core::ffi::c_void = (*pool).cb_priv;
        lock_buffer_pool(pool);
        if release_fb_cb.expect("non-null function pointer")(cb_priv, fb) != 0 {
            unlock_buffer_pool(pool);
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to free current frame buffer before superres upscaling\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if aom_realloc_frame_buffer(
            frame_to_show,
            (*cm).superres_upscaled_width,
            (*cm).superres_upscaled_height,
            (*seq_params).subsampling_x,
            (*seq_params).subsampling_y,
            (*seq_params).use_highbitdepth as core::ffi::c_int,
            AOM_BORDER_IN_PIXELS,
            byte_alignment,
            fb,
            cb,
            cb_priv,
            alloc_pyramid,
            0 as core::ffi::c_int,
        ) != 0
        {
            unlock_buffer_pool(pool);
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate current frame buffer for superres upscaling\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        unlock_buffer_pool(pool);
    } else {
        copy_buffer_config(frame_to_show, &raw mut copy_buffer);
        if aom_alloc_frame_buffer(
            frame_to_show,
            (*cm).superres_upscaled_width,
            (*cm).superres_upscaled_height,
            (*seq_params).subsampling_x,
            (*seq_params).subsampling_y,
            (*seq_params).use_highbitdepth as core::ffi::c_int,
            AOM_BORDER_IN_PIXELS,
            byte_alignment,
            alloc_pyramid,
            0 as core::ffi::c_int,
        ) != 0
        {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to reallocate current frame buffer for superres upscaling\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        copy_buffer_config(&raw mut copy_buffer, frame_to_show);
    }
    if (*frame_to_show).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        == (*cm).superres_upscaled_width
    {} else {
        __assert_fail(
            b"frame_to_show->y_crop_width == cm->superres_upscaled_width\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            1395 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"void av1_superres_upscale(AV1_COMMON *, BufferPool *const, _Bool)\0"))
                .as_ptr(),
        );
    };
    if (*frame_to_show).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        == (*cm).superres_upscaled_height
    {} else {
        __assert_fail(
            b"frame_to_show->y_crop_height == cm->superres_upscaled_height\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            1396 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"void av1_superres_upscale(AV1_COMMON *, BufferPool *const, _Bool)\0"))
                .as_ptr(),
        );
    };
    if (*frame_to_show).c2rust_unnamed_1.c2rust_unnamed.y_crop_width != (*cm).width
    {} else {
        __assert_fail(
            b"frame_to_show->y_crop_width != cm->width\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/resize.c\0" as *const u8
                as *const core::ffi::c_char,
            1399 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"void av1_superres_upscale(AV1_COMMON *, BufferPool *const, _Bool)\0"))
                .as_ptr(),
        );
    };
    upscale_normative_and_extend_frame(cm, &raw mut copy_buffer, frame_to_show);
    aom_free_frame_buffer(&raw mut copy_buffer);
}
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
pub const SCALE_NUMERATOR: core::ffi::c_int = 8 as core::ffi::c_int;
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SUBPEL_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const RS_SUBPEL_BITS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const RS_SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int) << RS_SUBPEL_BITS)
    - 1 as core::ffi::c_int;
pub const RS_SCALE_SUBPEL_BITS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const RS_SCALE_SUBPEL_MASK: core::ffi::c_int = ((1 as core::ffi::c_int)
    << RS_SCALE_SUBPEL_BITS) - 1 as core::ffi::c_int;
pub const RS_SCALE_EXTRA_BITS: core::ffi::c_int = RS_SCALE_SUBPEL_BITS - RS_SUBPEL_BITS;
pub const RS_SCALE_EXTRA_OFF: core::ffi::c_int = (1 as core::ffi::c_int)
    << RS_SCALE_EXTRA_BITS - 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_superres_scaled(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).width != (*cm).superres_upscaled_width) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_has_optimized_scaler(
    src_width: core::ffi::c_int,
    src_height: core::ffi::c_int,
    dst_width: core::ffi::c_int,
    dst_height: core::ffi::c_int,
) -> bool {
    let mut has_optimized_scaler: bool = dst_width * 4 as core::ffi::c_int >= src_width
        && dst_height * 4 as core::ffi::c_int >= src_height
        && (dst_width <= src_width * 16 as core::ffi::c_int
            && dst_height <= src_height * 16 as core::ffi::c_int)
        && 16 as core::ffi::c_int * dst_width % src_width == 0 as core::ffi::c_int
        && 16 as core::ffi::c_int * src_width % dst_width == 0 as core::ffi::c_int
        && 16 as core::ffi::c_int * dst_height % src_height == 0 as core::ffi::c_int
        && 16 as core::ffi::c_int * src_height % dst_height == 0 as core::ffi::c_int;
    return has_optimized_scaler;
}
pub const UPSCALE_NORMATIVE_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
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
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
}
