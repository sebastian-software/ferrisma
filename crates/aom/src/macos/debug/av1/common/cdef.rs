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

// Mechanically translated from upstream/aom/av1/common/cdef.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    static mut cdef_copy_rect8_16bit_to_16bit: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_copy_rect8_8bit_to_16bit: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            core::ffi::c_int,
            *const uint8_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    fn av1_cdef_filter_fb(
        dst8: *mut uint8_t,
        dst16: *mut uint16_t,
        dstride: core::ffi::c_int,
        in_0: *const uint16_t,
        xdec: core::ffi::c_int,
        ydec: core::ffi::c_int,
        dir: *mut [core::ffi::c_int; 16],
        dirinit: *mut core::ffi::c_int,
        var: *mut [core::ffi::c_int; 16],
        pli: core::ffi::c_int,
        dlist: *mut cdef_list,
        cdef_count: core::ffi::c_int,
        level: core::ffi::c_int,
        sec_strength: core::ffi::c_int,
        damping: core::ffi::c_int,
        coeff_shift: core::ffi::c_int,
    );
    fn av1_setup_dst_planes(
        planes: *mut macroblockd_plane,
        bsize: BLOCK_SIZE,
        src: *const YV12_BUFFER_CONFIG,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        plane_start: core::ffi::c_int,
        plane_end: core::ffi::c_int,
    );
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
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
pub type uintptr_t = usize;
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
pub type uint16_t = u16;
pub type pthread_cond_t = libc::pthread_cond_t;
pub type qm_val_t = uint8_t;
pub type TX_SIZE = uint8_t;
pub type TX_TYPE = uint8_t;
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
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const PLANE_TYPES: C2RustUnnamed_11 = 2;
pub const PLANE_TYPE_UV: C2RustUnnamed_11 = 1;
pub const PLANE_TYPE_Y: C2RustUnnamed_11 = 0;
pub type PLANE_TYPE = uint8_t;
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
pub type MACROBLOCKD = macroblockd;
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
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const BOUNDARIES: C2RustUnnamed_12 = 4;
pub const RIGHT: C2RustUnnamed_12 = 3;
pub const BOTTOM: C2RustUnnamed_12 = 2;
pub const LEFT: C2RustUnnamed_12 = 1;
pub const TOP: C2RustUnnamed_12 = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE_64X64: core::ffi::c_int = 64 as core::ffi::c_int >> MI_SIZE_LOG2;
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
unsafe extern "C-unwind" fn get_plane_type(mut plane: core::ffi::c_int) -> PLANE_TYPE {
    return (if plane == 0 as core::ffi::c_int {
        PLANE_TYPE_Y as core::ffi::c_int
    } else {
        PLANE_TYPE_UV as core::ffi::c_int
    }) as PLANE_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
pub const CDEF_SEC_STRENGTHS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const CDEF_VBORDER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const CDEF_HBORDER: core::ffi::c_int = 8 as core::ffi::c_int;
pub const CDEF_BSTRIDE: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 7 as core::ffi::c_int) + 2 as core::ffi::c_int * 8 as core::ffi::c_int
    + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
    & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int);
pub const CDEF_VERY_LARGE: core::ffi::c_int = 0x4000 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn fill_rect(
    mut dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut v: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut x: uint16_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < v {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < h {
            *dst.offset((i * dstride + j) as isize) = x;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn is_8x8_block_skip(
    mut grid: *mut *mut MB_MODE_INFO,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut mi_stride: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut mbmi: *mut *mut MB_MODE_INFO = grid
        .offset((mi_row * mi_stride) as isize)
        .offset(mi_col as isize);
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < mi_size_high[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c
            < mi_size_wide[BLOCK_8X8 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            if (**mbmi.offset(c as isize)).skip_txfm == 0 {
                return 0 as core::ffi::c_int;
            }
            c += 1;
        }
        r += 1;
        mbmi = mbmi.offset(mi_stride as isize);
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_compute_sb_list(
    mi_params: *const CommonModeInfoParams,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut dlist: *mut cdef_list,
    mut bs: BLOCK_SIZE,
) -> core::ffi::c_int {
    let mut grid: *mut *mut MB_MODE_INFO = (*mi_params).mi_grid_base;
    let mut maxc: core::ffi::c_int = (*mi_params).mi_cols - mi_col;
    let mut maxr: core::ffi::c_int = (*mi_params).mi_rows - mi_row;
    if bs as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        || bs as core::ffi::c_int == BLOCK_128X64 as core::ffi::c_int
    {
        maxc = if maxc < 128 as core::ffi::c_int >> 2 as core::ffi::c_int {
            maxc
        } else {
            128 as core::ffi::c_int >> 2 as core::ffi::c_int
        };
    } else {
        maxc = if maxc < 64 as core::ffi::c_int >> 2 as core::ffi::c_int {
            maxc
        } else {
            64 as core::ffi::c_int >> 2 as core::ffi::c_int
        };
    }
    if bs as core::ffi::c_int == BLOCK_128X128 as core::ffi::c_int
        || bs as core::ffi::c_int == BLOCK_64X128 as core::ffi::c_int
    {
        maxr = if maxr < 128 as core::ffi::c_int >> 2 as core::ffi::c_int {
            maxr
        } else {
            128 as core::ffi::c_int >> 2 as core::ffi::c_int
        };
    } else {
        maxr = if maxr < 64 as core::ffi::c_int >> 2 as core::ffi::c_int {
            maxr
        } else {
            64 as core::ffi::c_int >> 2 as core::ffi::c_int
        };
    }
    let r_step: core::ffi::c_int = 2 as core::ffi::c_int;
    let c_step: core::ffi::c_int = 2 as core::ffi::c_int;
    let r_shift: core::ffi::c_int = 1 as core::ffi::c_int;
    let c_shift: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < maxr {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < maxc {
            if is_8x8_block_skip(grid, mi_row + r, mi_col + c, (*mi_params).mi_stride)
                == 0
            {
                (*dlist.offset(count as isize)).by = (r >> r_shift) as uint8_t;
                (*dlist.offset(count as isize)).bx = (c >> c_shift) as uint8_t;
                count += 1;
            }
            c += c_step;
        }
        r += r_step;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_copy_rect8_8bit_to_16bit_c(
    mut dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *const uint8_t,
    mut sstride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < width {
            *dst.offset((i * dstride + j) as isize) = *src
                .offset((i * sstride + j) as isize) as uint16_t;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_copy_rect8_16bit_to_16bit_c(
    mut dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *const uint16_t,
    mut sstride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < width {
            *dst.offset((i * dstride + j) as isize) = *src
                .offset((i * sstride + j) as isize);
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_copy_sb8_16_lowbd(
    dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_voffset: core::ffi::c_int,
    mut src_hoffset: core::ffi::c_int,
    mut sstride: core::ffi::c_int,
    mut vsize: core::ffi::c_int,
    mut hsize: core::ffi::c_int,
) {
    let mut base: *const uint8_t = &*src
        .offset(
            (src_voffset as ptrdiff_t * sstride as ptrdiff_t + src_hoffset as ptrdiff_t)
                as isize,
        ) as *const uint8_t;
    cdef_copy_rect8_8bit_to_16bit
        .expect("non-null function pointer")(dst, dstride, base, sstride, hsize, vsize);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_copy_sb8_16_highbd(
    dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_voffset: core::ffi::c_int,
    mut src_hoffset: core::ffi::c_int,
    mut sstride: core::ffi::c_int,
    mut vsize: core::ffi::c_int,
    mut hsize: core::ffi::c_int,
) {
    let mut base: *const uint16_t = &raw mut *(((src as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t)
        .offset(
            (src_voffset as ptrdiff_t * sstride as ptrdiff_t + src_hoffset as ptrdiff_t)
                as isize,
        ) as *mut uint16_t;
    cdef_copy_rect8_16bit_to_16bit
        .expect("non-null function pointer")(dst, dstride, base, sstride, hsize, vsize);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_copy_sb8_16(
    cm: *const AV1_COMMON,
    dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *const uint8_t,
    mut src_voffset: core::ffi::c_int,
    mut src_hoffset: core::ffi::c_int,
    mut sstride: core::ffi::c_int,
    mut vsize: core::ffi::c_int,
    mut hsize: core::ffi::c_int,
) {
    if (*(*cm).seq_params).use_highbitdepth != 0 {
        av1_cdef_copy_sb8_16_highbd(
            dst,
            dstride,
            src,
            src_voffset,
            src_hoffset,
            sstride,
            vsize,
            hsize,
        );
        return;
    }
    av1_cdef_copy_sb8_16_lowbd(
        dst,
        dstride,
        src,
        src_voffset,
        src_hoffset,
        sstride,
        vsize,
        hsize,
    );
}
#[inline]
unsafe extern "C-unwind" fn copy_rect(
    mut dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *const uint16_t,
    mut sstride: core::ffi::c_int,
    mut v: core::ffi::c_int,
    mut h: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < v {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < h {
            *dst.offset((i * dstride + j) as isize) = *src
                .offset((i * sstride + j) as isize);
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn cdef_prepare_fb(
    cm: *const AV1_COMMON,
    mut fb_info: *mut CdefBlockInfo,
    colbuf: *mut *mut uint16_t,
    cdef_left: core::ffi::c_int,
    mut fbc: core::ffi::c_int,
    mut fbr: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mut src: *mut uint16_t = (*fb_info).src;
    let luma_stride: core::ffi::c_int = ((*mi_params).mi_cols << 2 as core::ffi::c_int)
        + (((1 as core::ffi::c_int) << 4 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 4 as core::ffi::c_int) - 1 as core::ffi::c_int);
    let nvfb: core::ffi::c_int = ((*mi_params).mi_rows + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    let nhfb: core::ffi::c_int = ((*mi_params).mi_cols + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    let mut cstart: core::ffi::c_int = 0 as core::ffi::c_int;
    if cdef_left == 0 {
        cstart = -CDEF_HBORDER;
    }
    let mut rend: core::ffi::c_int = 0;
    let mut cend: core::ffi::c_int = 0;
    let nhb: core::ffi::c_int = if (64 as core::ffi::c_int >> 2 as core::ffi::c_int)
        < (*mi_params).mi_cols - (64 as core::ffi::c_int >> 2 as core::ffi::c_int) * fbc
    {
        64 as core::ffi::c_int >> 2 as core::ffi::c_int
    } else {
        (*mi_params).mi_cols - (64 as core::ffi::c_int >> 2 as core::ffi::c_int) * fbc
    };
    let nvb: core::ffi::c_int = if (64 as core::ffi::c_int >> 2 as core::ffi::c_int)
        < (*mi_params).mi_rows - (64 as core::ffi::c_int >> 2 as core::ffi::c_int) * fbr
    {
        64 as core::ffi::c_int >> 2 as core::ffi::c_int
    } else {
        (*mi_params).mi_rows - (64 as core::ffi::c_int >> 2 as core::ffi::c_int) * fbr
    };
    let hsize: core::ffi::c_int = nhb << (*fb_info).mi_wide_l2;
    let vsize: core::ffi::c_int = nvb << (*fb_info).mi_high_l2;
    let mut top_linebuf: *const uint16_t = (*fb_info).top_linebuf[plane as usize];
    let mut bot_linebuf: *const uint16_t = (*fb_info).bot_linebuf[plane as usize];
    let bot_offset: core::ffi::c_int = (vsize + CDEF_VBORDER) * CDEF_BSTRIDE;
    let stride: core::ffi::c_int = luma_stride
        >> (if plane == AOM_PLANE_Y {
            0 as core::ffi::c_int
        } else {
            (*(*cm).seq_params).subsampling_x
        });
    if fbc == nhfb - 1 as core::ffi::c_int {
        cend = hsize;
    } else {
        cend = hsize + CDEF_HBORDER;
    }
    if fbr == nvfb - 1 as core::ffi::c_int {
        rend = vsize;
    } else {
        rend = vsize + CDEF_VBORDER;
    }
    av1_cdef_copy_sb8_16(
        cm,
        &raw mut *src.offset((CDEF_VBORDER * CDEF_BSTRIDE + CDEF_HBORDER + cstart) as isize),
        CDEF_BSTRIDE,
        (*fb_info).dst,
        (*fb_info).roffset,
        (*fb_info).coffset + cstart,
        (*fb_info).dst_stride,
        vsize,
        cend - cstart,
    );
    if fbr < nvfb - 1 as core::ffi::c_int {
        copy_rect(
            &raw mut *src.offset((bot_offset + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            &*bot_linebuf.offset((*fb_info).coffset as isize),
            stride,
            CDEF_VBORDER,
            hsize,
        );
    } else {
        fill_rect(
            &raw mut *src.offset((bot_offset + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            CDEF_VBORDER,
            hsize,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if fbr < nvfb - 1 as core::ffi::c_int && fbc > 0 as core::ffi::c_int {
        copy_rect(
            &raw mut *src.offset(bot_offset as isize),
            CDEF_BSTRIDE,
            &*bot_linebuf.offset(((*fb_info).coffset - CDEF_HBORDER) as isize),
            stride,
            CDEF_VBORDER,
            CDEF_HBORDER,
        );
    } else {
        fill_rect(
            &raw mut *src.offset(bot_offset as isize),
            CDEF_BSTRIDE,
            CDEF_VBORDER,
            CDEF_HBORDER,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if fbr < nvfb - 1 as core::ffi::c_int && fbc < nhfb - 1 as core::ffi::c_int {
        copy_rect(
            &raw mut *src.offset((bot_offset + hsize + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            &*bot_linebuf.offset(((*fb_info).coffset + hsize) as isize),
            stride,
            CDEF_VBORDER,
            CDEF_HBORDER,
        );
    } else {
        fill_rect(
            &raw mut *src.offset((bot_offset + hsize + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            CDEF_VBORDER,
            CDEF_HBORDER,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if fbr > 0 as core::ffi::c_int {
        copy_rect(
            &raw mut *src.offset(CDEF_HBORDER as isize),
            CDEF_BSTRIDE,
            &*top_linebuf.offset((*fb_info).coffset as isize),
            stride,
            CDEF_VBORDER,
            hsize,
        );
    } else {
        fill_rect(
            &raw mut *src.offset(CDEF_HBORDER as isize),
            CDEF_BSTRIDE,
            CDEF_VBORDER,
            hsize,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if fbr > 0 as core::ffi::c_int && fbc > 0 as core::ffi::c_int {
        copy_rect(
            src,
            CDEF_BSTRIDE,
            &*top_linebuf.offset(((*fb_info).coffset - CDEF_HBORDER) as isize),
            stride,
            CDEF_VBORDER,
            CDEF_HBORDER,
        );
    } else {
        fill_rect(
            src,
            CDEF_BSTRIDE,
            CDEF_VBORDER,
            CDEF_HBORDER,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if fbr > 0 as core::ffi::c_int && fbc < nhfb - 1 as core::ffi::c_int {
        copy_rect(
            &raw mut *src.offset((hsize + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            &*top_linebuf.offset(((*fb_info).coffset + hsize) as isize),
            stride,
            CDEF_VBORDER,
            CDEF_HBORDER,
        );
    } else {
        fill_rect(
            &raw mut *src.offset((hsize + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            CDEF_VBORDER,
            CDEF_HBORDER,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if cdef_left != 0 {
        copy_rect(
            src,
            CDEF_BSTRIDE,
            *colbuf.offset(plane as isize),
            CDEF_HBORDER,
            rend + CDEF_VBORDER,
            CDEF_HBORDER,
        );
    }
    copy_rect(
        *colbuf.offset(plane as isize),
        CDEF_HBORDER,
        src.offset(hsize as isize),
        CDEF_BSTRIDE,
        rend + CDEF_VBORDER,
        CDEF_HBORDER,
    );
    if (*fb_info).frame_boundary[LEFT as core::ffi::c_int as usize] != 0 {
        fill_rect(
            src,
            CDEF_BSTRIDE,
            vsize + 2 as core::ffi::c_int * CDEF_VBORDER,
            CDEF_HBORDER,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
    if (*fb_info).frame_boundary[RIGHT as core::ffi::c_int as usize] != 0 {
        fill_rect(
            &raw mut *src.offset((hsize + CDEF_HBORDER) as isize),
            CDEF_BSTRIDE,
            vsize + 2 as core::ffi::c_int * CDEF_VBORDER,
            CDEF_HBORDER,
            CDEF_VERY_LARGE as uint16_t,
        );
    }
}
#[inline]
unsafe extern "C-unwind" fn cdef_filter_fb(
    fb_info: *mut CdefBlockInfo,
    mut plane: core::ffi::c_int,
    mut use_highbitdepth: uint8_t,
) {
    let mut offset: ptrdiff_t = (*fb_info).dst_stride as ptrdiff_t
        * (*fb_info).roffset as ptrdiff_t + (*fb_info).coffset as ptrdiff_t;
    if use_highbitdepth != 0 {
        av1_cdef_filter_fb(
            0 as *mut uint8_t,
            ((((*fb_info).dst).offset(offset as isize) as uintptr_t)
                << 1 as core::ffi::c_int) as *mut uint16_t,
            (*fb_info).dst_stride,
            &raw mut *((*fb_info).src)
                .offset((CDEF_VBORDER * CDEF_BSTRIDE + CDEF_HBORDER) as isize),
            (*fb_info).xdec,
            (*fb_info).ydec,
            ((*fb_info).dir).as_mut_ptr(),
            0 as *mut core::ffi::c_int,
            ((*fb_info).var).as_mut_ptr(),
            plane,
            ((*fb_info).dlist).as_mut_ptr(),
            (*fb_info).cdef_count,
            (*fb_info).level,
            (*fb_info).sec_strength,
            (*fb_info).damping,
            (*fb_info).coeff_shift,
        );
    } else {
        av1_cdef_filter_fb(
            ((*fb_info).dst).offset(offset as isize),
            0 as *mut uint16_t,
            (*fb_info).dst_stride,
            &raw mut *((*fb_info).src)
                .offset((CDEF_VBORDER * CDEF_BSTRIDE + CDEF_HBORDER) as isize),
            (*fb_info).xdec,
            (*fb_info).ydec,
            ((*fb_info).dir).as_mut_ptr(),
            0 as *mut core::ffi::c_int,
            ((*fb_info).var).as_mut_ptr(),
            plane,
            ((*fb_info).dlist).as_mut_ptr(),
            (*fb_info).cdef_count,
            (*fb_info).level,
            (*fb_info).sec_strength,
            (*fb_info).damping,
            (*fb_info).coeff_shift,
        );
    };
}
#[inline]
unsafe extern "C-unwind" fn cdef_init_fb_col(
    xd: *const MACROBLOCKD,
    fb_info: *mut CdefBlockInfo,
    mut level: *mut core::ffi::c_int,
    mut sec_strength: *mut core::ffi::c_int,
    mut fbc: core::ffi::c_int,
    mut fbr: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let plane_type: PLANE_TYPE = get_plane_type(plane) as PLANE_TYPE;
    (*fb_info).level = *level.offset(plane_type as isize);
    (*fb_info).sec_strength = *sec_strength.offset(plane_type as isize);
    (*fb_info).dst = (*xd).plane[plane as usize].dst.buf;
    (*fb_info).dst_stride = (*xd).plane[plane as usize].dst.stride;
    (*fb_info).xdec = (*xd).plane[plane as usize].subsampling_x;
    (*fb_info).ydec = (*xd).plane[plane as usize].subsampling_y;
    (*fb_info).mi_wide_l2 = MI_SIZE_LOG2 - (*xd).plane[plane as usize].subsampling_x;
    (*fb_info).mi_high_l2 = MI_SIZE_LOG2 - (*xd).plane[plane as usize].subsampling_y;
    (*fb_info).roffset = MI_SIZE_64X64 * fbr << (*fb_info).mi_high_l2;
    (*fb_info).coffset = MI_SIZE_64X64 * fbc << (*fb_info).mi_wide_l2;
}
unsafe extern "C-unwind" fn cdef_fb_col(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    fb_info: *mut CdefBlockInfo,
    colbuf: *mut *mut uint16_t,
    mut cdef_left: *mut core::ffi::c_int,
    mut fbc: core::ffi::c_int,
    mut fbr: core::ffi::c_int,
) {
    let mi_params: *const CommonModeInfoParams = &(*cm).mi_params;
    let mbmi_cdef_strength: core::ffi::c_int = (**((*mi_params).mi_grid_base)
        .offset(
            (MI_SIZE_64X64 * fbr * (*mi_params).mi_stride + MI_SIZE_64X64 * fbc) as isize,
        ))
        .cdef_strength() as core::ffi::c_int;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut is_zero_level: [core::ffi::c_int; 2] = [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ];
    let mut level: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
    let mut sec_strength: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
    let cdef_info: *const CdefInfo = &(*cm).cdef_info;
    if (*((*mi_params).mi_grid_base)
        .offset(
            (MI_SIZE_64X64 * fbr * (*mi_params).mi_stride + MI_SIZE_64X64 * fbc) as isize,
        ))
        .is_null() || mbmi_cdef_strength == -(1 as core::ffi::c_int)
    {
        memset(
            cdef_left as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (num_planes as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        return;
    }
    level[PLANE_TYPE_Y as core::ffi::c_int as usize] = (*cdef_info)
        .cdef_strengths[mbmi_cdef_strength as usize] / CDEF_SEC_STRENGTHS;
    sec_strength[PLANE_TYPE_Y as core::ffi::c_int as usize] = (*cdef_info)
        .cdef_strengths[mbmi_cdef_strength as usize] % CDEF_SEC_STRENGTHS;
    sec_strength[PLANE_TYPE_Y as core::ffi::c_int as usize]
        += (sec_strength[PLANE_TYPE_Y as core::ffi::c_int as usize]
            == 3 as core::ffi::c_int) as core::ffi::c_int;
    is_zero_level[PLANE_TYPE_Y as core::ffi::c_int as usize] = (level[PLANE_TYPE_Y
        as core::ffi::c_int as usize] == 0 as core::ffi::c_int
        && sec_strength[PLANE_TYPE_Y as core::ffi::c_int as usize]
            == 0 as core::ffi::c_int) as core::ffi::c_int;
    if num_planes > 1 as core::ffi::c_int {
        level[PLANE_TYPE_UV as core::ffi::c_int as usize] = (*cdef_info)
            .cdef_uv_strengths[mbmi_cdef_strength as usize] / CDEF_SEC_STRENGTHS;
        sec_strength[PLANE_TYPE_UV as core::ffi::c_int as usize] = (*cdef_info)
            .cdef_uv_strengths[mbmi_cdef_strength as usize] % CDEF_SEC_STRENGTHS;
        sec_strength[PLANE_TYPE_UV as core::ffi::c_int as usize]
            += (sec_strength[PLANE_TYPE_UV as core::ffi::c_int as usize]
                == 3 as core::ffi::c_int) as core::ffi::c_int;
        is_zero_level[PLANE_TYPE_UV as core::ffi::c_int as usize] = (level[PLANE_TYPE_UV
            as core::ffi::c_int as usize] == 0 as core::ffi::c_int
            && sec_strength[PLANE_TYPE_UV as core::ffi::c_int as usize]
                == 0 as core::ffi::c_int) as core::ffi::c_int;
    }
    if is_zero_level[PLANE_TYPE_Y as core::ffi::c_int as usize] != 0
        && is_zero_level[PLANE_TYPE_UV as core::ffi::c_int as usize] != 0
    {
        memset(
            cdef_left as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (num_planes as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        return;
    }
    (*fb_info).cdef_count = av1_cdef_compute_sb_list(
        mi_params,
        fbr * MI_SIZE_64X64,
        fbc * MI_SIZE_64X64,
        ((*fb_info).dlist).as_mut_ptr(),
        BLOCK_64X64,
    );
    if (*fb_info).cdef_count == 0 {
        memset(
            cdef_left as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (num_planes as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        return;
    }
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if plane != 0 && is_zero_level[get_plane_type(plane) as usize] != 0 {
            *cdef_left.offset(plane as isize) = 0 as core::ffi::c_int;
        } else {
            cdef_init_fb_col(
                xd,
                fb_info,
                level.as_mut_ptr(),
                sec_strength.as_mut_ptr(),
                fbc,
                fbr,
                plane,
            );
            cdef_prepare_fb(
                cm,
                fb_info,
                colbuf,
                *cdef_left.offset(plane as isize),
                fbc,
                fbr,
                plane,
            );
            cdef_filter_fb(fb_info, plane, (*(*cm).seq_params).use_highbitdepth);
            *cdef_left.offset(plane as isize) = 1 as core::ffi::c_int;
        }
        plane += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_init_fb_row(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    fb_info: *mut CdefBlockInfo,
    linebuf: *mut *mut uint16_t,
    src: *mut uint16_t,
    cdef_sync: *mut AV1CdefSyncData,
    mut fbr: core::ffi::c_int,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let nvfb: core::ffi::c_int = ((*cm).mi_params.mi_rows + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    let luma_stride: core::ffi::c_int = ((*cm).mi_params.mi_cols
        << 2 as core::ffi::c_int)
        + (((1 as core::ffi::c_int) << 4 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 4 as core::ffi::c_int) - 1 as core::ffi::c_int);
    let ping_pong: bool = fbr & 1 as core::ffi::c_int != 0;
    (*fb_info).frame_boundary[TOP as core::ffi::c_int as usize] = if MI_SIZE_64X64 * fbr
        == 0 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if fbr != nvfb - 1 as core::ffi::c_int {
        (*fb_info).frame_boundary[BOTTOM as core::ffi::c_int as usize] = if MI_SIZE_64X64
            * (fbr + 1 as core::ffi::c_int) == (*cm).mi_params.mi_rows
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
    } else {
        (*fb_info).frame_boundary[BOTTOM as core::ffi::c_int as usize] = 1
            as core::ffi::c_int;
    }
    (*fb_info).src = src;
    (*fb_info).damping = (*cm).cdef_info.cdef_damping;
    (*fb_info).coeff_shift = (if ((*(*cm).seq_params).bit_depth as core::ffi::c_uint)
        .wrapping_sub(8 as core::ffi::c_uint) > 0 as core::ffi::c_uint
    {
        ((*(*cm).seq_params).bit_depth as core::ffi::c_uint)
            .wrapping_sub(8 as core::ffi::c_uint)
    } else {
        0 as core::ffi::c_uint
    }) as core::ffi::c_int;
    memset(
        &raw mut (*fb_info).dir as *mut [[core::ffi::c_int; 16]; 16]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[core::ffi::c_int; 16]; 16]>() as size_t,
    );
    memset(
        &raw mut (*fb_info).var as *mut [[core::ffi::c_int; 16]; 16]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[core::ffi::c_int; 16]; 16]>() as size_t,
    );
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let mi_high_l2: core::ffi::c_int = MI_SIZE_LOG2
            - (*xd).plane[plane as usize].subsampling_y;
        let offset: core::ffi::c_int = MI_SIZE_64X64 * (fbr + 1 as core::ffi::c_int)
            << mi_high_l2;
        let stride: core::ffi::c_int = luma_stride
            >> (*xd).plane[plane as usize].subsampling_x;
        let top_linebuf: *mut uint16_t = &raw mut *(*linebuf.offset(plane as isize))
            .offset((ping_pong as core::ffi::c_int * CDEF_VBORDER * stride) as isize)
            as *mut uint16_t;
        (*fb_info).bot_linebuf[plane as usize] = &raw mut *(*linebuf.offset(plane as isize))
            .offset(((CDEF_VBORDER << 1 as core::ffi::c_int) * stride) as isize)
            as *mut uint16_t;
        if fbr != nvfb - 1 as core::ffi::c_int {
            av1_cdef_copy_sb8_16(
                cm,
                top_linebuf,
                stride,
                (*xd).plane[plane as usize].dst.buf,
                offset - CDEF_VBORDER,
                0 as core::ffi::c_int,
                (*xd).plane[plane as usize].dst.stride,
                CDEF_VBORDER,
                stride,
            );
        }
        (*fb_info).top_linebuf[plane as usize] = &raw mut *(*linebuf.offset(plane as isize))
            .offset((!ping_pong as core::ffi::c_int * CDEF_VBORDER * stride) as isize)
            as *mut uint16_t;
        if fbr != nvfb - 1 as core::ffi::c_int {
            av1_cdef_copy_sb8_16(
                cm,
                (*fb_info).bot_linebuf[plane as usize],
                stride,
                (*xd).plane[plane as usize].dst.buf,
                offset,
                0 as core::ffi::c_int,
                (*xd).plane[plane as usize].dst.stride,
                CDEF_VBORDER,
                stride,
            );
        }
        plane += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_fb_row(
    cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    linebuf: *mut *mut uint16_t,
    colbuf: *mut *mut uint16_t,
    src: *mut uint16_t,
    mut fbr: core::ffi::c_int,
    mut cdef_init_fb_row_fn: cdef_init_fb_row_t,
    cdef_sync: *mut AV1CdefSyncData,
    mut error_info: *mut aom_internal_error_info,
) {
    let mut fb_info: CdefBlockInfo = CdefBlockInfo {
        src: 0 as *mut uint16_t,
        top_linebuf: [0 as *mut uint16_t; 3],
        bot_linebuf: [0 as *mut uint16_t; 3],
        dst: 0 as *mut uint8_t,
        dlist: [cdef_list { by: 0, bx: 0 }; 256],
        xdec: 0,
        ydec: 0,
        mi_wide_l2: 0,
        mi_high_l2: 0,
        frame_boundary: [0; 4],
        damping: 0,
        coeff_shift: 0,
        level: 0,
        sec_strength: 0,
        cdef_count: 0,
        dir: [[0; 16]; 16],
        var: [[0; 16]; 16],
        dst_stride: 0,
        coffset: 0,
        roffset: 0,
    };
    let mut cdef_left: [core::ffi::c_int; 3] = [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ];
    let nhfb: core::ffi::c_int = ((*cm).mi_params.mi_cols + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    cdef_init_fb_row_fn
        .expect(
            "non-null function pointer",
        )(cm, xd, &raw mut fb_info, linebuf, src, cdef_sync, fbr);
    if !cdef_sync.is_null()
        && (*cm).cdef_info.allocated_num_workers > 1 as core::ffi::c_int
    {
        pthread_mutex_lock((*cdef_sync).mutex_ as *mut pthread_mutex_t);
        let cdef_mt_exit: bool = (*cdef_sync).cdef_mt_exit;
        pthread_mutex_unlock((*cdef_sync).mutex_ as *mut pthread_mutex_t);
        if cdef_mt_exit {
            return;
        }
    }
    let mut fbc: core::ffi::c_int = 0 as core::ffi::c_int;
    while fbc < nhfb {
        fb_info.frame_boundary[LEFT as core::ffi::c_int as usize] = if MI_SIZE_64X64
            * fbc == 0 as core::ffi::c_int
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        if fbc != nhfb - 1 as core::ffi::c_int {
            fb_info.frame_boundary[RIGHT as core::ffi::c_int as usize] = if MI_SIZE_64X64
                * (fbc + 1 as core::ffi::c_int) == (*cm).mi_params.mi_cols
            {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            };
        } else {
            fb_info.frame_boundary[RIGHT as core::ffi::c_int as usize] = 1
                as core::ffi::c_int;
        }
        cdef_fb_col(
            cm,
            xd,
            &raw mut fb_info,
            colbuf,
            &raw mut *cdef_left.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
            fbc,
            fbr,
        );
        fbc += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_frame(
    mut frame: *mut YV12_BUFFER_CONFIG,
    cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut cdef_init_fb_row_fn: cdef_init_fb_row_t,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let nvfb: core::ffi::c_int = ((*cm).mi_params.mi_rows + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        (*(*cm).seq_params).sb_size,
        frame,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        num_planes,
    );
    let mut fbr: core::ffi::c_int = 0 as core::ffi::c_int;
    while fbr < nvfb {
        av1_cdef_fb_row(
            cm,
            xd,
            ((*cm).cdef_info.linebuf).as_mut_ptr(),
            ((*cm).cdef_info.colbuf).as_mut_ptr(),
            (*cm).cdef_info.srcbuf,
            fbr,
            cdef_init_fb_row_fn,
            0 as *mut AV1CdefSyncData,
            (*xd).error_info as *mut aom_internal_error_info,
        );
        fbr += 1;
    }
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
}
