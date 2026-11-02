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

// Mechanically translated from upstream/aom/av1/encoder/reconinter_enc.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn aom_blend_a64_mask_c(
        dst: *mut uint8_t,
        dst_stride: uint32_t,
        src0: *const uint8_t,
        src0_stride: uint32_t,
        src1: *const uint8_t,
        src1_stride: uint32_t,
        mask: *const uint8_t,
        mask_stride: uint32_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        subw: core::ffi::c_int,
        subh: core::ffi::c_int,
    );
    fn aom_comp_mask_pred_c(
        comp_pred: *mut uint8_t,
        pred: *const uint8_t,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ref_0: *const uint8_t,
        ref_stride: core::ffi::c_int,
        mask: *const uint8_t,
        mask_stride: core::ffi::c_int,
        invert_mask: core::ffi::c_int,
    );
    fn aom_convolve8_horiz_c(
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
    );
    fn aom_convolve8_vert_c(
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
    );
    fn aom_convolve_copy_c(
        src: *const uint8_t,
        src_stride: ptrdiff_t,
        dst: *mut uint8_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
    fn aom_highbd_blend_a64_mask_c(
        dst: *mut uint8_t,
        dst_stride: uint32_t,
        src0: *const uint8_t,
        src0_stride: uint32_t,
        src1: *const uint8_t,
        src1_stride: uint32_t,
        mask: *const uint8_t,
        mask_stride: uint32_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
        subw: core::ffi::c_int,
        subh: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn aom_highbd_comp_mask_pred_c(
        comp_pred: *mut uint8_t,
        pred8: *const uint8_t,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ref8: *const uint8_t,
        ref_stride: core::ffi::c_int,
        mask: *const uint8_t,
        mask_stride: core::ffi::c_int,
        invert_mask: core::ffi::c_int,
    );
    fn aom_highbd_convolve8_horiz_c(
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
        bd: core::ffi::c_int,
    );
    fn aom_highbd_convolve8_vert_c(
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
        bd: core::ffi::c_int,
    );
    fn aom_highbd_convolve_copy_c(
        src: *const uint16_t,
        src_stride: ptrdiff_t,
        dst: *mut uint16_t,
        dst_stride: ptrdiff_t,
        w: core::ffi::c_int,
        h: core::ffi::c_int,
    );
    fn av1_build_compound_diffwtd_mask_c(
        mask: *mut uint8_t,
        mask_type: DIFFWTD_MASK_TYPE,
        src0: *const uint8_t,
        src0_stride: core::ffi::c_int,
        src1: *const uint8_t,
        src1_stride: core::ffi::c_int,
        h: core::ffi::c_int,
        w: core::ffi::c_int,
    );
    fn av1_build_compound_diffwtd_mask_highbd_c(
        mask: *mut uint8_t,
        mask_type: DIFFWTD_MASK_TYPE,
        src0: *const uint8_t,
        src0_stride: core::ffi::c_int,
        src1: *const uint8_t,
        src1_stride: core::ffi::c_int,
        h: core::ffi::c_int,
        w: core::ffi::c_int,
        bd: core::ffi::c_int,
    );
    fn av1_init_warp_params(
        inter_pred_params: *mut InterPredParams,
        warp_types: *const WarpTypesAllowed,
        ref_0: core::ffi::c_int,
        xd: *const MACROBLOCKD,
        mi: *const MB_MODE_INFO,
    );
    fn av1_skip_u4x4_pred_in_obmc(
        bsize: BLOCK_SIZE,
        pd: *const macroblockd_plane,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_make_inter_predictor(
        src: *const uint8_t,
        src_stride: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        inter_pred_params: *mut InterPredParams,
        subpel_params: *const SubpelParams,
    );
    fn av1_make_masked_inter_predictor(
        pre: *const uint8_t,
        pre_stride: core::ffi::c_int,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        inter_pred_params: *mut InterPredParams,
        subpel_params: *const SubpelParams,
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
    fn av1_setup_pre_planes(
        xd: *mut MACROBLOCKD,
        idx: core::ffi::c_int,
        src: *const YV12_BUFFER_CONFIG,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        sf: *const scale_factors,
        num_planes: core::ffi::c_int,
    );
    fn av1_setup_obmc_dst_bufs(
        xd: *mut MACROBLOCKD,
        dst_buf1: *mut *mut uint8_t,
        dst_buf2: *mut *mut uint8_t,
    );
    fn av1_build_obmc_inter_prediction(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        above: *mut *mut uint8_t,
        above_stride: *mut core::ffi::c_int,
        left: *mut *mut uint8_t,
        left_stride: *mut core::ffi::c_int,
    );
    fn av1_dist_wtd_comp_weight_assign(
        cm: *const AV1_COMMON,
        mbmi: *const MB_MODE_INFO,
        fwd_offset: *mut core::ffi::c_int,
        bck_offset: *mut core::ffi::c_int,
        use_dist_wtd_comp_avg: *mut core::ffi::c_int,
        is_compound: core::ffi::c_int,
    );
    fn av1_get_compound_type_mask(
        comp_data: *const INTERINTER_COMPOUND_DATA,
        sb_type: BLOCK_SIZE,
    ) -> *const uint8_t;
    fn av1_build_interintra_predictor(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        pred: *mut uint8_t,
        stride: core::ffi::c_int,
        ctx: *const BUFFER_SET,
        plane: core::ffi::c_int,
        bsize: BLOCK_SIZE,
    );
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
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
pub type PLANE_TYPE = uint8_t;
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_11 = 25;
pub const INTRA_MODES: C2RustUnnamed_11 = 13;
pub const INTER_MODE_END: C2RustUnnamed_11 = 25;
pub const INTER_MODE_START: C2RustUnnamed_11 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_11 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_11 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_11 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_11 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_11 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_11 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_11 = 13;
pub const DIR_MODE_END: C2RustUnnamed_11 = 9;
pub const DIR_MODE_START: C2RustUnnamed_11 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_11 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_11 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_11 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_11 = 25;
pub const NEW_NEWMV: C2RustUnnamed_11 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_11 = 23;
pub const NEW_NEARMV: C2RustUnnamed_11 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_11 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_11 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_11 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_11 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_11 = 17;
pub const NEWMV: C2RustUnnamed_11 = 16;
pub const GLOBALMV: C2RustUnnamed_11 = 15;
pub const NEARMV: C2RustUnnamed_11 = 14;
pub const NEARESTMV: C2RustUnnamed_11 = 13;
pub const PAETH_PRED: C2RustUnnamed_11 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_11 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_11 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_11 = 9;
pub const D67_PRED: C2RustUnnamed_11 = 8;
pub const D203_PRED: C2RustUnnamed_11 = 7;
pub const D157_PRED: C2RustUnnamed_11 = 6;
pub const D113_PRED: C2RustUnnamed_11 = 5;
pub const D135_PRED: C2RustUnnamed_11 = 4;
pub const D45_PRED: C2RustUnnamed_11 = 3;
pub const H_PRED: C2RustUnnamed_11 = 2;
pub const V_PRED: C2RustUnnamed_11 = 1;
pub const DC_PRED: C2RustUnnamed_11 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const MOTION_MODES: C2RustUnnamed_12 = 3;
pub const WARPED_CAUSAL: C2RustUnnamed_12 = 2;
pub const OBMC_CAUSAL: C2RustUnnamed_12 = 1;
pub const SIMPLE_TRANSLATION: C2RustUnnamed_12 = 0;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const MASKED_COMPOUND_TYPES: C2RustUnnamed_13 = 2;
pub const COMPOUND_TYPES: C2RustUnnamed_13 = 4;
pub const COMPOUND_DIFFWTD: C2RustUnnamed_13 = 3;
pub const COMPOUND_WEDGE: C2RustUnnamed_13 = 2;
pub const COMPOUND_DISTWTD: C2RustUnnamed_13 = 1;
pub const COMPOUND_AVERAGE: C2RustUnnamed_13 = 0;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_14 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_14 = 7;
pub const BWD_REFS: C2RustUnnamed_14 = 3;
pub const FWD_REFS: C2RustUnnamed_14 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_14 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_14 = 8;
pub const REF_FRAMES: C2RustUnnamed_14 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_14 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_14 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_14 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_14 = 4;
pub const LAST3_FRAME: C2RustUnnamed_14 = 3;
pub const LAST2_FRAME: C2RustUnnamed_14 = 2;
pub const LAST_FRAME: C2RustUnnamed_14 = 1;
pub const INTRA_FRAME: C2RustUnnamed_14 = 0;
pub const NONE_FRAME: C2RustUnnamed_14 = -1;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
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
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const TRANS_TYPES: C2RustUnnamed_15 = 4;
pub const AFFINE: C2RustUnnamed_15 = 3;
pub const ROTZOOM: C2RustUnnamed_15 = 2;
pub const TRANSLATION: C2RustUnnamed_15 = 1;
pub const IDENTITY: C2RustUnnamed_15 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WarpTypesAllowed {
    pub global_warp_allowed: core::ffi::c_int,
    pub local_warp_allowed: core::ffi::c_int,
}
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const USE_8_TAPS: C2RustUnnamed_16 = 3;
pub const USE_4_TAPS: C2RustUnnamed_16 = 2;
pub const USE_2_TAPS: C2RustUnnamed_16 = 1;
pub const USE_2_TAPS_ORIG: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilterParams {
    pub filter_ptr: *const int16_t,
    pub taps: uint16_t,
    pub interp_filter: InterpFilter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConvolveParams {
    pub do_average: core::ffi::c_int,
    pub dst: *mut CONV_BUF_TYPE,
    pub dst_stride: core::ffi::c_int,
    pub round_0: core::ffi::c_int,
    pub round_1: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub is_compound: core::ffi::c_int,
    pub use_dist_wtd_comp_avg: core::ffi::c_int,
    pub fwd_offset: core::ffi::c_int,
    pub bck_offset: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUFFER_SET {
    pub plane: [*mut uint8_t; 3],
    pub stride: [core::ffi::c_int; 3],
}
pub type MACROBLOCKD_PLANE = macroblockd_plane;
pub type MACROBLOCKD = macroblockd;
pub type AV1_COMMON = AV1Common;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterPredParams {
    pub mode: InterPredMode,
    pub comp_mode: InterCompMode,
    pub warp_params: WarpedMotionParams,
    pub conv_params: ConvolveParams,
    pub interp_filter_params: [*const InterpFilterParams; 2],
    pub block_width: core::ffi::c_int,
    pub block_height: core::ffi::c_int,
    pub pix_row: core::ffi::c_int,
    pub pix_col: core::ffi::c_int,
    pub ref_frame_buf: buf_2d,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub scale_factors: *const scale_factors,
    pub bit_depth: core::ffi::c_int,
    pub use_hbd_buf: core::ffi::c_int,
    pub mask_comp: INTERINTER_COMPOUND_DATA,
    pub sb_type: BLOCK_SIZE,
    pub is_intrabc: core::ffi::c_int,
    pub top: core::ffi::c_int,
    pub left: core::ffi::c_int,
}
pub type InterCompMode = core::ffi::c_uint;
pub const MASK_COMP: InterCompMode = 2;
pub const UNIFORM_COMP: InterCompMode = 1;
pub const UNIFORM_SINGLE: InterCompMode = 0;
pub type InterPredMode = core::ffi::c_uint;
pub const WARP_PRED: InterPredMode = 1;
pub const TRANSLATION_PRED: InterPredMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpelParams {
    pub xs: core::ffi::c_int,
    pub ys: core::ffi::c_int,
    pub subpel_x: core::ffi::c_int,
    pub subpel_y: core::ffi::c_int,
    pub pos_x: core::ffi::c_int,
    pub pos_y: core::ffi::c_int,
}
pub type overlappable_nb_visitor_t = Option<
    unsafe extern "C-unwind" fn(
        *mut MACROBLOCKD,
        core::ffi::c_int,
        core::ffi::c_int,
        uint8_t,
        core::ffi::c_int,
        *mut MB_MODE_INFO,
        *mut core::ffi::c_void,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct build_prediction_ctxt {
    pub cm: *const AV1_COMMON,
    pub tmp_buf: *mut *mut uint8_t,
    pub tmp_width: *mut core::ffi::c_int,
    pub tmp_height: *mut core::ffi::c_int,
    pub tmp_stride: *mut core::ffi::c_int,
    pub mb_to_far_edge: core::ffi::c_int,
    pub dcb: *mut core::ffi::c_void,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MAX_SB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_SB_SIZE_LOG2;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
static mut mi_size_wide_log2: [uint8_t; 22] = [
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
];
static mut mi_size_high_log2: [uint8_t; 22] = [
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
];
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
unsafe extern "C-unwind" fn get_ref_scale_factors_const(
    cm: *const AV1_COMMON,
    ref_frame: MV_REFERENCE_FRAME,
) -> *const scale_factors {
    let map_idx: core::ffi::c_int = get_ref_frame_map_idx(cm, ref_frame)
        as core::ffi::c_int;
    return if map_idx != INVALID_IDX {
        &*((*cm).ref_scale_factors).as_ptr().offset(map_idx as isize)
            as *const scale_factors
    } else {
        0 as *const scale_factors
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
pub const AOM_INTERP_EXTEND: core::ffi::c_int = 4 as core::ffi::c_int;
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_broadcast_interp_filter(
    mut filter: InterpFilter,
) -> int_interpfilters {
    let mut filters: int_interpfilters = int_interpfilters { as_int: 0 };
    filters.as_filters.x_filter = filter as uint16_t;
    filters.as_filters.y_filter = filter as uint16_t;
    return filters;
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
static mut av1_intrabc_bilinear_filter: crate::platform::Aligned256<[int16_t; 32]> = crate::platform::Aligned256([
    128 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    64 as core::ffi::c_int as int16_t,
    64 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
]);
static mut av1_intrabc_filter_params: InterpFilterParams = unsafe {
    {
        let mut init = InterpFilterParams {
            filter_ptr: core::ptr::addr_of!(av1_intrabc_bilinear_filter.0).cast::<int16_t>(),
            taps: 2 as uint16_t,
            interp_filter: BILINEAR,
        };
        init
    }
};
static mut av1_sub_pel_filters_4: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
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
        -(4 as core::ffi::c_int) as int16_t,
        126 as core::ffi::c_int as int16_t,
        8 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        122 as core::ffi::c_int as int16_t,
        18 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        116 as core::ffi::c_int as int16_t,
        28 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        110 as core::ffi::c_int as int16_t,
        38 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        102 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        94 as core::ffi::c_int as int16_t,
        58 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        84 as core::ffi::c_int as int16_t,
        66 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        76 as core::ffi::c_int as int16_t,
        76 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        66 as core::ffi::c_int as int16_t,
        84 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        58 as core::ffi::c_int as int16_t,
        94 as core::ffi::c_int as int16_t,
        -(14 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        48 as core::ffi::c_int as int16_t,
        102 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        38 as core::ffi::c_int as int16_t,
        110 as core::ffi::c_int as int16_t,
        -(12 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(6 as core::ffi::c_int) as int16_t,
        28 as core::ffi::c_int as int16_t,
        116 as core::ffi::c_int as int16_t,
        -(10 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        18 as core::ffi::c_int as int16_t,
        122 as core::ffi::c_int as int16_t,
        -(8 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(2 as core::ffi::c_int) as int16_t,
        8 as core::ffi::c_int as int16_t,
        126 as core::ffi::c_int as int16_t,
        -(4 as core::ffi::c_int) as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_sub_pel_filters_4smooth: crate::platform::Aligned256<[InterpKernel; 16]> = crate::platform::Aligned256([
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
        30 as core::ffi::c_int as int16_t,
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
        0 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        52 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
    [
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        12 as core::ffi::c_int as int16_t,
        48 as core::ffi::c_int as int16_t,
        54 as core::ffi::c_int as int16_t,
        14 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
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
        30 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
    ],
]);
static mut av1_interp_4tap: [InterpFilterParams; 4] = unsafe {
    [
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_4.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_REGULAR,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_4smooth.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_SMOOTH,
            };
            init
        },
        {
            let mut init = InterpFilterParams {
                filter_ptr: core::ptr::addr_of!(av1_sub_pel_filters_4.0).cast::<InterpKernel>() as *const int16_t,
                taps: SUBPEL_TAPS as uint16_t,
                interp_filter: EIGHTTAP_REGULAR,
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
    ]
};
#[inline]
unsafe extern "C-unwind" fn av1_get_interp_filter_params_with_block_size(
    interp_filter: InterpFilter,
    w: core::ffi::c_int,
) -> *const InterpFilterParams {
    if w <= 4 as core::ffi::c_int
        && interp_filter as core::ffi::c_int != MULTITAP_SHARP2 as core::ffi::c_int
    {
        return &*core::ptr::addr_of!(av1_interp_4tap).cast::<InterpFilterParams>().offset(interp_filter as isize)
            as *const InterpFilterParams;
    }
    return &*core::ptr::addr_of!(av1_interp_filter_params_list).cast::<InterpFilterParams>().offset(interp_filter as isize)
        as *const InterpFilterParams;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_interp_filter_subpel_kernel(
    filter_params: *const InterpFilterParams,
    subpel: core::ffi::c_int,
) -> *const int16_t {
    return ((*filter_params).filter_ptr)
        .offset(((*filter_params).taps as core::ffi::c_int * subpel) as isize);
}
#[inline]
unsafe extern "C-unwind" fn av1_get_filter(
    mut subpel_search: core::ffi::c_int,
) -> *const InterpFilterParams {
    if subpel_search >= USE_2_TAPS as core::ffi::c_int {} else {
        __assert_fail(
            b"subpel_search >= USE_2_TAPS\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/filter.h\0" as *const u8
                as *const core::ffi::c_char,
            272 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"const InterpFilterParams *av1_get_filter(int)\0"))
                .as_ptr(),
        );
    };
    match subpel_search {
        1 => {
            return &*core::ptr::addr_of!(av1_interp_4tap).cast::<InterpFilterParams>()
                .offset(BILINEAR as core::ffi::c_int as isize)
                as *const InterpFilterParams;
        }
        2 => {
            return &*core::ptr::addr_of!(av1_interp_4tap).cast::<InterpFilterParams>()
                .offset(EIGHTTAP_REGULAR as core::ffi::c_int as isize)
                as *const InterpFilterParams;
        }
        3 => {
            return &*core::ptr::addr_of!(av1_interp_filter_params_list).cast::<InterpFilterParams>()
                .offset(EIGHTTAP_REGULAR as core::ffi::c_int as isize)
                as *const InterpFilterParams;
        }
        _ => {
            if 0 as core::ffi::c_int != 0 {} else {
                __assert_fail(
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/common/filter.h\0" as *const u8
                        as *const core::ffi::c_char,
                    278 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 46],
                        [core::ffi::c_char; 46],
                    >(*b"const InterpFilterParams *av1_get_filter(int)\0"))
                        .as_ptr(),
                );
            };
            return 0 as *const InterpFilterParams;
        }
    };
}
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn is_masked_compound_type(
    mut type_0: COMPOUND_TYPE,
) -> core::ffi::c_int {
    return (type_0 as core::ffi::c_int == COMPOUND_WEDGE as core::ffi::c_int
        || type_0 as core::ffi::c_int == COMPOUND_DIFFWTD as core::ffi::c_int)
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
unsafe extern "C-unwind" fn has_second_ref(mut mbmi: *const MB_MODE_INFO) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
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
#[inline]
unsafe extern "C-unwind" fn is_cur_buf_hbd(mut xd: *const MACROBLOCKD) -> core::ffi::c_int {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn get_buf_by_bd(
    mut xd: *const MACROBLOCKD,
    mut buf16: *mut uint8_t,
) -> *mut uint8_t {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        (buf16 as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t
    } else {
        buf16
    };
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
unsafe extern "C-unwind" fn is_interintra_allowed_bsize(bsize: BLOCK_SIZE) -> core::ffi::c_int {
    return (bsize as core::ffi::c_int >= BLOCK_8X8 as core::ffi::c_int
        && bsize as core::ffi::c_int <= BLOCK_32X32 as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_mode(
    mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= SINGLE_INTER_MODE_START as core::ffi::c_int
        && (mode as core::ffi::c_int) < SINGLE_INTER_MODE_END as core::ffi::c_int)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed_ref(
    mut rf: *const MV_REFERENCE_FRAME,
) -> core::ffi::c_int {
    return (*rf.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
        && *rf.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            <= INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_allowed(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (is_interintra_allowed_bsize((*mbmi).bsize) != 0
        && is_interintra_allowed_mode((*mbmi).mode) != 0
        && is_interintra_allowed_ref(((*mbmi).ref_frame).as_ptr()) != 0)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_interintra_pred(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return ((*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
        > INTRA_FRAME as core::ffi::c_int
        && (*mbmi).ref_frame[1 as core::ffi::c_int as usize] as core::ffi::c_int
            == INTRA_FRAME as core::ffi::c_int && is_interintra_allowed(mbmi) != 0)
        as core::ffi::c_int;
}
static mut max_neighbor_obmc: [core::ffi::c_int; 6] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
#[inline]
unsafe extern "C-unwind" fn is_neighbor_overlappable(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return is_inter_block(mbmi);
}
pub const ROUND0_BITS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const COMPOUND_ROUND1_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_conv_params_no_round(
    mut cmp_index: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut dst: *mut CONV_BUF_TYPE,
    mut dst_stride: core::ffi::c_int,
    mut is_compound: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> ConvolveParams {
    let mut conv_params: ConvolveParams = ConvolveParams {
        do_average: 0,
        dst: 0 as *mut CONV_BUF_TYPE,
        dst_stride: 0,
        round_0: 0,
        round_1: 0,
        plane: 0,
        is_compound: 0,
        use_dist_wtd_comp_avg: 0,
        fwd_offset: 0,
        bck_offset: 0,
    };
    if cmp_index == 0 || is_compound != 0 {} else {
        __assert_fail(
            b"IMPLIES(cmp_index, is_compound)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/convolve.h\0" as *const u8
                as *const core::ffi::c_char,
            73 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 82],
                [core::ffi::c_char; 82],
            >(
                *b"ConvolveParams get_conv_params_no_round(int, int, CONV_BUF_TYPE *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    conv_params.is_compound = is_compound;
    conv_params.use_dist_wtd_comp_avg = 0 as core::ffi::c_int;
    conv_params.round_0 = ROUND0_BITS;
    conv_params.round_1 = if is_compound != 0 {
        COMPOUND_ROUND1_BITS
    } else {
        2 as core::ffi::c_int * FILTER_BITS - conv_params.round_0
    };
    let intbufrange: core::ffi::c_int = bd + FILTER_BITS - conv_params.round_0
        + 2 as core::ffi::c_int;
    if !(bd < 12 as core::ffi::c_int) || intbufrange <= 16 as core::ffi::c_int {} else {
        __assert_fail(
            b"IMPLIES(bd < 12, intbufrange <= 16)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/convolve.h\0" as *const u8
                as *const core::ffi::c_char,
            82 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 82],
                [core::ffi::c_char; 82],
            >(
                *b"ConvolveParams get_conv_params_no_round(int, int, CONV_BUF_TYPE *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if intbufrange > 16 as core::ffi::c_int {
        conv_params.round_0 += intbufrange - 16 as core::ffi::c_int;
        if is_compound == 0 {
            conv_params.round_1 -= intbufrange - 16 as core::ffi::c_int;
        }
    }
    conv_params.dst = dst;
    conv_params.dst_stride = dst_stride;
    conv_params.plane = plane;
    conv_params.do_average = cmp_index;
    return conv_params;
}
#[inline]
unsafe extern "C-unwind" fn get_conv_params(
    mut do_average: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> ConvolveParams {
    return get_conv_params_no_round(
        do_average,
        plane,
        0 as *mut CONV_BUF_TYPE,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        bd,
    );
}
pub const INVALID_IDX: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn enc_calc_subpel_params(
    src_mv: *const MV,
    inter_pred_params: *mut InterPredParams,
    mut pre: *mut *mut uint8_t,
    mut subpel_params: *mut SubpelParams,
    mut src_stride: *mut core::ffi::c_int,
) {
    let mut pre_buf: *mut buf_2d = &raw mut (*inter_pred_params).ref_frame_buf;
    init_subpel_params(
        src_mv,
        inter_pred_params,
        subpel_params,
        (*pre_buf).width,
        (*pre_buf).height,
    );
    *pre = ((*pre_buf).buf0)
        .offset(
            (((*subpel_params).pos_y >> SCALE_SUBPEL_BITS) * (*pre_buf).stride) as isize,
        )
        .offset(((*subpel_params).pos_x >> SCALE_SUBPEL_BITS) as isize);
    *src_stride = (*pre_buf).stride;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_enc_build_one_inter_predictor(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut src_mv: *const MV,
    mut inter_pred_params: *mut InterPredParams,
) {
    build_one_inter_predictor(dst, dst_stride, src_mv, inter_pred_params);
}
unsafe extern "C-unwind" fn enc_build_inter_predictors(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
) {
    build_inter_predictors(cm, xd, plane, mi, 0 as core::ffi::c_int, bw, bh, mi_x, mi_y);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_enc_build_inter_predictor_y(
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
) {
    let mi_x: core::ffi::c_int = mi_col * MI_SIZE;
    let mi_y: core::ffi::c_int = mi_row * MI_SIZE;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut inter_pred_params: InterPredParams = InterPredParams {
        mode: TRANSLATION_PRED,
        comp_mode: UNIFORM_SINGLE,
        warp_params: WarpedMotionParams {
            wmmat: [0; 6],
            alpha: 0,
            beta: 0,
            gamma: 0,
            delta: 0,
            wmtype: 0,
            invalid: 0,
        },
        conv_params: ConvolveParams {
            do_average: 0,
            dst: 0 as *mut CONV_BUF_TYPE,
            dst_stride: 0,
            round_0: 0,
            round_1: 0,
            plane: 0,
            is_compound: 0,
            use_dist_wtd_comp_avg: 0,
            fwd_offset: 0,
            bck_offset: 0,
        },
        interp_filter_params: [0 as *const InterpFilterParams; 2],
        block_width: 0,
        block_height: 0,
        pix_row: 0,
        pix_col: 0,
        ref_frame_buf: buf_2d {
            buf: 0 as *mut uint8_t,
            buf0: 0 as *mut uint8_t,
            width: 0,
            height: 0,
            stride: 0,
        },
        subsampling_x: 0,
        subsampling_y: 0,
        scale_factors: 0 as *const scale_factors,
        bit_depth: 0,
        use_hbd_buf: 0,
        mask_comp: INTERINTER_COMPOUND_DATA {
            seg_mask: 0 as *mut uint8_t,
            wedge_index: 0,
            wedge_sign: 0,
            mask_type: 0,
            type_0: 0,
        },
        sb_type: BLOCK_4X4,
        is_intrabc: 0,
        top: 0,
        left: 0,
    };
    let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
    let dst: *mut uint8_t = (*dst_buf).buf;
    let mv: MV = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
        .mv[0 as core::ffi::c_int as usize]
        .as_mv;
    let sf: *const scale_factors = (*xd)
        .block_ref_scale_factors[0 as core::ffi::c_int as usize];
    av1_init_inter_params(
        &raw mut inter_pred_params,
        (*pd).width as core::ffi::c_int,
        (*pd).height as core::ffi::c_int,
        mi_y,
        mi_x,
        (*pd).subsampling_x,
        (*pd).subsampling_y,
        (*xd).bd,
        is_cur_buf_hbd(xd),
        false_0,
        sf,
        ((*pd).pre).as_mut_ptr(),
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).interp_filters,
    );
    inter_pred_params.conv_params = get_conv_params_no_round(
        0 as core::ffi::c_int,
        AOM_PLANE_Y,
        (*xd).tmp_conv_dst,
        MAX_SB_SIZE,
        false_0,
        (*xd).bd,
    );
    inter_pred_params.conv_params.use_dist_wtd_comp_avg = 0 as core::ffi::c_int;
    av1_enc_build_one_inter_predictor(
        dst,
        (*dst_buf).stride,
        &mv,
        &raw mut inter_pred_params,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_enc_build_inter_predictor_y_nonrd(
    mut xd: *mut MACROBLOCKD,
    mut inter_pred_params: *mut InterPredParams,
    mut subpel_params: *const SubpelParams,
) {
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(AOM_PLANE_Y as isize) as *mut macroblockd_plane;
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
    let mut pre_buf: *const buf_2d = &raw mut *((*pd).pre)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut buf_2d;
    let mut src: *const uint8_t = ((*pre_buf).buf0)
        .offset(
            (((*subpel_params).pos_y >> SCALE_SUBPEL_BITS) * (*pre_buf).stride) as isize,
        )
        .offset(((*subpel_params).pos_x >> SCALE_SUBPEL_BITS) as isize);
    let dst: *mut uint8_t = (*dst_buf).buf;
    let mut src_stride: core::ffi::c_int = (*pre_buf).stride;
    let mut dst_stride: core::ffi::c_int = (*dst_buf).stride;
    (*inter_pred_params).ref_frame_buf = *pre_buf;
    init_interp_filter_params(
        ((*inter_pred_params).interp_filter_params).as_mut_ptr(),
        &(*mbmi).interp_filters.as_filters,
        (*pd).width as core::ffi::c_int,
        (*pd).height as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
    av1_make_inter_predictor(
        src,
        src_stride,
        dst,
        dst_stride,
        inter_pred_params,
        subpel_params,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_enc_build_inter_predictor(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut ctx: *const BUFFER_SET,
    mut bsize: BLOCK_SIZE,
    mut plane_from: core::ffi::c_int,
    mut plane_to: core::ffi::c_int,
) {
    let mut plane: core::ffi::c_int = plane_from;
    while plane <= plane_to {
        if plane != 0 && !(*xd).is_chroma_ref {
            break;
        }
        let mi_x: core::ffi::c_int = mi_col * MI_SIZE;
        let mi_y: core::ffi::c_int = mi_row * MI_SIZE;
        enc_build_inter_predictors(
            cm,
            xd,
            plane,
            *((*xd).mi).offset(0 as core::ffi::c_int as isize),
            (*xd).plane[plane as usize].width as core::ffi::c_int,
            (*xd).plane[plane as usize].height as core::ffi::c_int,
            mi_x,
            mi_y,
        );
        if is_interintra_pred(*((*xd).mi).offset(0 as core::ffi::c_int as isize)) != 0 {
            let mut default_ctx: BUFFER_SET = {
                let mut init = BUFFER_SET {
                    plane: [
                        (*xd).plane[0 as core::ffi::c_int as usize].dst.buf,
                        (*xd).plane[1 as core::ffi::c_int as usize].dst.buf,
                        (*xd).plane[2 as core::ffi::c_int as usize].dst.buf,
                    ],
                    stride: [
                        (*xd).plane[0 as core::ffi::c_int as usize].dst.stride,
                        (*xd).plane[1 as core::ffi::c_int as usize].dst.stride,
                        (*xd).plane[2 as core::ffi::c_int as usize].dst.stride,
                    ],
                };
                init
            };
            if ctx.is_null() {
                ctx = &raw mut default_ctx;
            }
            av1_build_interintra_predictor(
                cm,
                xd,
                (*xd).plane[plane as usize].dst.buf,
                (*xd).plane[plane as usize].dst.stride,
                ctx,
                plane,
                bsize,
            );
        }
        plane += 1;
    }
}
unsafe extern "C-unwind" fn setup_address_for_obmc(
    mut xd: *mut MACROBLOCKD,
    mut mi_row_offset: core::ffi::c_int,
    mut mi_col_offset: core::ffi::c_int,
    mut ref_mbmi: *mut MB_MODE_INFO,
    mut ctxt: *mut build_prediction_ctxt,
    num_planes: core::ffi::c_int,
) {
    let ref_bsize: BLOCK_SIZE = (if BLOCK_8X8 as core::ffi::c_int
        > (*ref_mbmi).bsize as core::ffi::c_int
    {
        BLOCK_8X8 as core::ffi::c_int
    } else {
        (*ref_mbmi).bsize as core::ffi::c_int
    }) as BLOCK_SIZE;
    let ref_mi_row: core::ffi::c_int = (*xd).mi_row + mi_row_offset;
    let ref_mi_col: core::ffi::c_int = (*xd).mi_col + mi_col_offset;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblockd_plane;
        setup_pred_plane(
            &raw mut (*pd).dst,
            ref_bsize,
            *((*ctxt).tmp_buf).offset(plane as isize),
            *((*ctxt).tmp_width).offset(plane as isize),
            *((*ctxt).tmp_height).offset(plane as isize),
            *((*ctxt).tmp_stride).offset(plane as isize),
            mi_row_offset,
            mi_col_offset,
            0 as *const scale_factors,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        );
        plane += 1;
    }
    let frame: MV_REFERENCE_FRAME = (*ref_mbmi)
        .ref_frame[0 as core::ffi::c_int as usize];
    let ref_buf: *const RefCntBuffer = get_ref_frame_buf((*ctxt).cm, frame);
    let sf: *const scale_factors = get_ref_scale_factors_const((*ctxt).cm, frame)
        as *const scale_factors;
    (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize] = sf;
    if av1_is_valid_scale(sf) == 0 {
        aom_internal_error(
            (*xd).error_info as *mut aom_internal_error_info,
            AOM_CODEC_UNSUP_BITSTREAM,
            b"Reference frame has invalid dimensions\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    av1_setup_pre_planes(
        xd,
        0 as core::ffi::c_int,
        &(*ref_buf).buf,
        ref_mi_row,
        ref_mi_col,
        sf,
        num_planes,
    );
}
#[inline]
unsafe extern "C-unwind" fn build_obmc_prediction(
    mut xd: *mut MACROBLOCKD,
    mut rel_mi_row: core::ffi::c_int,
    mut rel_mi_col: core::ffi::c_int,
    mut op_mi_size: uint8_t,
    mut dir: core::ffi::c_int,
    mut above_mbmi: *mut MB_MODE_INFO,
    mut fun_ctxt: *mut core::ffi::c_void,
    num_planes: core::ffi::c_int,
) {
    let mut ctxt: *mut build_prediction_ctxt = fun_ctxt as *mut build_prediction_ctxt;
    setup_address_for_obmc(xd, rel_mi_row, rel_mi_col, above_mbmi, ctxt, num_planes);
    let mi_x: core::ffi::c_int = (*xd).mi_col + rel_mi_col << MI_SIZE_LOG2;
    let mi_y: core::ffi::c_int = (*xd).mi_row + rel_mi_row << MI_SIZE_LOG2;
    let bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize;
    let mut inter_pred_params: InterPredParams = InterPredParams {
        mode: TRANSLATION_PRED,
        comp_mode: UNIFORM_SINGLE,
        warp_params: WarpedMotionParams {
            wmmat: [0; 6],
            alpha: 0,
            beta: 0,
            gamma: 0,
            delta: 0,
            wmtype: 0,
            invalid: 0,
        },
        conv_params: ConvolveParams {
            do_average: 0,
            dst: 0 as *mut CONV_BUF_TYPE,
            dst_stride: 0,
            round_0: 0,
            round_1: 0,
            plane: 0,
            is_compound: 0,
            use_dist_wtd_comp_avg: 0,
            fwd_offset: 0,
            bck_offset: 0,
        },
        interp_filter_params: [0 as *const InterpFilterParams; 2],
        block_width: 0,
        block_height: 0,
        pix_row: 0,
        pix_col: 0,
        ref_frame_buf: buf_2d {
            buf: 0 as *mut uint8_t,
            buf0: 0 as *mut uint8_t,
            width: 0,
            height: 0,
            stride: 0,
        },
        subsampling_x: 0,
        subsampling_y: 0,
        scale_factors: 0 as *const scale_factors,
        bit_depth: 0,
        use_hbd_buf: 0,
        mask_comp: INTERINTER_COMPOUND_DATA {
            seg_mask: 0 as *mut uint8_t,
            wedge_index: 0,
            wedge_sign: 0,
            mask_type: 0,
            type_0: 0,
        },
        sb_type: BLOCK_4X4,
        is_intrabc: 0,
        top: 0,
        left: 0,
    };
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    while j < num_planes {
        let mut pd: *const macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(j as isize) as *mut macroblockd_plane;
        let mut bw: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut bh: core::ffi::c_int = 0 as core::ffi::c_int;
        if dir != 0 {
            bw = clamp(
                block_size_wide[bsize as usize] as core::ffi::c_int
                    >> (*pd).subsampling_x + 1 as core::ffi::c_int,
                4 as core::ffi::c_int,
                block_size_wide[BLOCK_64X64 as core::ffi::c_int as usize]
                    as core::ffi::c_int >> (*pd).subsampling_x + 1 as core::ffi::c_int,
            );
            bh = (op_mi_size as core::ffi::c_int) << MI_SIZE_LOG2 >> (*pd).subsampling_y;
        } else {
            bw = op_mi_size as core::ffi::c_int * MI_SIZE >> (*pd).subsampling_x;
            bh = clamp(
                block_size_high[bsize as usize] as core::ffi::c_int
                    >> (*pd).subsampling_y + 1 as core::ffi::c_int,
                4 as core::ffi::c_int,
                block_size_high[BLOCK_64X64 as core::ffi::c_int as usize]
                    as core::ffi::c_int >> (*pd).subsampling_y + 1 as core::ffi::c_int,
            );
        }
        if !(av1_skip_u4x4_pred_in_obmc(bsize, pd, dir) != 0) {
            let pre_buf: *const buf_2d = &*((*pd).pre)
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize) as *const buf_2d;
            let mv: MV = (*above_mbmi).mv[0 as core::ffi::c_int as usize].as_mv;
            av1_init_inter_params(
                &raw mut inter_pred_params,
                bw,
                bh,
                mi_y >> (*pd).subsampling_y,
                mi_x >> (*pd).subsampling_x,
                (*pd).subsampling_x,
                (*pd).subsampling_y,
                (*xd).bd,
                is_cur_buf_hbd(xd),
                0 as core::ffi::c_int,
                (*xd).block_ref_scale_factors[0 as core::ffi::c_int as usize],
                pre_buf,
                (*above_mbmi).interp_filters,
            );
            inter_pred_params.conv_params = get_conv_params(
                0 as core::ffi::c_int,
                j,
                (*xd).bd,
            );
            av1_enc_build_one_inter_predictor(
                (*pd).dst.buf,
                (*pd).dst.stride,
                &mv,
                &raw mut inter_pred_params,
            );
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_build_prediction_by_above_preds(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut tmp_buf: *mut *mut uint8_t,
    mut tmp_width: *mut core::ffi::c_int,
    mut tmp_height: *mut core::ffi::c_int,
    mut tmp_stride: *mut core::ffi::c_int,
) {
    if !(*xd).up_available {
        return;
    }
    let mut ctxt: build_prediction_ctxt = {
        let mut init = build_prediction_ctxt {
            cm: cm,
            tmp_buf: tmp_buf as *mut *mut uint8_t,
            tmp_width: tmp_width as *mut core::ffi::c_int,
            tmp_height: tmp_height as *mut core::ffi::c_int,
            tmp_stride: tmp_stride as *mut core::ffi::c_int,
            mb_to_far_edge: (*xd).mb_to_right_edge,
            dcb: NULL,
        };
        init
    };
    let mut bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
        .bsize;
    foreach_overlappable_nb_above(
        cm,
        xd,
        max_neighbor_obmc[mi_size_wide_log2[bsize as usize] as usize],
        Some(
            build_obmc_prediction
                as unsafe extern "C-unwind" fn(
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    uint8_t,
                    core::ffi::c_int,
                    *mut MB_MODE_INFO,
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                ) -> (),
        ),
        &raw mut ctxt as *mut build_prediction_ctxt as *mut core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_build_prediction_by_left_preds(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut tmp_buf: *mut *mut uint8_t,
    mut tmp_width: *mut core::ffi::c_int,
    mut tmp_height: *mut core::ffi::c_int,
    mut tmp_stride: *mut core::ffi::c_int,
) {
    if !(*xd).left_available {
        return;
    }
    let mut ctxt: build_prediction_ctxt = {
        let mut init = build_prediction_ctxt {
            cm: cm,
            tmp_buf: tmp_buf as *mut *mut uint8_t,
            tmp_width: tmp_width as *mut core::ffi::c_int,
            tmp_height: tmp_height as *mut core::ffi::c_int,
            tmp_stride: tmp_stride as *mut core::ffi::c_int,
            mb_to_far_edge: (*xd).mb_to_bottom_edge,
            dcb: NULL,
        };
        init
    };
    let mut bsize: BLOCK_SIZE = (**((*xd).mi).offset(0 as core::ffi::c_int as isize))
        .bsize;
    foreach_overlappable_nb_left(
        cm,
        xd,
        max_neighbor_obmc[mi_size_high_log2[bsize as usize] as usize],
        Some(
            build_obmc_prediction
                as unsafe extern "C-unwind" fn(
                    *mut MACROBLOCKD,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    uint8_t,
                    core::ffi::c_int,
                    *mut MB_MODE_INFO,
                    *mut core::ffi::c_void,
                    core::ffi::c_int,
                ) -> (),
        ),
        &raw mut ctxt as *mut build_prediction_ctxt as *mut core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_build_obmc_inter_predictors_sb(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut dst_buf1: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut dst_buf2: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut dst_stride1: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_stride2: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_width1: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_width2: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_height1: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    let mut dst_height2: [core::ffi::c_int; 3] = [MAX_SB_SIZE, MAX_SB_SIZE, MAX_SB_SIZE];
    av1_setup_obmc_dst_bufs(xd, dst_buf1.as_mut_ptr(), dst_buf2.as_mut_ptr());
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    av1_build_prediction_by_above_preds(
        cm,
        xd,
        dst_buf1.as_mut_ptr(),
        dst_width1.as_mut_ptr(),
        dst_height1.as_mut_ptr(),
        dst_stride1.as_mut_ptr(),
    );
    av1_build_prediction_by_left_preds(
        cm,
        xd,
        dst_buf2.as_mut_ptr(),
        dst_width2.as_mut_ptr(),
        dst_height2.as_mut_ptr(),
        dst_stride2.as_mut_ptr(),
    );
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        (**((*xd).mi).offset(0 as core::ffi::c_int as isize)).bsize,
        &raw mut (*(*cm).cur_frame).buf,
        mi_row,
        mi_col,
        0 as core::ffi::c_int,
        num_planes,
    );
    av1_build_obmc_inter_prediction(
        cm,
        xd,
        dst_buf1.as_mut_ptr(),
        dst_stride1.as_mut_ptr(),
        dst_buf2.as_mut_ptr(),
        dst_stride2.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_build_inter_predictors_for_planes_single_buf(
    mut xd: *mut MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane_from: core::ffi::c_int,
    mut plane_to: core::ffi::c_int,
    mut ref_0: core::ffi::c_int,
    mut ext_dst: *mut *mut uint8_t,
    mut ext_dst_stride: *mut core::ffi::c_int,
) {
    if (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int {} else {
        __assert_fail(
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/reconinter_enc.c\0" as *const u8
                as *const core::ffi::c_char,
            274 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 115],
                [core::ffi::c_char; 115],
            >(
                *b"void av1_build_inter_predictors_for_planes_single_buf(MACROBLOCKD *, BLOCK_SIZE, int, int, int, uint8_t **, int *)\0",
            ))
                .as_ptr(),
        );
    };
    let mut mi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mi_x: core::ffi::c_int = mi_col * MI_SIZE;
    let mi_y: core::ffi::c_int = mi_row * MI_SIZE;
    let mut warp_types: WarpTypesAllowed = WarpTypesAllowed {
        global_warp_allowed: 0,
        local_warp_allowed: 0,
    };
    let wm: *const WarpedMotionParams = &*((*xd).global_motion)
        .offset(*((*mi).ref_frame).as_ptr().offset(ref_0 as isize) as isize)
        as *const WarpedMotionParams;
    warp_types.global_warp_allowed = is_global_mv_block(mi, (*wm).wmtype);
    warp_types.local_warp_allowed = ((*mi).motion_mode as core::ffi::c_int
        == WARPED_CAUSAL as core::ffi::c_int) as core::ffi::c_int;
    let mut plane: core::ffi::c_int = plane_from;
    while plane <= plane_to {
        let mut pd: *const macroblockd_plane = &raw mut *((*xd).plane)
            .as_mut_ptr()
            .offset(plane as isize) as *mut macroblockd_plane;
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        let bw: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let bh: core::ffi::c_int = block_size_high[plane_bsize as usize]
            as core::ffi::c_int;
        let mut inter_pred_params: InterPredParams = InterPredParams {
            mode: TRANSLATION_PRED,
            comp_mode: UNIFORM_SINGLE,
            warp_params: WarpedMotionParams {
                wmmat: [0; 6],
                alpha: 0,
                beta: 0,
                gamma: 0,
                delta: 0,
                wmtype: 0,
                invalid: 0,
            },
            conv_params: ConvolveParams {
                do_average: 0,
                dst: 0 as *mut CONV_BUF_TYPE,
                dst_stride: 0,
                round_0: 0,
                round_1: 0,
                plane: 0,
                is_compound: 0,
                use_dist_wtd_comp_avg: 0,
                fwd_offset: 0,
                bck_offset: 0,
            },
            interp_filter_params: [0 as *const InterpFilterParams; 2],
            block_width: 0,
            block_height: 0,
            pix_row: 0,
            pix_col: 0,
            ref_frame_buf: buf_2d {
                buf: 0 as *mut uint8_t,
                buf0: 0 as *mut uint8_t,
                width: 0,
                height: 0,
                stride: 0,
            },
            subsampling_x: 0,
            subsampling_y: 0,
            scale_factors: 0 as *const scale_factors,
            bit_depth: 0,
            use_hbd_buf: 0,
            mask_comp: INTERINTER_COMPOUND_DATA {
                seg_mask: 0 as *mut uint8_t,
                wedge_index: 0,
                wedge_sign: 0,
                mask_type: 0,
                type_0: 0,
            },
            sb_type: BLOCK_4X4,
            is_intrabc: 0,
            top: 0,
            left: 0,
        };
        av1_init_inter_params(
            &raw mut inter_pred_params,
            bw,
            bh,
            mi_y >> (*pd).subsampling_y,
            mi_x >> (*pd).subsampling_x,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
            (*xd).bd,
            is_cur_buf_hbd(xd),
            0 as core::ffi::c_int,
            (*xd).block_ref_scale_factors[ref_0 as usize],
            &*((*pd).pre).as_ptr().offset(ref_0 as isize),
            (*mi).interp_filters,
        );
        inter_pred_params.conv_params = get_conv_params(
            0 as core::ffi::c_int,
            plane,
            (*xd).bd,
        );
        av1_init_warp_params(&raw mut inter_pred_params, &raw mut warp_types, ref_0, xd, mi);
        let dst: *mut uint8_t = get_buf_by_bd(xd, *ext_dst.offset(plane as isize))
            as *mut uint8_t;
        let mv: MV = (*mi).mv[ref_0 as usize].as_mv;
        av1_enc_build_one_inter_predictor(
            dst,
            *ext_dst_stride.offset(plane as isize),
            &mv,
            &raw mut inter_pred_params,
        );
        plane += 1;
    }
}
unsafe extern "C-unwind" fn build_masked_compound(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut src0: *const uint8_t,
    mut src0_stride: core::ffi::c_int,
    mut src1: *const uint8_t,
    mut src1_stride: core::ffi::c_int,
    comp_data: *const INTERINTER_COMPOUND_DATA,
    mut sb_type: BLOCK_SIZE,
    mut h: core::ffi::c_int,
    mut w: core::ffi::c_int,
) {
    let subh: core::ffi::c_int = ((2 as core::ffi::c_int)
        << mi_size_high_log2[sb_type as usize] as core::ffi::c_int == h)
        as core::ffi::c_int;
    let subw: core::ffi::c_int = ((2 as core::ffi::c_int)
        << mi_size_wide_log2[sb_type as usize] as core::ffi::c_int == w)
        as core::ffi::c_int;
    let mut mask: *const uint8_t = av1_get_compound_type_mask(comp_data, sb_type);
    aom_blend_a64_mask_c(
        dst,
        dst_stride as uint32_t,
        src0,
        src0_stride as uint32_t,
        src1,
        src1_stride as uint32_t,
        mask,
        block_size_wide[sb_type as usize] as uint32_t,
        w,
        h,
        subw,
        subh,
    );
}
unsafe extern "C-unwind" fn build_masked_compound_highbd(
    mut dst_8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut src0_8: *const uint8_t,
    mut src0_stride: core::ffi::c_int,
    mut src1_8: *const uint8_t,
    mut src1_stride: core::ffi::c_int,
    comp_data: *const INTERINTER_COMPOUND_DATA,
    mut sb_type: BLOCK_SIZE,
    mut h: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let subh: core::ffi::c_int = ((2 as core::ffi::c_int)
        << mi_size_high_log2[sb_type as usize] as core::ffi::c_int == h)
        as core::ffi::c_int;
    let subw: core::ffi::c_int = ((2 as core::ffi::c_int)
        << mi_size_wide_log2[sb_type as usize] as core::ffi::c_int == w)
        as core::ffi::c_int;
    let mut mask: *const uint8_t = av1_get_compound_type_mask(comp_data, sb_type);
    aom_highbd_blend_a64_mask_c(
        dst_8,
        dst_stride as uint32_t,
        src0_8,
        src0_stride as uint32_t,
        src1_8,
        src1_stride as uint32_t,
        mask,
        block_size_wide[sb_type as usize] as uint32_t,
        w,
        h,
        subw,
        subh,
        bd,
    );
}
unsafe extern "C-unwind" fn build_wedge_inter_predictor_from_buf(
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut ext_dst0: *mut uint8_t,
    mut ext_dst_stride0: core::ffi::c_int,
    mut ext_dst1: *mut uint8_t,
    mut ext_dst_stride1: core::ffi::c_int,
) {
    let mbmi: *mut MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let is_compound: core::ffi::c_int = has_second_ref(mbmi) as core::ffi::c_int;
    let pd: *mut MACROBLOCKD_PLANE = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut MACROBLOCKD_PLANE;
    let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
    let dst: *mut uint8_t = ((*dst_buf).buf)
        .offset(((*dst_buf).stride * y) as isize)
        .offset(x as isize);
    (*mbmi).interinter_comp.seg_mask = (*xd).seg_mask;
    let mut comp_data: *const INTERINTER_COMPOUND_DATA = &raw mut (*mbmi).interinter_comp;
    let is_hbd: core::ffi::c_int = is_cur_buf_hbd(xd) as core::ffi::c_int;
    if is_compound != 0 && is_masked_compound_type((*comp_data).type_0) != 0 {
        if plane == 0
            && (*comp_data).type_0 as core::ffi::c_int
                == COMPOUND_DIFFWTD as core::ffi::c_int
        {
            if is_hbd != 0 {
                av1_build_compound_diffwtd_mask_highbd_c(
                    (*comp_data).seg_mask,
                    (*comp_data).mask_type,
                    (ext_dst0 as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                    ext_dst_stride0,
                    (ext_dst1 as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                    ext_dst_stride1,
                    h,
                    w,
                    (*xd).bd,
                );
            } else {
                av1_build_compound_diffwtd_mask_c(
                    (*comp_data).seg_mask,
                    (*comp_data).mask_type,
                    ext_dst0,
                    ext_dst_stride0,
                    ext_dst1,
                    ext_dst_stride1,
                    h,
                    w,
                );
            }
        }
        if is_hbd != 0 {
            build_masked_compound_highbd(
                dst,
                (*dst_buf).stride,
                (ext_dst0 as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                ext_dst_stride0,
                (ext_dst1 as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
                ext_dst_stride1,
                comp_data,
                (*mbmi).bsize,
                h,
                w,
                (*xd).bd,
            );
        } else {
            build_masked_compound(
                dst,
                (*dst_buf).stride,
                ext_dst0,
                ext_dst_stride0,
                ext_dst1,
                ext_dst_stride1,
                comp_data,
                (*mbmi).bsize,
                h,
                w,
            );
        }
    } else if is_hbd != 0 {
        aom_highbd_convolve_copy_c(
            ((ext_dst0 as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            ext_dst_stride0 as ptrdiff_t,
            ((dst as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t,
            (*dst_buf).stride as ptrdiff_t,
            w,
            h,
        );
    } else {
        aom_convolve_copy_c(
            ext_dst0,
            ext_dst_stride0 as ptrdiff_t,
            dst,
            (*dst_buf).stride as ptrdiff_t,
            w,
            h,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_build_wedge_inter_predictor_from_buf(
    mut xd: *mut MACROBLOCKD,
    mut bsize: BLOCK_SIZE,
    mut plane_from: core::ffi::c_int,
    mut plane_to: core::ffi::c_int,
    mut ext_dst0: *mut *mut uint8_t,
    mut ext_dst_stride0: *mut core::ffi::c_int,
    mut ext_dst1: *mut *mut uint8_t,
    mut ext_dst_stride1: *mut core::ffi::c_int,
) {
    let mut plane: core::ffi::c_int = 0;
    if (bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int {} else {
        __assert_fail(
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/reconinter_enc.c\0" as *const u8
                as *const core::ffi::c_char,
            412 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 121],
                [core::ffi::c_char; 121],
            >(
                *b"void av1_build_wedge_inter_predictor_from_buf(MACROBLOCKD *, BLOCK_SIZE, int, int, uint8_t **, int *, uint8_t **, int *)\0",
            ))
                .as_ptr(),
        );
    };
    plane = plane_from;
    while plane <= plane_to {
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            bsize,
            (*xd).plane[plane as usize].subsampling_x,
            (*xd).plane[plane as usize].subsampling_y,
        ) as BLOCK_SIZE;
        let bw: core::ffi::c_int = block_size_wide[plane_bsize as usize]
            as core::ffi::c_int;
        let bh: core::ffi::c_int = block_size_high[plane_bsize as usize]
            as core::ffi::c_int;
        build_wedge_inter_predictor_from_buf(
            xd,
            plane,
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
            bw,
            bh,
            *ext_dst0.offset(plane as isize),
            *ext_dst_stride0.offset(plane as isize),
            *ext_dst1.offset(plane as isize),
            *ext_dst_stride1.offset(plane as isize),
        );
        plane += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_upsampled_pred_scaled(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> bool {
    if !xd.is_null() {
        let mut mi: *const MB_MODE_INFO = *((*xd).mi)
            .offset(0 as core::ffi::c_int as isize);
        let ref_num: core::ffi::c_int = 0 as core::ffi::c_int;
        let is_intrabc: core::ffi::c_int = is_intrabc_block(mi) as core::ffi::c_int;
        let sf: *const scale_factors = if is_intrabc != 0 {
            &(*cm).sf_identity
        } else {
            (*xd).block_ref_scale_factors[ref_num as usize]
        };
        let is_scaled: core::ffi::c_int = av1_is_scaled(sf) as core::ffi::c_int;
        if is_scaled != 0 {
            let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
            let mi_x: core::ffi::c_int = mi_col * MI_SIZE;
            let mi_y: core::ffi::c_int = mi_row * MI_SIZE;
            let pd: *const macroblockd_plane = &raw mut *((*xd).plane)
                .as_mut_ptr()
                .offset(plane as isize) as *mut macroblockd_plane;
            let dst_buf: *const buf_2d = &(*pd).dst;
            let pre_buf: *const buf_2d = if is_intrabc != 0 {
                dst_buf
            } else {
                &*((*pd).pre).as_ptr().offset(ref_num as isize) as *const buf_2d
            };
            let mut inter_pred_params: InterPredParams = InterPredParams {
                mode: TRANSLATION_PRED,
                comp_mode: UNIFORM_SINGLE,
                warp_params: WarpedMotionParams {
                    wmmat: [0; 6],
                    alpha: 0,
                    beta: 0,
                    gamma: 0,
                    delta: 0,
                    wmtype: 0,
                    invalid: 0,
                },
                conv_params: ConvolveParams {
                    do_average: 0,
                    dst: 0 as *mut CONV_BUF_TYPE,
                    dst_stride: 0,
                    round_0: 0,
                    round_1: 0,
                    plane: 0,
                    is_compound: 0,
                    use_dist_wtd_comp_avg: 0,
                    fwd_offset: 0,
                    bck_offset: 0,
                },
                interp_filter_params: [0 as *const InterpFilterParams; 2],
                block_width: 0,
                block_height: 0,
                pix_row: 0,
                pix_col: 0,
                ref_frame_buf: buf_2d {
                    buf: 0 as *mut uint8_t,
                    buf0: 0 as *mut uint8_t,
                    width: 0,
                    height: 0,
                    stride: 0,
                },
                subsampling_x: 0,
                subsampling_y: 0,
                scale_factors: 0 as *const scale_factors,
                bit_depth: 0,
                use_hbd_buf: 0,
                mask_comp: INTERINTER_COMPOUND_DATA {
                    seg_mask: 0 as *mut uint8_t,
                    wedge_index: 0,
                    wedge_sign: 0,
                    mask_type: 0,
                    type_0: 0,
                },
                sb_type: BLOCK_4X4,
                is_intrabc: 0,
                top: 0,
                left: 0,
            };
            inter_pred_params.conv_params = get_conv_params(
                0 as core::ffi::c_int,
                plane,
                (*xd).bd,
            );
            let filters: int_interpfilters = av1_broadcast_interp_filter(
                EIGHTTAP_REGULAR,
            ) as int_interpfilters;
            av1_init_inter_params(
                &raw mut inter_pred_params,
                width,
                height,
                mi_y >> (*pd).subsampling_y,
                mi_x >> (*pd).subsampling_x,
                (*pd).subsampling_x,
                (*pd).subsampling_y,
                (*xd).bd,
                is_cur_buf_hbd(xd),
                is_intrabc,
                sf,
                pre_buf,
                filters,
            );
            av1_enc_build_one_inter_predictor(
                comp_pred,
                width,
                mv,
                &raw mut inter_pred_params,
            );
            return true_0 != 0;
        }
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_upsampled_pred_c(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subpel_x_q3: core::ffi::c_int,
    mut subpel_y_q3: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut subpel_search: core::ffi::c_int,
) {
    if aom_upsampled_pred_scaled(xd, cm, mi_row, mi_col, mv, comp_pred, width, height) {
        return;
    }
    let mut filter: *const InterpFilterParams = av1_get_filter(subpel_search);
    if subpel_x_q3 == 0 && subpel_y_q3 == 0 {
        aom_convolve_copy_c(
            ref_0,
            ref_stride as ptrdiff_t,
            comp_pred,
            width as ptrdiff_t,
            width,
            height,
        );
    } else if subpel_y_q3 == 0 {
        let kernel: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_x_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        aom_convolve8_horiz_c(
            ref_0,
            ref_stride as ptrdiff_t,
            comp_pred,
            width as ptrdiff_t,
            kernel,
            16 as core::ffi::c_int,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            width,
            height,
        );
    } else if subpel_x_q3 == 0 {
        let kernel_0: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_y_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        aom_convolve8_vert_c(
            ref_0,
            ref_stride as ptrdiff_t,
            comp_pred,
            width as ptrdiff_t,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            kernel_0,
            16 as core::ffi::c_int,
            width,
            height,
        );
    } else {
        let mut temp: *mut uint8_t = comp_pred;
        let kernel_x: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_x_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        let kernel_y: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_y_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        let intermediate_height: core::ffi::c_int = ((height - 1 as core::ffi::c_int)
            * 8 as core::ffi::c_int + subpel_y_q3 >> 3 as core::ffi::c_int)
            + (*filter).taps as core::ffi::c_int;
        if intermediate_height
            <= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 16 as core::ffi::c_int + 16 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"intermediate_height <= (MAX_SB_SIZE * 2 + 16) + 16\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/reconinter_enc.c\0" as *const u8
                    as *const core::ffi::c_char,
                494 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 151],
                    [core::ffi::c_char; 151],
                >(
                    *b"void aom_upsampled_pred_c(MACROBLOCKD *, const AV1_COMMON *const, int, int, const MV *const, uint8_t *, int, int, int, int, const uint8_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        aom_convolve8_horiz_c(
            ref_0
                .offset(
                    -((ref_stride
                        * (((*filter).taps as core::ffi::c_int >> 1 as core::ffi::c_int)
                            - 1 as core::ffi::c_int)) as isize),
                ),
            ref_stride as ptrdiff_t,
            temp,
            MAX_SB_SIZE as ptrdiff_t,
            kernel_x,
            16 as core::ffi::c_int,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            width,
            intermediate_height,
        );
        aom_convolve8_vert_c(
            temp
                .offset(
                    (MAX_SB_SIZE
                        * (((*filter).taps as core::ffi::c_int >> 1 as core::ffi::c_int)
                            - 1 as core::ffi::c_int)) as isize,
                ),
            MAX_SB_SIZE as ptrdiff_t,
            comp_pred,
            width as ptrdiff_t,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            kernel_y,
            16 as core::ffi::c_int,
            width,
            height,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_comp_avg_upsampled_pred_c(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred: *mut uint8_t,
    mut pred: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subpel_x_q3: core::ffi::c_int,
    mut subpel_y_q3: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut subpel_search: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    aom_upsampled_pred_c(
        xd,
        cm,
        mi_row,
        mi_col,
        mv,
        comp_pred,
        width,
        height,
        subpel_x_q3,
        subpel_y_q3,
        ref_0,
        ref_stride,
        subpel_search,
    );
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            *comp_pred.offset(j as isize) = (*comp_pred.offset(j as isize)
                as core::ffi::c_int + *pred.offset(j as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int) as uint8_t;
            j += 1;
        }
        comp_pred = comp_pred.offset(width as isize);
        pred = pred.offset(width as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_comp_mask_upsampled_pred(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1_COMMON,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred: *mut uint8_t,
    mut pred: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subpel_x_q3: core::ffi::c_int,
    mut subpel_y_q3: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut subpel_search: core::ffi::c_int,
) {
    aom_upsampled_pred_c(
        xd,
        cm,
        mi_row,
        mi_col,
        mv,
        comp_pred,
        width,
        height,
        subpel_x_q3,
        subpel_y_q3,
        ref_0,
        ref_stride,
        subpel_search,
    );
    aom_comp_mask_pred_c(
        comp_pred,
        pred,
        width,
        height,
        comp_pred,
        width,
        mask,
        mask_stride,
        invert_mask,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_upsampled_pred_c(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1Common,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred8: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subpel_x_q3: core::ffi::c_int,
    mut subpel_y_q3: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut subpel_search: core::ffi::c_int,
) {
    if aom_upsampled_pred_scaled(
        xd,
        cm as *const AV1_COMMON,
        mi_row,
        mi_col,
        mv,
        comp_pred8,
        width,
        height,
    ) {
        return;
    }
    let mut filter: *const InterpFilterParams = av1_get_filter(subpel_search);
    if subpel_x_q3 == 0 && subpel_y_q3 == 0 {
        let mut ref_0: *const uint16_t = ((ref8 as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut comp_pred: *mut uint16_t = ((comp_pred8 as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        aom_highbd_convolve_copy_c(
            ref_0,
            ref_stride as ptrdiff_t,
            comp_pred,
            width as ptrdiff_t,
            width,
            height,
        );
    } else if subpel_y_q3 == 0 {
        let kernel: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_x_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        aom_highbd_convolve8_horiz_c(
            ref8,
            ref_stride as ptrdiff_t,
            comp_pred8,
            width as ptrdiff_t,
            kernel,
            16 as core::ffi::c_int,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            width,
            height,
            bd,
        );
    } else if subpel_x_q3 == 0 {
        let kernel_0: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_y_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        aom_highbd_convolve8_vert_c(
            ref8,
            ref_stride as ptrdiff_t,
            comp_pred8,
            width as ptrdiff_t,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            kernel_0,
            16 as core::ffi::c_int,
            width,
            height,
            bd,
        );
    } else {
        let mut temp: *mut uint16_t = ((comp_pred8 as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        let mut ref_1: *const uint16_t = ((ref8 as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let kernel_x: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_x_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        let kernel_y: *const int16_t = av1_get_interp_filter_subpel_kernel(
            filter,
            subpel_y_q3 << 1 as core::ffi::c_int,
        ) as *const int16_t;
        let intermediate_height: core::ffi::c_int = ((height - 1 as core::ffi::c_int)
            * 8 as core::ffi::c_int + subpel_y_q3 >> 3 as core::ffi::c_int)
            + (*filter).taps as core::ffi::c_int;
        if intermediate_height
            <= ((1 as core::ffi::c_int) << 7 as core::ffi::c_int) * 2 as core::ffi::c_int
                + 16 as core::ffi::c_int + 16 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"intermediate_height <= (MAX_SB_SIZE * 2 + 16) + 16\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/reconinter_enc.c\0" as *const u8
                    as *const core::ffi::c_char,
                577 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 169],
                    [core::ffi::c_char; 169],
                >(
                    *b"void aom_highbd_upsampled_pred_c(MACROBLOCKD *, const struct AV1Common *const, int, int, const MV *const, uint8_t *, int, int, int, int, const uint8_t *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        aom_highbd_convolve8_horiz_c(
            (ref_1
                .offset(
                    -((ref_stride
                        * (((*filter).taps as core::ffi::c_int >> 1 as core::ffi::c_int)
                            - 1 as core::ffi::c_int)) as isize),
                ) as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
            ref_stride as ptrdiff_t,
            (temp as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
            MAX_SB_SIZE as ptrdiff_t,
            kernel_x,
            16 as core::ffi::c_int,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            width,
            intermediate_height,
            bd,
        );
        aom_highbd_convolve8_vert_c(
            (temp
                .offset(
                    (((1 as core::ffi::c_int) << 7 as core::ffi::c_int)
                        * (((*filter).taps as core::ffi::c_int >> 1 as core::ffi::c_int)
                            - 1 as core::ffi::c_int)) as isize,
                ) as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
            MAX_SB_SIZE as ptrdiff_t,
            comp_pred8,
            width as ptrdiff_t,
            0 as *const int16_t,
            -(1 as core::ffi::c_int),
            kernel_y,
            16 as core::ffi::c_int,
            width,
            height,
            bd,
        );
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_comp_avg_upsampled_pred_c(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1Common,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred8: *mut uint8_t,
    mut pred8: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subpel_x_q3: core::ffi::c_int,
    mut subpel_y_q3: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut subpel_search: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut pred: *const uint16_t = ((pred8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut comp_pred: *mut uint16_t = ((comp_pred8 as uintptr_t)
        << 1 as core::ffi::c_int) as *mut uint16_t;
    aom_highbd_upsampled_pred_c(
        xd,
        cm,
        mi_row,
        mi_col,
        mv,
        comp_pred8,
        width,
        height,
        subpel_x_q3,
        subpel_y_q3,
        ref8,
        ref_stride,
        bd,
        subpel_search,
    );
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            *comp_pred.offset(j as isize) = (*pred.offset(j as isize) as core::ffi::c_int
                + *comp_pred.offset(j as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int) as uint16_t;
            j += 1;
        }
        comp_pred = comp_pred.offset(width as isize);
        pred = pred.offset(width as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_comp_mask_upsampled_pred(
    mut xd: *mut MACROBLOCKD,
    cm: *const AV1Common,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mv: *const MV,
    mut comp_pred8: *mut uint8_t,
    mut pred8: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subpel_x_q3: core::ffi::c_int,
    mut subpel_y_q3: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut bd: core::ffi::c_int,
    mut subpel_search: core::ffi::c_int,
) {
    aom_highbd_upsampled_pred_c(
        xd,
        cm,
        mi_row,
        mi_col,
        mv,
        comp_pred8,
        width,
        height,
        subpel_x_q3,
        subpel_y_q3,
        ref8,
        ref_stride,
        bd,
        subpel_search,
    );
    aom_highbd_comp_mask_pred_c(
        comp_pred8,
        pred8,
        width,
        height,
        comp_pred8,
        width,
        mask,
        mask_stride,
        invert_mask,
    );
}
pub const FILTER_BITS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const SUBPEL_BITS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const SUBPEL_TAPS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SCALE_SUBPEL_BITS: core::ffi::c_int = 10 as core::ffi::c_int;
pub const SCALE_SUBPEL_SHIFTS: core::ffi::c_int = (1 as core::ffi::c_int)
    << SCALE_SUBPEL_BITS;
pub const SCALE_SUBPEL_MASK: core::ffi::c_int = SCALE_SUBPEL_SHIFTS
    - 1 as core::ffi::c_int;
pub const SCALE_EXTRA_BITS: core::ffi::c_int = SCALE_SUBPEL_BITS - SUBPEL_BITS;
pub const SCALE_EXTRA_OFF: core::ffi::c_int = ((1 as core::ffi::c_int)
    << SCALE_EXTRA_BITS) / 2 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn foreach_overlappable_nb_above(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut nb_max: core::ffi::c_int,
    mut fun: overlappable_nb_visitor_t,
    mut fun_ctxt: *mut core::ffi::c_void,
) {
    if !(*xd).up_available {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut nb_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_col: core::ffi::c_int = (*xd).mi_col;
    let mut prev_row_mi: *mut *mut MB_MODE_INFO = ((*xd).mi)
        .offset(-(mi_col as isize))
        .offset(-((1 as core::ffi::c_int * (*xd).mi_stride) as isize));
    let end_col: core::ffi::c_int = if (mi_col + (*xd).width as core::ffi::c_int)
        < (*cm).mi_params.mi_cols
    {
        mi_col + (*xd).width as core::ffi::c_int
    } else {
        (*cm).mi_params.mi_cols
    };
    let mut mi_step: uint8_t = 0;
    let mut above_mi_col: core::ffi::c_int = mi_col;
    while above_mi_col < end_col && nb_count < nb_max {
        let mut above_mi: *mut *mut MB_MODE_INFO = prev_row_mi
            .offset(above_mi_col as isize);
        mi_step = (if (mi_size_wide[(**above_mi.offset(0 as core::ffi::c_int as isize))
            .bsize as usize] as core::ffi::c_int)
            < mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            mi_size_wide[(**above_mi.offset(0 as core::ffi::c_int as isize)).bsize
                as usize] as core::ffi::c_int
        } else {
            mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        }) as uint8_t;
        if mi_step as core::ffi::c_int == 1 as core::ffi::c_int {
            above_mi_col &= !(1 as core::ffi::c_int);
            above_mi = prev_row_mi
                .offset(above_mi_col as isize)
                .offset(1 as core::ffi::c_int as isize);
            mi_step = 2 as uint8_t;
        }
        if is_neighbor_overlappable(*above_mi) != 0 {
            nb_count += 1;
            fun
                .expect(
                    "non-null function pointer",
                )(
                xd,
                0 as core::ffi::c_int,
                above_mi_col - mi_col,
                (if ((*xd).width as core::ffi::c_int) < mi_step as core::ffi::c_int {
                    (*xd).width as core::ffi::c_int
                } else {
                    mi_step as core::ffi::c_int
                }) as uint8_t,
                0 as core::ffi::c_int,
                *above_mi,
                fun_ctxt,
                num_planes,
            );
        }
        above_mi_col += mi_step as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn foreach_overlappable_nb_left(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut nb_max: core::ffi::c_int,
    mut fun: overlappable_nb_visitor_t,
    mut fun_ctxt: *mut core::ffi::c_void,
) {
    if !(*xd).left_available {
        return;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut nb_count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mi_row: core::ffi::c_int = (*xd).mi_row;
    let mut prev_col_mi: *mut *mut MB_MODE_INFO = ((*xd).mi)
        .offset(-(1 as core::ffi::c_int as isize))
        .offset(-((mi_row * (*xd).mi_stride) as isize));
    let end_row: core::ffi::c_int = if (mi_row + (*xd).height as core::ffi::c_int)
        < (*cm).mi_params.mi_rows
    {
        mi_row + (*xd).height as core::ffi::c_int
    } else {
        (*cm).mi_params.mi_rows
    };
    let mut mi_step: uint8_t = 0;
    let mut left_mi_row: core::ffi::c_int = mi_row;
    while left_mi_row < end_row && nb_count < nb_max {
        let mut left_mi: *mut *mut MB_MODE_INFO = prev_col_mi
            .offset((left_mi_row * (*xd).mi_stride) as isize);
        mi_step = (if (mi_size_high[(**left_mi.offset(0 as core::ffi::c_int as isize))
            .bsize as usize] as core::ffi::c_int)
            < mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            mi_size_high[(**left_mi.offset(0 as core::ffi::c_int as isize)).bsize
                as usize] as core::ffi::c_int
        } else {
            mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        }) as uint8_t;
        if mi_step as core::ffi::c_int == 1 as core::ffi::c_int {
            left_mi_row &= !(1 as core::ffi::c_int);
            left_mi = prev_col_mi
                .offset(
                    ((left_mi_row + 1 as core::ffi::c_int) * (*xd).mi_stride) as isize,
                );
            mi_step = 2 as uint8_t;
        }
        if is_neighbor_overlappable(*left_mi) != 0 {
            nb_count += 1;
            fun
                .expect(
                    "non-null function pointer",
                )(
                xd,
                left_mi_row - mi_row,
                0 as core::ffi::c_int,
                (if ((*xd).height as core::ffi::c_int) < mi_step as core::ffi::c_int {
                    (*xd).height as core::ffi::c_int
                } else {
                    mi_step as core::ffi::c_int
                }) as uint8_t,
                1 as core::ffi::c_int,
                *left_mi,
                fun_ctxt,
                num_planes,
            );
        }
        left_mi_row += mi_step as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn init_subpel_params(
    src_mv: *const MV,
    inter_pred_params: *mut InterPredParams,
    mut subpel_params: *mut SubpelParams,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    let mut sf: *const scale_factors = (*inter_pred_params).scale_factors;
    let mut ssx: core::ffi::c_int = (*inter_pred_params).subsampling_x;
    let mut ssy: core::ffi::c_int = (*inter_pred_params).subsampling_y;
    let mut orig_pos_y: core::ffi::c_int = (*inter_pred_params).pix_row << SUBPEL_BITS;
    orig_pos_y
        += (*src_mv).row as core::ffi::c_int
            * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ssy);
    let mut orig_pos_x: core::ffi::c_int = (*inter_pred_params).pix_col << SUBPEL_BITS;
    orig_pos_x
        += (*src_mv).col as core::ffi::c_int
            * ((1 as core::ffi::c_int) << 1 as core::ffi::c_int - ssx);
    let is_scaled: core::ffi::c_int = av1_is_scaled(sf) as core::ffi::c_int;
    let mut pos_x: core::ffi::c_int = 0;
    let mut pos_y: core::ffi::c_int = 0;
    if (is_scaled == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        pos_y = av1_unscaled_value(orig_pos_y, sf);
        pos_x = av1_unscaled_value(orig_pos_x, sf);
    } else {
        pos_y = av1_scaled_y(orig_pos_y, sf);
        pos_x = av1_scaled_x(orig_pos_x, sf);
    }
    pos_x += SCALE_EXTRA_OFF;
    pos_y += SCALE_EXTRA_OFF;
    let bottom: core::ffi::c_int = height + AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS;
    let right: core::ffi::c_int = width + AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS;
    pos_y = clamp(pos_y, (*inter_pred_params).top, bottom);
    pos_x = clamp(pos_x, (*inter_pred_params).left, right);
    (*subpel_params).pos_x = pos_x;
    (*subpel_params).pos_y = pos_y;
    (*subpel_params).subpel_x = pos_x & SCALE_SUBPEL_MASK;
    (*subpel_params).subpel_y = pos_y & SCALE_SUBPEL_MASK;
    (*subpel_params).xs = (*sf).x_step_q4;
    (*subpel_params).ys = (*sf).y_step_q4;
}
#[inline]
unsafe extern "C-unwind" fn init_interp_filter_params(
    mut interp_filter_params: *mut *const InterpFilterParams,
    mut filter: *const InterpFilters,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
) {
    if is_intrabc as core::ffi::c_long != 0 {
        let fresh0 = &raw mut *interp_filter_params
            .offset(0 as core::ffi::c_int as isize);
        *fresh0 = &raw const av1_intrabc_filter_params;
        let fresh1 = &raw mut *interp_filter_params
            .offset(1 as core::ffi::c_int as isize);
        *fresh1 = &raw const av1_intrabc_filter_params;
    } else {
        let fresh2 = &raw mut *interp_filter_params
            .offset(0 as core::ffi::c_int as isize);
        *fresh2 = av1_get_interp_filter_params_with_block_size(
            (*filter).x_filter as InterpFilter,
            block_width,
        );
        let fresh3 = &raw mut *interp_filter_params
            .offset(1 as core::ffi::c_int as isize);
        *fresh3 = av1_get_interp_filter_params_with_block_size(
            (*filter).y_filter as InterpFilter,
            block_height,
        );
    };
}
#[inline]
unsafe extern "C-unwind" fn init_inter_block_params(
    mut inter_pred_params: *mut InterPredParams,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut pix_row: core::ffi::c_int,
    mut pix_col: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut use_hbd_buf: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
) {
    (*inter_pred_params).block_width = block_width;
    (*inter_pred_params).block_height = block_height;
    (*inter_pred_params).pix_row = pix_row;
    (*inter_pred_params).pix_col = pix_col;
    (*inter_pred_params).subsampling_x = subsampling_x;
    (*inter_pred_params).subsampling_y = subsampling_y;
    (*inter_pred_params).bit_depth = bit_depth;
    (*inter_pred_params).use_hbd_buf = use_hbd_buf;
    (*inter_pred_params).is_intrabc = is_intrabc;
    (*inter_pred_params).mode = TRANSLATION_PRED;
    (*inter_pred_params).comp_mode = UNIFORM_SINGLE;
    (*inter_pred_params).top = -((AOM_BORDER_IN_PIXELS >> subsampling_y)
        - AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS);
    (*inter_pred_params).left = -((AOM_BORDER_IN_PIXELS >> subsampling_x)
        - AOM_INTERP_EXTEND << SCALE_SUBPEL_BITS);
}
#[inline]
unsafe extern "C-unwind" fn av1_init_inter_params(
    mut inter_pred_params: *mut InterPredParams,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut pix_row: core::ffi::c_int,
    mut pix_col: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut use_hbd_buf: core::ffi::c_int,
    mut is_intrabc: core::ffi::c_int,
    mut sf: *const scale_factors,
    mut ref_buf: *const buf_2d,
    mut interp_filters: int_interpfilters,
) {
    init_inter_block_params(
        inter_pred_params,
        block_width,
        block_height,
        pix_row,
        pix_col,
        subsampling_x,
        subsampling_y,
        bit_depth,
        use_hbd_buf,
        is_intrabc,
    );
    init_interp_filter_params(
        ((*inter_pred_params).interp_filter_params).as_mut_ptr(),
        &raw mut interp_filters.as_filters,
        block_width,
        block_height,
        is_intrabc,
    );
    (*inter_pred_params).scale_factors = sf;
    (*inter_pred_params).ref_frame_buf = *ref_buf;
}
#[inline]
unsafe extern "C-unwind" fn av1_init_comp_mode(mut inter_pred_params: *mut InterPredParams) {
    (*inter_pred_params).comp_mode = UNIFORM_COMP;
}
#[inline]
unsafe extern "C-unwind" fn scaled_buffer_offset(
    mut x_offset: core::ffi::c_int,
    mut y_offset: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> int64_t {
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    if sf.is_null() {
        x = x_offset;
        y = y_offset;
    } else if av1_is_scaled(sf) != 0 {
        x = av1_scaled_x(x_offset, sf) >> SCALE_EXTRA_BITS;
        y = av1_scaled_y(y_offset, sf) >> SCALE_EXTRA_BITS;
    } else {
        x = av1_unscaled_value(x_offset, sf) >> SCALE_EXTRA_BITS;
        y = av1_unscaled_value(y_offset, sf) >> SCALE_EXTRA_BITS;
    }
    return y as int64_t * stride as int64_t + x as int64_t;
}
#[inline]
unsafe extern "C-unwind" fn setup_pred_plane(
    mut dst: *mut buf_2d,
    mut bsize: BLOCK_SIZE,
    mut src: *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut scale: *const scale_factors,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) {
    if subsampling_y != 0 && mi_row & 0x1 as core::ffi::c_int != 0
        && mi_size_high[bsize as usize] as core::ffi::c_int == 1 as core::ffi::c_int
    {
        mi_row -= 1 as core::ffi::c_int;
    }
    if subsampling_x != 0 && mi_col & 0x1 as core::ffi::c_int != 0
        && mi_size_wide[bsize as usize] as core::ffi::c_int == 1 as core::ffi::c_int
    {
        mi_col -= 1 as core::ffi::c_int;
    }
    let x: core::ffi::c_int = MI_SIZE * mi_col >> subsampling_x;
    let y: core::ffi::c_int = MI_SIZE * mi_row >> subsampling_y;
    (*dst).buf = src.offset(scaled_buffer_offset(x, y, stride, scale) as isize);
    (*dst).buf0 = src;
    (*dst).width = width;
    (*dst).height = height;
    (*dst).stride = stride;
}
pub const REF_SCALE_SHIFT: core::ffi::c_int = 14 as core::ffi::c_int;
pub const REF_NO_SCALE: core::ffi::c_int = (1 as core::ffi::c_int) << REF_SCALE_SHIFT;
pub const REF_INVALID_SCALE: core::ffi::c_int = -(1 as core::ffi::c_int);
#[inline]
unsafe extern "C-unwind" fn av1_scaled_x(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    let off: core::ffi::c_int = ((*sf).x_scale_fp
        - ((1 as core::ffi::c_int) << REF_SCALE_SHIFT))
        * ((1 as core::ffi::c_int) << SUBPEL_BITS - 1 as core::ffi::c_int);
    let tval: int64_t = val as int64_t * (*sf).x_scale_fp as int64_t + off as int64_t;
    return (if tval < 0 as int64_t {
        -(-tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int))
    } else {
        tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_scaled_y(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    let off: core::ffi::c_int = ((*sf).y_scale_fp
        - ((1 as core::ffi::c_int) << REF_SCALE_SHIFT))
        * ((1 as core::ffi::c_int) << SUBPEL_BITS - 1 as core::ffi::c_int);
    let tval: int64_t = val as int64_t * (*sf).y_scale_fp as int64_t + off as int64_t;
    return (if tval < 0 as int64_t {
        -(-tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int))
    } else {
        tval
            + ((1 as core::ffi::c_int as int64_t)
                << 14 as core::ffi::c_int
                    - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
                >> 1 as core::ffi::c_int)
            >> 14 as core::ffi::c_int - (10 as core::ffi::c_int - 4 as core::ffi::c_int)
    }) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_unscaled_value(
    mut val: core::ffi::c_int,
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    return val * ((1 as core::ffi::c_int) << SCALE_EXTRA_BITS);
}
#[inline]
unsafe extern "C-unwind" fn av1_is_valid_scale(
    mut sf: *const scale_factors,
) -> core::ffi::c_int {
    if !sf.is_null() {} else {
        __assert_fail(
            b"sf != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/scale.h\0" as *const u8
                as *const core::ffi::c_char,
            65 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"int av1_is_valid_scale(const struct scale_factors *)\0"))
                .as_ptr(),
        );
    };
    return ((*sf).x_scale_fp != REF_INVALID_SCALE
        && (*sf).y_scale_fp != REF_INVALID_SCALE) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_scaled(mut sf: *const scale_factors) -> core::ffi::c_int {
    if !sf.is_null() {} else {
        __assert_fail(
            b"sf != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/scale.h\0" as *const u8
                as *const core::ffi::c_char,
            71 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 48],
                [core::ffi::c_char; 48],
            >(*b"int av1_is_scaled(const struct scale_factors *)\0"))
                .as_ptr(),
        );
    };
    return (av1_is_valid_scale(sf) != 0
        && ((*sf).x_scale_fp != REF_NO_SCALE || (*sf).y_scale_fp != REF_NO_SCALE))
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn build_one_inter_predictor(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut src_mv: *const MV,
    mut inter_pred_params: *mut InterPredParams,
) {
    let mut subpel_params: SubpelParams = SubpelParams {
        xs: 0,
        ys: 0,
        subpel_x: 0,
        subpel_y: 0,
        pos_x: 0,
        pos_y: 0,
    };
    let mut src: *mut uint8_t = 0 as *mut uint8_t;
    let mut src_stride: core::ffi::c_int = 0;
    enc_calc_subpel_params(
        src_mv,
        inter_pred_params,
        &raw mut src,
        &raw mut subpel_params,
        &raw mut src_stride,
    );
    if (*inter_pred_params).comp_mode as core::ffi::c_uint
        == UNIFORM_SINGLE as core::ffi::c_int as core::ffi::c_uint
        || (*inter_pred_params).comp_mode as core::ffi::c_uint
            == UNIFORM_COMP as core::ffi::c_int as core::ffi::c_uint
    {
        av1_make_inter_predictor(
            src,
            src_stride,
            dst,
            dst_stride,
            inter_pred_params,
            &raw mut subpel_params,
        );
    } else {
        av1_make_masked_inter_predictor(
            src,
            src_stride,
            dst,
            dst_stride,
            inter_pred_params,
            &raw mut subpel_params,
        );
    };
}
unsafe extern "C-unwind" fn is_sub8x8_inter(
    mut xd: *const MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut bsize: BLOCK_SIZE,
    mut is_intrabc: core::ffi::c_int,
    mut build_for_obmc: core::ffi::c_int,
) -> bool {
    if is_intrabc != 0 || build_for_obmc != 0 {
        return false_0 != 0;
    }
    let pd: *const macroblockd_plane = &*((*xd).plane).as_ptr().offset(plane as isize)
        as *const macroblockd_plane;
    let ss_x: core::ffi::c_int = (*pd).subsampling_x;
    let ss_y: core::ffi::c_int = (*pd).subsampling_y;
    let is_sub4_x: core::ffi::c_int = (block_size_wide[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_x != 0) as core::ffi::c_int;
    let is_sub4_y: core::ffi::c_int = (block_size_high[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_y != 0) as core::ffi::c_int;
    if is_sub4_x == 0 && is_sub4_y == 0 {
        return false_0 != 0;
    }
    let row_start: core::ffi::c_int = if is_sub4_y != 0 {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let col_start: core::ffi::c_int = if is_sub4_x != 0 {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let mut row: core::ffi::c_int = row_start;
    while row <= 0 as core::ffi::c_int {
        let mut col: core::ffi::c_int = col_start;
        while col <= 0 as core::ffi::c_int {
            let mut this_mbmi: *const MB_MODE_INFO = *((*xd).mi)
                .offset((row * (*xd).mi_stride + col) as isize);
            if is_inter_block(this_mbmi) == 0 {
                return false_0 != 0;
            }
            if is_intrabc_block(this_mbmi) != 0 {
                return false_0 != 0;
            }
            col += 1;
        }
        row += 1;
    }
    return true_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn build_inter_predictors_sub8x8(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
) {
    let bsize: BLOCK_SIZE = (*mi).bsize;
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let ss_x: bool = (*pd).subsampling_x != 0;
    let ss_y: bool = (*pd).subsampling_y != 0;
    let b4_w: core::ffi::c_int = block_size_wide[bsize as usize] as core::ffi::c_int
        >> ss_x as core::ffi::c_int;
    let b4_h: core::ffi::c_int = block_size_high[bsize as usize] as core::ffi::c_int
        >> ss_y as core::ffi::c_int;
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        ss_x as core::ffi::c_int,
        ss_y as core::ffi::c_int,
    ) as BLOCK_SIZE;
    let b8_w: core::ffi::c_int = block_size_wide[plane_bsize as usize]
        as core::ffi::c_int;
    let b8_h: core::ffi::c_int = block_size_high[plane_bsize as usize]
        as core::ffi::c_int;
    let is_compound: core::ffi::c_int = has_second_ref(mi) as core::ffi::c_int;
    if is_compound == 0 {} else {
        __assert_fail(
            b"!is_compound\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/reconinter_template.inc\0" as *const u8
                as *const core::ffi::c_char,
            108 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 107],
                [core::ffi::c_char; 107],
            >(
                *b"void build_inter_predictors_sub8x8(const AV1_COMMON *, MACROBLOCKD *, int, const MB_MODE_INFO *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if is_intrabc_block(mi) == 0 {} else {
        __assert_fail(
            b"!is_intrabc_block(mi)\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/common/reconinter_template.inc\0" as *const u8
                as *const core::ffi::c_char,
            109 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 107],
                [core::ffi::c_char; 107],
            >(
                *b"void build_inter_predictors_sub8x8(const AV1_COMMON *, MACROBLOCKD *, int, const MB_MODE_INFO *, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let row_start: core::ffi::c_int = if block_size_high[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_y as core::ffi::c_int != 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let col_start: core::ffi::c_int = if block_size_wide[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_x as core::ffi::c_int != 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let pre_x: core::ffi::c_int = mi_x + MI_SIZE * col_start >> ss_x as core::ffi::c_int;
    let pre_y: core::ffi::c_int = mi_y + MI_SIZE * row_start >> ss_y as core::ffi::c_int;
    let mut row: core::ffi::c_int = row_start;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < b8_h {
        let mut col: core::ffi::c_int = col_start;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < b8_w {
            let mut this_mbmi: *mut MB_MODE_INFO = *((*xd).mi)
                .offset((row * (*xd).mi_stride + col) as isize);
            let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
            let mut dst: *mut uint8_t = ((*dst_buf).buf)
                .offset(((*dst_buf).stride * y) as isize)
                .offset(x as isize);
            let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut ref_buf: *const RefCntBuffer = get_ref_frame_buf(
                cm,
                (*this_mbmi).ref_frame[ref_0 as usize],
            );
            let mut ref_scale_factors: *const scale_factors = get_ref_scale_factors_const(
                cm,
                (*this_mbmi).ref_frame[ref_0 as usize],
            );
            let sf: *const scale_factors = ref_scale_factors;
            let pre_buf: buf_2d = {
                let mut init = buf_2d {
                    buf: 0 as *mut uint8_t,
                    buf0: if plane == 1 as core::ffi::c_int {
                        (*ref_buf).buf.c2rust_unnamed_4.c2rust_unnamed.u_buffer
                    } else {
                        (*ref_buf).buf.c2rust_unnamed_4.c2rust_unnamed.v_buffer
                    },
                    width: (*ref_buf).buf.c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
                    height: (*ref_buf)
                        .buf
                        .c2rust_unnamed_2
                        .c2rust_unnamed
                        .uv_crop_height,
                    stride: (*ref_buf).buf.c2rust_unnamed_3.c2rust_unnamed.uv_stride,
                };
                init
            };
            let mv: MV = (*this_mbmi).mv[ref_0 as usize].as_mv;
            let mut inter_pred_params: InterPredParams = InterPredParams {
                mode: TRANSLATION_PRED,
                comp_mode: UNIFORM_SINGLE,
                warp_params: WarpedMotionParams {
                    wmmat: [0; 6],
                    alpha: 0,
                    beta: 0,
                    gamma: 0,
                    delta: 0,
                    wmtype: 0,
                    invalid: 0,
                },
                conv_params: ConvolveParams {
                    do_average: 0,
                    dst: 0 as *mut CONV_BUF_TYPE,
                    dst_stride: 0,
                    round_0: 0,
                    round_1: 0,
                    plane: 0,
                    is_compound: 0,
                    use_dist_wtd_comp_avg: 0,
                    fwd_offset: 0,
                    bck_offset: 0,
                },
                interp_filter_params: [0 as *const InterpFilterParams; 2],
                block_width: 0,
                block_height: 0,
                pix_row: 0,
                pix_col: 0,
                ref_frame_buf: buf_2d {
                    buf: 0 as *mut uint8_t,
                    buf0: 0 as *mut uint8_t,
                    width: 0,
                    height: 0,
                    stride: 0,
                },
                subsampling_x: 0,
                subsampling_y: 0,
                scale_factors: 0 as *const scale_factors,
                bit_depth: 0,
                use_hbd_buf: 0,
                mask_comp: INTERINTER_COMPOUND_DATA {
                    seg_mask: 0 as *mut uint8_t,
                    wedge_index: 0,
                    wedge_sign: 0,
                    mask_type: 0,
                    type_0: 0,
                },
                sb_type: BLOCK_4X4,
                is_intrabc: 0,
                top: 0,
                left: 0,
            };
            av1_init_inter_params(
                &raw mut inter_pred_params,
                b4_w,
                b4_h,
                pre_y + y,
                pre_x + x,
                (*pd).subsampling_x,
                (*pd).subsampling_y,
                (*xd).bd,
                is_cur_buf_hbd(xd),
                (*mi).use_intrabc() as core::ffi::c_int,
                sf,
                &pre_buf,
                (*this_mbmi).interp_filters,
            );
            inter_pred_params.conv_params = get_conv_params_no_round(
                ref_0,
                plane,
                0 as *mut CONV_BUF_TYPE,
                0 as core::ffi::c_int,
                is_compound,
                (*xd).bd,
            );
            build_one_inter_predictor(
                dst,
                (*dst_buf).stride,
                &mv,
                &raw mut inter_pred_params,
            );
            col += 1;
            x += b4_w;
        }
        row += 1;
        y += b4_h;
    }
}
#[inline]
unsafe extern "C-unwind" fn build_inter_predictors_8x8_and_bigger(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut build_for_obmc: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
) {
    let is_compound: core::ffi::c_int = has_second_ref(mi) as core::ffi::c_int;
    let is_intrabc: core::ffi::c_int = is_intrabc_block(mi) as core::ffi::c_int;
    if is_intrabc == 0 || is_compound == 0 {} else {
        __assert_fail(
            b"IMPLIES(is_intrabc, !is_compound)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/common/reconinter_template.inc\0" as *const u8
                as *const core::ffi::c_char,
            175 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 130],
                [core::ffi::c_char; 130],
            >(
                *b"void build_inter_predictors_8x8_and_bigger(const AV1_COMMON *, MACROBLOCKD *, int, const MB_MODE_INFO *, int, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let dst_buf: *mut buf_2d = &raw mut (*pd).dst;
    let dst: *mut uint8_t = (*dst_buf).buf;
    let mut is_global: [core::ffi::c_int; 2] = [
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ];
    let mut ref_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_0 < 1 as core::ffi::c_int + is_compound {
        let wm: *const WarpedMotionParams = &*((*xd).global_motion)
            .offset(*((*mi).ref_frame).as_ptr().offset(ref_0 as isize) as isize)
            as *const WarpedMotionParams;
        is_global[ref_0 as usize] = is_global_mv_block(mi, (*wm).wmtype);
        ref_0 += 1;
    }
    let bsize: BLOCK_SIZE = (*mi).bsize;
    let ss_x: core::ffi::c_int = (*pd).subsampling_x;
    let ss_y: core::ffi::c_int = (*pd).subsampling_y;
    let row_start: core::ffi::c_int = if block_size_high[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_y != 0 && build_for_obmc == 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let col_start: core::ffi::c_int = if block_size_wide[bsize as usize]
        as core::ffi::c_int == 4 as core::ffi::c_int && ss_x != 0 && build_for_obmc == 0
    {
        -(1 as core::ffi::c_int)
    } else {
        0 as core::ffi::c_int
    };
    let pre_x: core::ffi::c_int = mi_x + MI_SIZE * col_start >> ss_x;
    let pre_y: core::ffi::c_int = mi_y + MI_SIZE * row_start >> ss_y;
    let mut ref_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while ref_1 < 1 as core::ffi::c_int + is_compound {
        let sf: *const scale_factors = if is_intrabc != 0 {
            &(*cm).sf_identity
        } else {
            (*xd).block_ref_scale_factors[ref_1 as usize]
        };
        let pre_buf: *mut buf_2d = if is_intrabc != 0 {
            dst_buf
        } else {
            &raw mut *((*pd).pre).as_mut_ptr().offset(ref_1 as isize) as *mut buf_2d
        };
        let mv: MV = (*mi).mv[ref_1 as usize].as_mv;
        let warp_types: WarpTypesAllowed = {
            let mut init = WarpTypesAllowed {
                global_warp_allowed: is_global[ref_1 as usize],
                local_warp_allowed: ((*mi).motion_mode as core::ffi::c_int
                    == WARPED_CAUSAL as core::ffi::c_int) as core::ffi::c_int,
            };
            init
        };
        let mut inter_pred_params: InterPredParams = InterPredParams {
            mode: TRANSLATION_PRED,
            comp_mode: UNIFORM_SINGLE,
            warp_params: WarpedMotionParams {
                wmmat: [0; 6],
                alpha: 0,
                beta: 0,
                gamma: 0,
                delta: 0,
                wmtype: 0,
                invalid: 0,
            },
            conv_params: ConvolveParams {
                do_average: 0,
                dst: 0 as *mut CONV_BUF_TYPE,
                dst_stride: 0,
                round_0: 0,
                round_1: 0,
                plane: 0,
                is_compound: 0,
                use_dist_wtd_comp_avg: 0,
                fwd_offset: 0,
                bck_offset: 0,
            },
            interp_filter_params: [0 as *const InterpFilterParams; 2],
            block_width: 0,
            block_height: 0,
            pix_row: 0,
            pix_col: 0,
            ref_frame_buf: buf_2d {
                buf: 0 as *mut uint8_t,
                buf0: 0 as *mut uint8_t,
                width: 0,
                height: 0,
                stride: 0,
            },
            subsampling_x: 0,
            subsampling_y: 0,
            scale_factors: 0 as *const scale_factors,
            bit_depth: 0,
            use_hbd_buf: 0,
            mask_comp: INTERINTER_COMPOUND_DATA {
                seg_mask: 0 as *mut uint8_t,
                wedge_index: 0,
                wedge_sign: 0,
                mask_type: 0,
                type_0: 0,
            },
            sb_type: BLOCK_4X4,
            is_intrabc: 0,
            top: 0,
            left: 0,
        };
        av1_init_inter_params(
            &raw mut inter_pred_params,
            bw,
            bh,
            pre_y,
            pre_x,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
            (*xd).bd,
            is_cur_buf_hbd(xd),
            (*mi).use_intrabc() as core::ffi::c_int,
            sf,
            pre_buf,
            (*mi).interp_filters,
        );
        if is_compound != 0 {
            av1_init_comp_mode(&raw mut inter_pred_params);
        }
        inter_pred_params.conv_params = get_conv_params_no_round(
            ref_1,
            plane,
            (*xd).tmp_conv_dst,
            MAX_SB_SIZE,
            is_compound,
            (*xd).bd,
        );
        av1_dist_wtd_comp_weight_assign(
            cm,
            mi,
            &raw mut inter_pred_params.conv_params.fwd_offset,
            &raw mut inter_pred_params.conv_params.bck_offset,
            &raw mut inter_pred_params.conv_params.use_dist_wtd_comp_avg,
            is_compound,
        );
        if build_for_obmc == 0 {
            av1_init_warp_params(&raw mut inter_pred_params, &warp_types, ref_1, xd, mi);
        }
        if is_masked_compound_type((*mi).interinter_comp.type_0) != 0 {
            inter_pred_params.sb_type = (*mi).bsize;
            inter_pred_params.mask_comp = (*mi).interinter_comp;
            if ref_1 == 1 as core::ffi::c_int {
                inter_pred_params.conv_params.do_average = 0 as core::ffi::c_int;
                inter_pred_params.comp_mode = MASK_COMP;
            }
            inter_pred_params.mask_comp.seg_mask = (*xd).seg_mask;
        }
        build_one_inter_predictor(dst, (*dst_buf).stride, &mv, &raw mut inter_pred_params);
        ref_1 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn build_inter_predictors(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut mi: *const MB_MODE_INFO,
    mut build_for_obmc: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut mi_x: core::ffi::c_int,
    mut mi_y: core::ffi::c_int,
) {
    if is_sub8x8_inter(xd, plane, (*mi).bsize, is_intrabc_block(mi), build_for_obmc) {
        if bw < 8 as core::ffi::c_int || bh < 8 as core::ffi::c_int {} else {
            __assert_fail(
                b"bw < 8 || bh < 8\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/common/reconinter_template.inc\0" as *const u8
                    as *const core::ffi::c_char,
                263 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 115],
                    [core::ffi::c_char; 115],
                >(
                    *b"void build_inter_predictors(const AV1_COMMON *, MACROBLOCKD *, int, const MB_MODE_INFO *, int, int, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        };
        build_inter_predictors_sub8x8(cm, xd, plane, mi, mi_x, mi_y);
    } else {
        build_inter_predictors_8x8_and_bigger(
            cm,
            xd,
            plane,
            mi,
            build_for_obmc,
            bw,
            bh,
            mi_x,
            mi_y,
        );
    };
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
