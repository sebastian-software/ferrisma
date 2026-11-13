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

// Mechanically translated from upstream/aom/av1/common/av1_loopfilter.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    static mut aom_highbd_lpf_horizontal_14: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_4: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_6: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_8: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_horizontal_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_14: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_4: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_6: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_8: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_highbd_lpf_vertical_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, core::ffi::c_int) -> ()>;
    static mut aom_lpf_horizontal_14: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_14_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_4_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_6: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_6_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_horizontal_8_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_14: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_14_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_14_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_4: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_4_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_4_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_6: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_6_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_6_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_8_dual: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
    static mut aom_lpf_vertical_8_quad: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, *const uint8_t, *const uint8_t, *const uint8_t) -> ()>;
}
pub type ptrdiff_t = isize;
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
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_12 = 25;
pub const INTRA_MODES: C2RustUnnamed_12 = 13;
pub const INTER_MODE_END: C2RustUnnamed_12 = 25;
pub const INTER_MODE_START: C2RustUnnamed_12 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_12 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_12 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_12 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_12 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_12 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_12 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_12 = 13;
pub const DIR_MODE_END: C2RustUnnamed_12 = 9;
pub const DIR_MODE_START: C2RustUnnamed_12 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_12 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_12 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_12 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_12 = 25;
pub const NEW_NEWMV: C2RustUnnamed_12 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_12 = 23;
pub const NEW_NEARMV: C2RustUnnamed_12 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_12 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_12 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_12 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_12 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_12 = 17;
pub const NEWMV: C2RustUnnamed_12 = 16;
pub const GLOBALMV: C2RustUnnamed_12 = 15;
pub const NEARMV: C2RustUnnamed_12 = 14;
pub const NEARESTMV: C2RustUnnamed_12 = 13;
pub const PAETH_PRED: C2RustUnnamed_12 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_12 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_12 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_12 = 9;
pub const D67_PRED: C2RustUnnamed_12 = 8;
pub const D203_PRED: C2RustUnnamed_12 = 7;
pub const D157_PRED: C2RustUnnamed_12 = 6;
pub const D113_PRED: C2RustUnnamed_12 = 5;
pub const D135_PRED: C2RustUnnamed_12 = 4;
pub const D45_PRED: C2RustUnnamed_12 = 3;
pub const H_PRED: C2RustUnnamed_12 = 2;
pub const V_PRED: C2RustUnnamed_12 = 1;
pub const DC_PRED: C2RustUnnamed_12 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
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
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
pub type aom_cdf_prob = uint16_t;
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed_14 = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed_14 = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed_14 = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed_14 = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed_14 = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed_14 = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed_14 = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed_14 = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed_14 = 0;
pub type SEG_LVL_FEATURES = uint8_t;
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
pub type MACROBLOCKD_PLANE = macroblockd_plane;
pub type MACROBLOCKD = macroblockd;
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const NUM_EDGE_DIRS: C2RustUnnamed_15 = 2;
pub const HORZ_EDGE: C2RustUnnamed_15 = 1;
pub const VERT_EDGE: C2RustUnnamed_15 = 0;
pub type EDGE_DIR = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_DEBLOCKING_PARAMETERS {
    pub filter_length: uint8_t,
    pub lfthr: *const loop_filter_thresh,
}
pub type AV1_COMMON = AV1Common;
pub type USE_FILTER_TYPE = uint8_t;
pub const USE_SINGLE: C2RustUnnamed_16 = 0;
pub const USE_DUAL: C2RustUnnamed_16 = 1;
pub const USE_QUAD: C2RustUnnamed_16 = 2;
pub type C2RustUnnamed_16 = core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_MODE_LF_DELTAS: core::ffi::c_int = 2 as core::ffi::c_int;
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
static mut max_txsize_rect_lookup: [TX_SIZE; 22] = [
    TX_4X4 as core::ffi::c_int as TX_SIZE,
    TX_4X8 as core::ffi::c_int as TX_SIZE,
    TX_8X4 as core::ffi::c_int as TX_SIZE,
    TX_8X8 as core::ffi::c_int as TX_SIZE,
    TX_8X16 as core::ffi::c_int as TX_SIZE,
    TX_16X8 as core::ffi::c_int as TX_SIZE,
    TX_16X16 as core::ffi::c_int as TX_SIZE,
    TX_16X32 as core::ffi::c_int as TX_SIZE,
    TX_32X16 as core::ffi::c_int as TX_SIZE,
    TX_32X32 as core::ffi::c_int as TX_SIZE,
    TX_32X64 as core::ffi::c_int as TX_SIZE,
    TX_64X32 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_64X64 as core::ffi::c_int as TX_SIZE,
    TX_4X16 as core::ffi::c_int as TX_SIZE,
    TX_16X4 as core::ffi::c_int as TX_SIZE,
    TX_8X32 as core::ffi::c_int as TX_SIZE,
    TX_32X8 as core::ffi::c_int as TX_SIZE,
    TX_16X64 as core::ffi::c_int as TX_SIZE,
    TX_64X16 as core::ffi::c_int as TX_SIZE,
];
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
static mut tx_size_wide_unit_log2: [core::ffi::c_int; 19] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    0 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut tx_size_high_unit_log2: [core::ffi::c_int; 19] = [
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    4 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    3 as core::ffi::c_int,
    2 as core::ffi::c_int,
    0 as core::ffi::c_int,
    3 as core::ffi::c_int,
    1 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
];
pub const MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn segfeature_active(
    mut seg: *const segmentation,
    mut segment_id: uint8_t,
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return ((*seg).enabled as core::ffi::c_int != 0
        && (*seg).feature_mask[segment_id as usize]
            & ((1 as core::ffi::c_int) << feature_id as core::ffi::c_int)
                as core::ffi::c_uint != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_segdata(
    mut seg: *const segmentation,
    mut segment_id: core::ffi::c_int,
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return (*seg).feature_data[segment_id as usize][feature_id as usize]
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
unsafe extern "C-unwind" fn get_plane_block_size(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> BLOCK_SIZE {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1194 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_x >= 0 as core::ffi::c_int && subsampling_x < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1195 as core::ffi::c_int,
            b"subsampling_x >= 0 && subsampling_x < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_y >= 0 as core::ffi::c_int && subsampling_y < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1196 as core::ffi::c_int,
            b"subsampling_y >= 0 && subsampling_y < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return av1_ss_size_lookup[bsize
        as usize][subsampling_x as usize][subsampling_y as usize];
}
#[inline]
unsafe extern "C-unwind" fn av1_get_txb_size_index(
    mut bsize: BLOCK_SIZE,
    mut blk_row: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
) -> core::ffi::c_int {
    static mut tw_w_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
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
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
    ];
    static mut tw_h_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
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
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
    ];
    static mut stride_log2_table: [uint8_t; 22] = [
        0 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
    ];
    let index: core::ffi::c_int = ((blk_row
        >> tw_h_log2_table[bsize as usize] as core::ffi::c_int)
        << stride_log2_table[bsize as usize] as core::ffi::c_int)
        + (blk_col >> tw_w_log2_table[bsize as usize] as core::ffi::c_int);
    if !(index < 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_get_txb_size_index\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1225 as core::ffi::c_int,
            b"index < INTER_TX_SIZE_BUF_LEN\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return index;
}
#[inline]
unsafe extern "C-unwind" fn av1_get_adjusted_tx_size(mut tx_size: TX_SIZE) -> TX_SIZE {
    match tx_size as core::ffi::c_int {
        4 | 12 | 11 => return TX_32X32 as core::ffi::c_int as TX_SIZE,
        18 => return TX_32X16 as core::ffi::c_int as TX_SIZE,
        17 => return TX_16X32 as core::ffi::c_int as TX_SIZE,
        _ => return tx_size,
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_get_max_uv_txsize(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> TX_SIZE {
    let plane_bsize: BLOCK_SIZE = get_plane_block_size(
        bsize,
        subsampling_x,
        subsampling_y,
    ) as BLOCK_SIZE;
    if !((plane_bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"av1_get_max_uv_txsize\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1381 as core::ffi::c_int,
            b"plane_bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let uv_tx: TX_SIZE = max_txsize_rect_lookup[plane_bsize as usize];
    return av1_get_adjusted_tx_size(uv_tx);
}
pub const MAX_LOOP_FILTER: core::ffi::c_int = 63 as core::ffi::c_int;
pub const SIMD_WIDTH: core::ffi::c_int = 16 as core::ffi::c_int;
static mut seg_lvl_lf_lut: [[SEG_LVL_FEATURES; 2]; 3] = [
    [
        SEG_LVL_ALT_LF_Y_V as core::ffi::c_int as SEG_LVL_FEATURES,
        SEG_LVL_ALT_LF_Y_H as core::ffi::c_int as SEG_LVL_FEATURES,
    ],
    [
        SEG_LVL_ALT_LF_U as core::ffi::c_int as SEG_LVL_FEATURES,
        SEG_LVL_ALT_LF_U as core::ffi::c_int as SEG_LVL_FEATURES,
    ],
    [
        SEG_LVL_ALT_LF_V as core::ffi::c_int as SEG_LVL_FEATURES,
        SEG_LVL_ALT_LF_V as core::ffi::c_int as SEG_LVL_FEATURES,
    ],
];
static mut delta_lf_id_lut: [[core::ffi::c_int; 2]; 3] = [
    [0 as core::ffi::c_int, 1 as core::ffi::c_int],
    [2 as core::ffi::c_int, 2 as core::ffi::c_int],
    [3 as core::ffi::c_int, 3 as core::ffi::c_int],
];
static mut mode_lf_lut: [core::ffi::c_int; 25] = [
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    1 as core::ffi::c_int,
];
unsafe extern "C-unwind" fn update_sharpness(
    mut lfi: *mut loop_filter_info_n,
    mut sharpness_lvl: core::ffi::c_int,
) {
    let mut lvl: core::ffi::c_int = 0;
    lvl = 0 as core::ffi::c_int;
    while lvl <= MAX_LOOP_FILTER {
        let mut block_inside_limit: core::ffi::c_int = lvl
            >> (sharpness_lvl > 0 as core::ffi::c_int) as core::ffi::c_int
                + (sharpness_lvl > 4 as core::ffi::c_int) as core::ffi::c_int;
        if sharpness_lvl > 0 as core::ffi::c_int {
            if block_inside_limit > 9 as core::ffi::c_int - sharpness_lvl {
                block_inside_limit = 9 as core::ffi::c_int - sharpness_lvl;
            }
        }
        if block_inside_limit < 1 as core::ffi::c_int {
            block_inside_limit = 1 as core::ffi::c_int;
        }
        memset(
            ((*lfi).lfthr[lvl as usize].lim).as_mut_ptr() as *mut core::ffi::c_void,
            block_inside_limit,
            SIMD_WIDTH as size_t,
        );
        memset(
            ((*lfi).lfthr[lvl as usize].mblim).as_mut_ptr() as *mut core::ffi::c_void,
            2 as core::ffi::c_int * (lvl + 2 as core::ffi::c_int) + block_inside_limit,
            SIMD_WIDTH as size_t,
        );
        lvl += 1;
    }
}
unsafe extern "C-unwind" fn get_filter_level(
    mut cm: *const AV1_COMMON,
    mut lfi_n: *const loop_filter_info_n,
    dir_idx: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut mbmi: *const MB_MODE_INFO,
) -> uint8_t {
    let segment_id: core::ffi::c_int = (*mbmi).segment_id() as core::ffi::c_int;
    if (*cm).delta_q_info.delta_lf_present_flag != 0 {
        let mut delta_lf: int8_t = 0;
        if (*cm).delta_q_info.delta_lf_multi != 0 {
            let delta_lf_idx: core::ffi::c_int = delta_lf_id_lut[plane
                as usize][dir_idx as usize];
            delta_lf = (*mbmi).delta_lf[delta_lf_idx as usize];
        } else {
            delta_lf = (*mbmi).delta_lf_from_base;
        }
        let mut base_level: core::ffi::c_int = 0;
        if plane == 0 as core::ffi::c_int {
            base_level = (*cm).lf.filter_level[dir_idx as usize];
        } else if plane == 1 as core::ffi::c_int {
            base_level = (*cm).lf.filter_level_u;
        } else {
            base_level = (*cm).lf.filter_level_v;
        }
        let mut lvl_seg: core::ffi::c_int = clamp(
            delta_lf as core::ffi::c_int + base_level,
            0 as core::ffi::c_int,
            MAX_LOOP_FILTER,
        );
        if !(plane >= 0 as core::ffi::c_int && plane <= 2 as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"get_filter_level\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                89 as core::ffi::c_int,
                b"plane >= 0 && plane <= 2\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let seg_lf_feature_id: core::ffi::c_int = seg_lvl_lf_lut[plane
            as usize][dir_idx as usize] as core::ffi::c_int;
        if segfeature_active(
            &(*cm).seg,
            segment_id as uint8_t,
            seg_lf_feature_id as SEG_LVL_FEATURES,
        ) != 0
        {
            let data: core::ffi::c_int = get_segdata(
                &(*cm).seg,
                segment_id,
                seg_lf_feature_id as SEG_LVL_FEATURES,
            ) as core::ffi::c_int;
            lvl_seg = clamp(lvl_seg + data, 0 as core::ffi::c_int, MAX_LOOP_FILTER);
        }
        if (*cm).lf.mode_ref_delta_enabled != 0 {
            let scale: core::ffi::c_int = (1 as core::ffi::c_int)
                << (lvl_seg >> 5 as core::ffi::c_int);
            lvl_seg
                += (*cm)
                    .lf
                    .ref_deltas[(*mbmi).ref_frame[0 as core::ffi::c_int as usize]
                    as usize] as core::ffi::c_int * scale;
            if (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > INTRA_FRAME as core::ffi::c_int
            {
                lvl_seg
                    += (*cm).lf.mode_deltas[mode_lf_lut[(*mbmi).mode as usize] as usize]
                        as core::ffi::c_int * scale;
            }
            lvl_seg = clamp(lvl_seg, 0 as core::ffi::c_int, MAX_LOOP_FILTER);
        }
        return lvl_seg as uint8_t;
    } else {
        return (*lfi_n)
            .lvl[plane
            as usize][segment_id
            as usize][dir_idx
            as usize][(*mbmi).ref_frame[0 as core::ffi::c_int as usize]
            as usize][mode_lf_lut[(*mbmi).mode as usize] as usize]
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_filter_init(mut cm: *mut AV1_COMMON) {
    if !(MB_MODE_COUNT as core::ffi::c_int
        == (::core::mem::size_of::<[core::ffi::c_int; 25]>() as usize)
            .wrapping_div(::core::mem::size_of::<core::ffi::c_int>() as usize)
            as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"av1_loop_filter_init\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            111 as core::ffi::c_int,
            b"MB_MODE_COUNT == NELEMENTS(mode_lf_lut)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut lfi: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
    let mut lf: *mut loopfilter = &raw mut (*cm).lf;
    let mut lvl: core::ffi::c_int = 0;
    update_sharpness(lfi, (*lf).sharpness_level);
    lvl = 0 as core::ffi::c_int;
    while lvl <= MAX_LOOP_FILTER {
        memset(
            ((*lfi).lfthr[lvl as usize].hev_thr).as_mut_ptr() as *mut core::ffi::c_void,
            lvl >> 4 as core::ffi::c_int,
            SIMD_WIDTH as size_t,
        );
        lvl += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_filter_frame_init(
    mut cm: *mut AV1_COMMON,
    mut plane_start: core::ffi::c_int,
    mut plane_end: core::ffi::c_int,
) {
    let mut filt_lvl: [core::ffi::c_int; 3] = [0; 3];
    let mut filt_lvl_r: [core::ffi::c_int; 3] = [0; 3];
    let mut plane: core::ffi::c_int = 0;
    let mut seg_id: core::ffi::c_int = 0;
    let lfi: *mut loop_filter_info_n = &raw mut (*cm).lf_info;
    let lf: *mut loopfilter = &raw mut (*cm).lf;
    let seg: *const segmentation = &raw mut (*cm).seg;
    update_sharpness(lfi, (*lf).sharpness_level);
    filt_lvl[0 as core::ffi::c_int as usize] = (*cm)
        .lf
        .filter_level[0 as core::ffi::c_int as usize];
    filt_lvl[1 as core::ffi::c_int as usize] = (*cm).lf.filter_level_u;
    filt_lvl[2 as core::ffi::c_int as usize] = (*cm).lf.filter_level_v;
    filt_lvl_r[0 as core::ffi::c_int as usize] = (*cm)
        .lf
        .filter_level[1 as core::ffi::c_int as usize];
    filt_lvl_r[1 as core::ffi::c_int as usize] = (*cm).lf.filter_level_u;
    filt_lvl_r[2 as core::ffi::c_int as usize] = (*cm).lf.filter_level_v;
    if !(plane_start >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_loop_filter_frame_init\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            150 as core::ffi::c_int,
            b"plane_start >= AOM_PLANE_Y\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(plane_end <= 3 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 27],
                [core::ffi::c_char; 27],
            >(*b"av1_loop_filter_frame_init\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            151 as core::ffi::c_int,
            b"plane_end <= MAX_MB_PLANE\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    plane = plane_start;
    while plane < plane_end {
        if plane == 0 as core::ffi::c_int
            && filt_lvl[0 as core::ffi::c_int as usize] == 0
            && filt_lvl_r[0 as core::ffi::c_int as usize] == 0
        {
            break;
        }
        if !(plane == 1 as core::ffi::c_int
            && filt_lvl[1 as core::ffi::c_int as usize] == 0)
        {
            if !(plane == 2 as core::ffi::c_int
                && filt_lvl[2 as core::ffi::c_int as usize] == 0)
            {
                seg_id = 0 as core::ffi::c_int;
                while seg_id < MAX_SEGMENTS {
                    let mut dir: core::ffi::c_int = 0 as core::ffi::c_int;
                    while dir < 2 as core::ffi::c_int {
                        let mut lvl_seg: core::ffi::c_int = if dir
                            == 0 as core::ffi::c_int
                        {
                            filt_lvl[plane as usize]
                        } else {
                            filt_lvl_r[plane as usize]
                        };
                        let seg_lf_feature_id: core::ffi::c_int = seg_lvl_lf_lut[plane
                            as usize][dir as usize] as core::ffi::c_int;
                        if segfeature_active(
                            seg,
                            seg_id as uint8_t,
                            seg_lf_feature_id as SEG_LVL_FEATURES,
                        ) != 0
                        {
                            let data: core::ffi::c_int = get_segdata(
                                &raw mut (*cm).seg,
                                seg_id,
                                seg_lf_feature_id as SEG_LVL_FEATURES,
                            ) as core::ffi::c_int;
                            lvl_seg = clamp(
                                lvl_seg + data,
                                0 as core::ffi::c_int,
                                MAX_LOOP_FILTER,
                            );
                        }
                        if (*lf).mode_ref_delta_enabled == 0 {
                            memset(
                                ((*lfi).lvl[plane as usize][seg_id as usize][dir as usize])
                                    .as_mut_ptr() as *mut core::ffi::c_void,
                                lvl_seg,
                                ::core::mem::size_of::<[[uint8_t; 2]; 8]>() as size_t,
                            );
                        } else {
                            let mut ref_0: core::ffi::c_int = 0;
                            let mut mode: core::ffi::c_int = 0;
                            let scale: core::ffi::c_int = (1 as core::ffi::c_int)
                                << (lvl_seg >> 5 as core::ffi::c_int);
                            let intra_lvl: core::ffi::c_int = lvl_seg
                                + (*lf).ref_deltas[INTRA_FRAME as core::ffi::c_int as usize]
                                    as core::ffi::c_int * scale;
                            (*lfi)
                                .lvl[plane
                                as usize][seg_id
                                as usize][dir
                                as usize][INTRA_FRAME as core::ffi::c_int
                                as usize][0 as core::ffi::c_int as usize] = clamp(
                                intra_lvl,
                                0 as core::ffi::c_int,
                                MAX_LOOP_FILTER,
                            ) as uint8_t;
                            ref_0 = LAST_FRAME as core::ffi::c_int;
                            while ref_0 < REF_FRAMES as core::ffi::c_int {
                                mode = 0 as core::ffi::c_int;
                                while mode < MAX_MODE_LF_DELTAS {
                                    let inter_lvl: core::ffi::c_int = lvl_seg
                                        + (*lf).ref_deltas[ref_0 as usize] as core::ffi::c_int
                                            * scale
                                        + (*lf).mode_deltas[mode as usize] as core::ffi::c_int
                                            * scale;
                                    (*lfi)
                                        .lvl[plane
                                        as usize][seg_id
                                        as usize][dir as usize][ref_0 as usize][mode as usize] = clamp(
                                        inter_lvl,
                                        0 as core::ffi::c_int,
                                        MAX_LOOP_FILTER,
                                    ) as uint8_t;
                                    mode += 1;
                                }
                                ref_0 += 1;
                            }
                        }
                        dir += 1;
                    }
                    seg_id += 1;
                }
            }
        }
        plane += 1;
    }
}
#[inline(always)]
unsafe extern "C-unwind" fn get_transform_size(
    xd: *const MACROBLOCKD,
    mbmi: *const MB_MODE_INFO,
    mi_row: core::ffi::c_int,
    mi_col: core::ffi::c_int,
    plane: core::ffi::c_int,
    ss_x: core::ffi::c_int,
    ss_y: core::ffi::c_int,
) -> TX_SIZE {
    if mbmi.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"get_transform_size\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            200 as core::ffi::c_int,
            b"mbmi != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !xd.is_null() && (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
        return TX_4X4 as core::ffi::c_int as TX_SIZE;
    }
    let mut tx_size: TX_SIZE = (if plane == AOM_PLANE_Y {
        (*mbmi).tx_size as core::ffi::c_int
    } else {
        av1_get_max_uv_txsize((*mbmi).bsize, ss_x, ss_y) as core::ffi::c_int
    }) as TX_SIZE;
    if !((tx_size as core::ffi::c_int) < TX_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"get_transform_size\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            206 as core::ffi::c_int,
            b"tx_size < TX_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if plane == AOM_PLANE_Y && is_inter_block(mbmi) != 0 && (*mbmi).skip_txfm == 0 {
        let sb_type: BLOCK_SIZE = (*mbmi).bsize;
        let blk_row: core::ffi::c_int = mi_row
            & mi_size_high[sb_type as usize] as core::ffi::c_int - 1 as core::ffi::c_int;
        let blk_col: core::ffi::c_int = mi_col
            & mi_size_wide[sb_type as usize] as core::ffi::c_int - 1 as core::ffi::c_int;
        let mb_tx_size: TX_SIZE = (*mbmi)
            .inter_tx_size[av1_get_txb_size_index(sb_type, blk_row, blk_col) as usize];
        if !((mb_tx_size as core::ffi::c_int) < TX_SIZES_ALL as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"get_transform_size\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                213 as core::ffi::c_int,
                b"mb_tx_size < TX_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        tx_size = mb_tx_size;
    }
    return tx_size;
}
static mut tx_dim_to_filter_length: [core::ffi::c_int; 5] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
    14 as core::ffi::c_int,
];
unsafe extern "C-unwind" fn set_lpf_parameters(
    params: *mut AV1_DEBLOCKING_PARAMETERS,
    mode_step: ptrdiff_t,
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    edge_dir: EDGE_DIR,
    x: uint32_t,
    y: uint32_t,
    plane: core::ffi::c_int,
    plane_ptr: *const macroblockd_plane,
) -> TX_SIZE {
    (*params).filter_length = 0 as uint8_t;
    let width: uint32_t = (*plane_ptr).dst.width as uint32_t;
    let height: uint32_t = (*plane_ptr).dst.height as uint32_t;
    if width <= x || height <= y {
        return TX_4X4 as core::ffi::c_int as TX_SIZE;
    }
    let scale_horz: uint32_t = (*plane_ptr).subsampling_x as uint32_t;
    let scale_vert: uint32_t = (*plane_ptr).subsampling_y as uint32_t;
    let mi_row: core::ffi::c_int = (scale_vert | y << scale_vert >> MI_SIZE_LOG2)
        as core::ffi::c_int;
    let mi_col: core::ffi::c_int = (scale_horz | x << scale_horz >> MI_SIZE_LOG2)
        as core::ffi::c_int;
    let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
        .offset((mi_row * (*cm).mi_params.mi_stride) as isize)
        .offset(mi_col as isize);
    let mut mbmi: *const MB_MODE_INFO = *mi.offset(0 as core::ffi::c_int as isize);
    if mbmi.is_null() {
        return TX_INVALID as core::ffi::c_int as TX_SIZE;
    }
    let ts: TX_SIZE = get_transform_size(
        xd,
        *mi.offset(0 as core::ffi::c_int as isize),
        mi_row,
        mi_col,
        plane,
        scale_horz as core::ffi::c_int,
        scale_vert as core::ffi::c_int,
    ) as TX_SIZE;
    let coord: uint32_t = if VERT_EDGE as core::ffi::c_int
        == edge_dir as core::ffi::c_int
    {
        x
    } else {
        y
    };
    let transform_masks: uint32_t = (if edge_dir as core::ffi::c_int
        == VERT_EDGE as core::ffi::c_int
    {
        tx_size_wide[ts as usize] - 1 as core::ffi::c_int
    } else {
        tx_size_high[ts as usize] - 1 as core::ffi::c_int
    }) as uint32_t;
    let tu_edge: int32_t = if coord & transform_masks != 0 {
        0 as int32_t
    } else {
        1 as int32_t
    };
    if tu_edge == 0 {
        return ts;
    }
    let curr_level: uint32_t = get_filter_level(
        cm,
        &(*cm).lf_info,
        edge_dir as core::ffi::c_int,
        plane,
        mbmi,
    ) as uint32_t;
    let curr_skipped: core::ffi::c_int = ((*mbmi).skip_txfm as core::ffi::c_int != 0
        && is_inter_block(mbmi) != 0) as core::ffi::c_int;
    let mut level: uint32_t = curr_level;
    if coord != 0 {
        let mi_prev: *const MB_MODE_INFO = *mi.offset(-(mode_step as isize));
        if mi_prev.is_null() {
            return TX_INVALID as core::ffi::c_int as TX_SIZE;
        }
        let pv_row: core::ffi::c_int = if VERT_EDGE as core::ffi::c_int
            == edge_dir as core::ffi::c_int
        {
            mi_row
        } else {
            mi_row - ((1 as core::ffi::c_int) << scale_vert)
        };
        let pv_col: core::ffi::c_int = if VERT_EDGE as core::ffi::c_int
            == edge_dir as core::ffi::c_int
        {
            mi_col - ((1 as core::ffi::c_int) << scale_horz)
        } else {
            mi_col
        };
        let pv_ts: TX_SIZE = get_transform_size(
            xd,
            mi_prev,
            pv_row,
            pv_col,
            plane,
            scale_horz as core::ffi::c_int,
            scale_vert as core::ffi::c_int,
        ) as TX_SIZE;
        let pv_lvl: uint32_t = get_filter_level(
            cm,
            &(*cm).lf_info,
            edge_dir as core::ffi::c_int,
            plane,
            mi_prev,
        ) as uint32_t;
        let pv_skip_txfm: core::ffi::c_int = ((*mi_prev).skip_txfm as core::ffi::c_int
            != 0 && is_inter_block(mi_prev) != 0) as core::ffi::c_int;
        let bsize: BLOCK_SIZE = get_plane_block_size(
            (*mbmi).bsize,
            (*plane_ptr).subsampling_x,
            (*plane_ptr).subsampling_y,
        ) as BLOCK_SIZE;
        if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"set_lpf_parameters\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                291 as core::ffi::c_int,
                b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let prediction_masks: core::ffi::c_int = if edge_dir as core::ffi::c_int
            == VERT_EDGE as core::ffi::c_int
        {
            block_size_wide[bsize as usize] as core::ffi::c_int - 1 as core::ffi::c_int
        } else {
            block_size_high[bsize as usize] as core::ffi::c_int - 1 as core::ffi::c_int
        };
        let pu_edge: int32_t = (coord & prediction_masks as uint32_t == 0)
            as core::ffi::c_int;
        if (curr_level != 0 || pv_lvl != 0)
            && (pv_skip_txfm == 0 || curr_skipped == 0 || pu_edge != 0)
        {
            let dim: core::ffi::c_int = if VERT_EDGE as core::ffi::c_int
                == edge_dir as core::ffi::c_int
            {
                if tx_size_wide_unit_log2[ts as usize]
                    < tx_size_wide_unit_log2[pv_ts as usize]
                {
                    tx_size_wide_unit_log2[ts as usize]
                } else {
                    tx_size_wide_unit_log2[pv_ts as usize]
                }
            } else if tx_size_high_unit_log2[ts as usize]
                < tx_size_high_unit_log2[pv_ts as usize]
            {
                tx_size_high_unit_log2[ts as usize]
            } else {
                tx_size_high_unit_log2[pv_ts as usize]
            };
            if plane != 0 {
                (*params).filter_length = (if dim == 0 as core::ffi::c_int {
                    4 as core::ffi::c_int
                } else {
                    6 as core::ffi::c_int
                }) as uint8_t;
            } else {
                if !(dim < TX_SIZES as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"set_lpf_parameters\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        308 as core::ffi::c_int,
                        b"dim < TX_SIZES\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                if !(dim >= 0 as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"set_lpf_parameters\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        309 as core::ffi::c_int,
                        b"dim >= 0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                (*params).filter_length = tx_dim_to_filter_length[dim as usize]
                    as uint8_t;
            }
            level = if curr_level != 0 { curr_level } else { pv_lvl };
        }
    }
    if (*params).filter_length != 0 {
        let limits: *const loop_filter_thresh = ((*cm).lf_info.lfthr)
            .as_ptr()
            .offset(level as isize);
        (*params).lfthr = limits;
    }
    return ts;
}
static mut vert_filter_length_luma: [[uint32_t; 19]; 19] = [
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
];
static mut horz_filter_length_luma: [[uint32_t; 19]; 19] = [
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        8 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
        14 as core::ffi::c_int as uint32_t,
    ],
];
static mut vert_filter_length_chroma: [[uint32_t; 19]; 19] = [
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
];
static mut horz_filter_length_chroma: [[uint32_t; 19]; 19] = [
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
    [
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        4 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
        6 as core::ffi::c_int as uint32_t,
    ],
];
#[inline(always)]
unsafe extern "C-unwind" fn set_one_param_for_line_luma(
    params: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_size: *mut TX_SIZE,
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    edge_dir: EDGE_DIR,
    mut mi_col: uint32_t,
    mut mi_row: uint32_t,
    plane_ptr: *const macroblockd_plane,
    mut coord: core::ffi::c_int,
    mut is_first_block: bool,
    mut prev_tx_size: TX_SIZE,
    mode_step: ptrdiff_t,
    mut min_dim: *mut core::ffi::c_int,
) {
    if !((mi_col << 2 as core::ffi::c_int) < (*plane_ptr).dst.width as uint32_t
        && (mi_row << 2 as core::ffi::c_int) < (*plane_ptr).dst.height as uint32_t)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"set_one_param_for_line_luma\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            655 as core::ffi::c_int,
            b"mi_col << MI_SIZE_LOG2 < (uint32_t)plane_ptr->dst.width && mi_row << MI_SIZE_LOG2 < (uint32_t)plane_ptr->dst.height\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let is_vert: core::ffi::c_int = (edge_dir as core::ffi::c_int
        == VERT_EDGE as core::ffi::c_int) as core::ffi::c_int;
    (*params).filter_length = 0 as uint8_t;
    let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
        .offset(mi_row.wrapping_mul((*cm).mi_params.mi_stride as uint32_t) as isize)
        .offset(mi_col as isize);
    let mut mbmi: *const MB_MODE_INFO = *mi.offset(0 as core::ffi::c_int as isize);
    if mbmi.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"set_one_param_for_line_luma\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            663 as core::ffi::c_int,
            b"mbmi\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let ts: TX_SIZE = get_transform_size(
        xd,
        *mi.offset(0 as core::ffi::c_int as isize),
        mi_row as core::ffi::c_int,
        mi_col as core::ffi::c_int,
        AOM_PLANE_Y,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ) as TX_SIZE;
    let transform_masks: uint32_t = (if is_vert != 0 {
        tx_size_wide[ts as usize] - 1 as core::ffi::c_int
    } else {
        tx_size_high[ts as usize] - 1 as core::ffi::c_int
    }) as uint32_t;
    let tu_edge: int32_t = if (coord * MI_SIZE) as uint32_t & transform_masks != 0 {
        0 as int32_t
    } else {
        1 as int32_t
    };
    if (tu_edge == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 28],
                [core::ffi::c_char; 28],
            >(*b"set_one_param_for_line_luma\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            672 as core::ffi::c_int,
            b"tu_edge\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut curr_skipped: bool = false_0 != 0;
    if !is_first_block || coord != 0 {
        let mi_prev: *const MB_MODE_INFO = *mi.offset(-(mode_step as isize));
        let pv_row: core::ffi::c_int = (if is_vert != 0 {
            mi_row
        } else {
            mi_row.wrapping_sub(1 as uint32_t)
        }) as core::ffi::c_int;
        let pv_col: core::ffi::c_int = (if is_vert != 0 {
            mi_col.wrapping_sub(1 as uint32_t)
        } else {
            mi_col
        }) as core::ffi::c_int;
        let pv_ts: TX_SIZE = (if is_first_block as core::ffi::c_int != 0 {
            get_transform_size(
                xd,
                mi_prev,
                pv_row,
                pv_col,
                AOM_PLANE_Y,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ) as core::ffi::c_int
        } else {
            prev_tx_size as core::ffi::c_int
        }) as TX_SIZE;
        if is_first_block {
            *min_dim = if is_vert != 0 {
                block_size_high[(*mi_prev).bsize as usize] as core::ffi::c_int
            } else {
                block_size_wide[(*mi_prev).bsize as usize] as core::ffi::c_int
            };
        }
        if mi_prev.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 28],
                    [core::ffi::c_char; 28],
                >(*b"set_one_param_for_line_luma\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                691 as core::ffi::c_int,
                b"mi_prev\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut level: uint8_t = get_filter_level(
            cm,
            &(*cm).lf_info,
            edge_dir as core::ffi::c_int,
            AOM_PLANE_Y,
            mbmi,
        );
        if level == 0 {
            level = get_filter_level(
                cm,
                &(*cm).lf_info,
                edge_dir as core::ffi::c_int,
                AOM_PLANE_Y,
                mi_prev,
            );
        }
        let pu_edge: int32_t = (mi_prev != mbmi) as core::ffi::c_int;
        if !(!(pu_edge == 0
            && ((*mbmi).bsize as core::ffi::c_int >= BLOCK_8X16 as core::ffi::c_int
                && (*mbmi).bsize as core::ffi::c_int <= BLOCK_16X16 as core::ffi::c_int))
            || pv_ts as core::ffi::c_int == ts as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 28],
                    [core::ffi::c_char; 28],
                >(*b"set_one_param_for_line_luma\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                705 as core::ffi::c_int,
                b"IMPLIES( !pu_edge && (mbmi->bsize >= BLOCK_8X16 && mbmi->bsize <= BLOCK_16X16), pv_ts == ts)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if pu_edge == 0 {
            curr_skipped = (*mbmi).skip_txfm as core::ffi::c_int != 0
                && is_inter_block(mbmi) != 0;
        }
        if (pu_edge != 0 || !curr_skipped) && level as core::ffi::c_int != 0 {
            (*params).filter_length = (if is_vert != 0 {
                vert_filter_length_luma[ts as usize][pv_ts as usize]
            } else {
                horz_filter_length_luma[ts as usize][pv_ts as usize]
            }) as uint8_t;
            let limits: *const loop_filter_thresh = ((*cm).lf_info.lfthr)
                .as_ptr()
                .offset(level as core::ffi::c_int as isize);
            (*params).lfthr = limits;
        }
    }
    let block_dim: core::ffi::c_int = if is_vert != 0 {
        block_size_high[(*mbmi).bsize as usize] as core::ffi::c_int
    } else {
        block_size_wide[(*mbmi).bsize as usize] as core::ffi::c_int
    };
    *min_dim = if *min_dim < block_dim { *min_dim } else { block_dim };
    *tx_size = ts;
}
#[inline(always)]
unsafe extern "C-unwind" fn set_lpf_parameters_for_line_luma(
    params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    edge_dir: EDGE_DIR,
    mut mi_col: uint32_t,
    mut mi_row: uint32_t,
    plane_ptr: *const macroblockd_plane,
    mi_range: uint32_t,
    mode_step: ptrdiff_t,
    mut min_dim: *mut core::ffi::c_int,
) {
    let is_vert: core::ffi::c_int = (edge_dir as core::ffi::c_int
        == VERT_EDGE as core::ffi::c_int) as core::ffi::c_int;
    let mut params: *mut AV1_DEBLOCKING_PARAMETERS = params_buf;
    let mut tx_size: *mut TX_SIZE = tx_buf;
    let mut counter_ptr: *mut uint32_t = if is_vert != 0 {
        &raw mut mi_col
    } else {
        &raw mut mi_row
    };
    let mut prev_tx_size: TX_SIZE = TX_INVALID as core::ffi::c_int as TX_SIZE;
    set_one_param_for_line_luma(
        params,
        tx_size,
        cm,
        xd,
        edge_dir,
        mi_col,
        mi_row,
        plane_ptr,
        *counter_ptr as core::ffi::c_int,
        true_0 != 0,
        prev_tx_size,
        mode_step,
        min_dim,
    );
    let mut advance_units: core::ffi::c_int = if is_vert != 0 {
        tx_size_wide_unit[*tx_size as usize]
    } else {
        tx_size_high_unit[*tx_size as usize]
    };
    prev_tx_size = *tx_size;
    *counter_ptr = (*counter_ptr).wrapping_add(advance_units as uint32_t);
    params = params.offset(advance_units as isize);
    tx_size = tx_size.offset(advance_units as isize);
    while *counter_ptr < mi_range {
        set_one_param_for_line_luma(
            params,
            tx_size,
            cm,
            xd,
            edge_dir,
            mi_col,
            mi_row,
            plane_ptr,
            *counter_ptr as core::ffi::c_int,
            false_0 != 0,
            prev_tx_size,
            mode_step,
            min_dim,
        );
        advance_units = if is_vert != 0 {
            tx_size_wide_unit[*tx_size as usize]
        } else {
            tx_size_high_unit[*tx_size as usize]
        };
        prev_tx_size = *tx_size;
        *counter_ptr = (*counter_ptr).wrapping_add(advance_units as uint32_t);
        params = params.offset(advance_units as isize);
        tx_size = tx_size.offset(advance_units as isize);
    }
}
#[inline(always)]
unsafe extern "C-unwind" fn set_one_param_for_line_chroma(
    params: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_size: *mut TX_SIZE,
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    edge_dir: EDGE_DIR,
    mut mi_col: uint32_t,
    mut mi_row: uint32_t,
    mut coord: core::ffi::c_int,
    mut is_first_block: bool,
    mut prev_tx_size: TX_SIZE,
    plane_ptr: *const macroblockd_plane,
    mode_step: ptrdiff_t,
    scale_horz: core::ffi::c_int,
    scale_vert: core::ffi::c_int,
    mut min_dim: *mut core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut joint_filter_chroma: core::ffi::c_int,
) {
    let is_vert: core::ffi::c_int = (edge_dir as core::ffi::c_int
        == VERT_EDGE as core::ffi::c_int) as core::ffi::c_int;
    if !((mi_col << 2 as core::ffi::c_int)
        < ((*plane_ptr).dst.width << scale_horz) as uint32_t
        && (mi_row << 2 as core::ffi::c_int)
            < ((*plane_ptr).dst.height << scale_vert) as uint32_t) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"set_one_param_for_line_chroma\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            782 as core::ffi::c_int,
            b"(mi_col << MI_SIZE_LOG2) < (uint32_t)(plane_ptr->dst.width << scale_horz) && (mi_row << MI_SIZE_LOG2) < (uint32_t)(plane_ptr->dst.height << scale_vert)\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*params).filter_length = 0 as uint8_t;
    mi_row |= scale_vert as uint32_t;
    mi_col |= scale_horz as uint32_t;
    let mut mi: *mut *mut MB_MODE_INFO = ((*cm).mi_params.mi_grid_base)
        .offset(mi_row.wrapping_mul((*cm).mi_params.mi_stride as uint32_t) as isize)
        .offset(mi_col as isize);
    let mut mbmi: *const MB_MODE_INFO = *mi.offset(0 as core::ffi::c_int as isize);
    if mbmi.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"set_one_param_for_line_chroma\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            795 as core::ffi::c_int,
            b"mbmi\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let ts: TX_SIZE = get_transform_size(
        xd,
        *mi.offset(0 as core::ffi::c_int as isize),
        mi_row as core::ffi::c_int,
        mi_col as core::ffi::c_int,
        plane,
        scale_horz,
        scale_vert,
    ) as TX_SIZE;
    *tx_size = ts;
    let transform_masks: uint32_t = (if is_vert != 0 {
        tx_size_wide[ts as usize] - 1 as core::ffi::c_int
    } else {
        tx_size_high[ts as usize] - 1 as core::ffi::c_int
    }) as uint32_t;
    let tu_edge: int32_t = if (coord * MI_SIZE) as uint32_t & transform_masks != 0 {
        0 as int32_t
    } else {
        1 as int32_t
    };
    if (tu_edge == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"set_one_param_for_line_chroma\0"))
                .as_ptr(),
            b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
            805 as core::ffi::c_int,
            b"tu_edge\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut curr_skipped: bool = false_0 != 0;
    if !is_first_block || coord != 0 {
        let mi_prev: *const MB_MODE_INFO = *mi.offset(-(mode_step as isize));
        if mi_prev.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 30],
                    [core::ffi::c_char; 30],
                >(*b"set_one_param_for_line_chroma\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                815 as core::ffi::c_int,
                b"mi_prev\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let pv_row: core::ffi::c_int = (if is_vert != 0 {
            mi_row
        } else {
            mi_row.wrapping_sub(((1 as core::ffi::c_int) << scale_vert) as uint32_t)
        }) as core::ffi::c_int;
        let pv_col: core::ffi::c_int = (if is_vert != 0 {
            mi_col.wrapping_sub(((1 as core::ffi::c_int) << scale_horz) as uint32_t)
        } else {
            mi_col
        }) as core::ffi::c_int;
        let pv_ts: TX_SIZE = (if is_first_block as core::ffi::c_int != 0 {
            get_transform_size(
                xd,
                mi_prev,
                pv_row,
                pv_col,
                plane,
                scale_horz,
                scale_vert,
            ) as core::ffi::c_int
        } else {
            prev_tx_size as core::ffi::c_int
        }) as TX_SIZE;
        if is_first_block {
            *min_dim = if is_vert != 0 {
                tx_size_high[pv_ts as usize]
            } else {
                tx_size_wide[pv_ts as usize]
            };
        }
        let mut level: uint8_t = get_filter_level(
            cm,
            &(*cm).lf_info,
            edge_dir as core::ffi::c_int,
            plane,
            mbmi,
        );
        if level == 0 {
            level = get_filter_level(
                cm,
                &(*cm).lf_info,
                edge_dir as core::ffi::c_int,
                plane,
                mi_prev,
            );
        }
        if joint_filter_chroma != 0 {
            let mut v_level: uint8_t = get_filter_level(
                cm,
                &(*cm).lf_info,
                edge_dir as core::ffi::c_int,
                AOM_PLANE_V,
                mbmi,
            );
            if v_level == 0 {
                v_level = get_filter_level(
                    cm,
                    &(*cm).lf_info,
                    edge_dir as core::ffi::c_int,
                    AOM_PLANE_V,
                    mi_prev,
                );
            }
            if !(level as core::ffi::c_int == v_level as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 30],
                        [core::ffi::c_char; 30],
                    >(*b"set_one_param_for_line_chroma\0"))
                        .as_ptr(),
                    b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                    838 as core::ffi::c_int,
                    b"level == v_level\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
        let pu_edge: int32_t = (mi_prev != mbmi) as core::ffi::c_int;
        if pu_edge == 0 {
            curr_skipped = (*mbmi).skip_txfm as core::ffi::c_int != 0
                && is_inter_block(mbmi) != 0;
        }
        if (!curr_skipped || pu_edge != 0) && level as core::ffi::c_int != 0 {
            (*params).filter_length = (if is_vert != 0 {
                vert_filter_length_chroma[ts as usize][pv_ts as usize]
            } else {
                horz_filter_length_chroma[ts as usize][pv_ts as usize]
            }) as uint8_t;
            let limits: *const loop_filter_thresh = ((*cm).lf_info.lfthr).as_ptr();
            (*params).lfthr = limits.offset(level as core::ffi::c_int as isize);
        }
    }
    let tx_dim: core::ffi::c_int = if is_vert != 0 {
        tx_size_high[ts as usize]
    } else {
        tx_size_wide[ts as usize]
    };
    *min_dim = if *min_dim < tx_dim { *min_dim } else { tx_dim };
}
#[inline(always)]
unsafe extern "C-unwind" fn set_lpf_parameters_for_line_chroma(
    params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    edge_dir: EDGE_DIR,
    mut mi_col: uint32_t,
    mut mi_row: uint32_t,
    plane_ptr: *const macroblockd_plane,
    mi_range: uint32_t,
    mode_step: ptrdiff_t,
    scale_horz: core::ffi::c_int,
    scale_vert: core::ffi::c_int,
    mut min_dim: *mut core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut joint_filter_chroma: core::ffi::c_int,
) {
    let is_vert: core::ffi::c_int = (edge_dir as core::ffi::c_int
        == VERT_EDGE as core::ffi::c_int) as core::ffi::c_int;
    let mut params: *mut AV1_DEBLOCKING_PARAMETERS = params_buf;
    let mut tx_size: *mut TX_SIZE = tx_buf;
    let mut counter_ptr: *mut uint32_t = if is_vert != 0 {
        &raw mut mi_col
    } else {
        &raw mut mi_row
    };
    let scale: uint32_t = (if is_vert != 0 { scale_horz } else { scale_vert })
        as uint32_t;
    let mut prev_tx_size: TX_SIZE = TX_INVALID as core::ffi::c_int as TX_SIZE;
    set_one_param_for_line_chroma(
        params,
        tx_size,
        cm,
        xd,
        edge_dir,
        mi_col,
        mi_row,
        *counter_ptr as core::ffi::c_int,
        true_0 != 0,
        prev_tx_size,
        plane_ptr,
        mode_step,
        scale_horz,
        scale_vert,
        min_dim,
        plane,
        joint_filter_chroma,
    );
    let mut advance_units: core::ffi::c_int = if is_vert != 0 {
        tx_size_wide_unit[*tx_size as usize]
    } else {
        tx_size_high_unit[*tx_size as usize]
    };
    prev_tx_size = *tx_size;
    *counter_ptr = (*counter_ptr).wrapping_add((advance_units << scale) as uint32_t);
    params = params.offset(advance_units as isize);
    tx_size = tx_size.offset(advance_units as isize);
    while *counter_ptr < mi_range {
        set_one_param_for_line_chroma(
            params,
            tx_size,
            cm,
            xd,
            edge_dir,
            mi_col,
            mi_row,
            *counter_ptr as core::ffi::c_int,
            false_0 != 0,
            prev_tx_size,
            plane_ptr,
            mode_step,
            scale_horz,
            scale_vert,
            min_dim,
            plane,
            joint_filter_chroma,
        );
        advance_units = if is_vert != 0 {
            tx_size_wide_unit[*tx_size as usize]
        } else {
            tx_size_high_unit[*tx_size as usize]
        };
        prev_tx_size = *tx_size;
        *counter_ptr = (*counter_ptr).wrapping_add((advance_units << scale) as uint32_t);
        params = params.offset(advance_units as isize);
        tx_size = tx_size.offset(advance_units as isize);
    }
}
#[inline]
unsafe extern "C-unwind" fn filter_vert(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut params: *const AV1_DEBLOCKING_PARAMETERS,
    mut seq_params: *const SequenceHeader,
    mut use_filter_type: USE_FILTER_TYPE,
) {
    let mut limits: *const loop_filter_thresh = (*params).lfthr;
    let use_highbitdepth: core::ffi::c_int = (*seq_params).use_highbitdepth
        as core::ffi::c_int;
    let bit_depth: aom_bit_depth_t = (*seq_params).bit_depth;
    if use_highbitdepth != 0 {
        let mut dst_shortptr: *mut uint16_t = ((dst as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 => {
                    aom_highbd_lpf_vertical_8_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_8_dual.expect("non-null function pointer")(
                        dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                14 => {
                    aom_highbd_lpf_vertical_14_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_14_dual.expect("non-null function pointer")(
                        dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                _ => {}
            }
        } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 => {
                    aom_highbd_lpf_vertical_8_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                14 => {
                    aom_highbd_lpf_vertical_14_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                _ => {}
            }
        } else {
            if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 12],
                        [core::ffi::c_char; 12],
                    >(*b"filter_vert\0"))
                        .as_ptr(),
                    b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                    997 as core::ffi::c_int,
                    b"use_filter_type == USE_SINGLE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_vertical_4.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_vertical_6.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 => {
                    aom_highbd_lpf_vertical_8.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                14 => {
                    aom_highbd_lpf_vertical_14.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                _ => {}
            }
        }
        return;
    }
    if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_vertical_4_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_vertical_6_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            8 => {
                aom_lpf_vertical_8_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            14 => {
                aom_lpf_vertical_14_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            _ => {}
        }
    } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_vertical_4_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_vertical_6_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            8 => {
                aom_lpf_vertical_8_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            14 => {
                aom_lpf_vertical_14_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            _ => {}
        }
    } else {
        if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"filter_vert\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                1082 as core::ffi::c_int,
                b"use_filter_type == USE_SINGLE\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_vertical_4.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_vertical_6.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            8 => {
                aom_lpf_vertical_8.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            14 => {
                aom_lpf_vertical_14.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            _ => {}
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn filter_vert_chroma(
    mut u_dst: *mut uint8_t,
    mut v_dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut params: *const AV1_DEBLOCKING_PARAMETERS,
    mut seq_params: *const SequenceHeader,
    mut use_filter_type: USE_FILTER_TYPE,
) {
    let mut u_limits: *const loop_filter_thresh = (*params).lfthr;
    let mut v_limits: *const loop_filter_thresh = (*params).lfthr;
    let use_highbitdepth: core::ffi::c_int = (*seq_params).use_highbitdepth
        as core::ffi::c_int;
    let bit_depth: aom_bit_depth_t = (*seq_params).bit_depth;
    if use_highbitdepth != 0 {
        let mut u_dst_shortptr: *mut uint16_t = ((u_dst as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        let mut v_dst_shortptr: *mut uint16_t = ((v_dst as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        u_dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        v_dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        u_dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        v_dst_shortptr
                            .offset(
                                (2 as core::ffi::c_int * MI_SIZE * dst_stride) as isize,
                            ),
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 | 14 => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"filter_vert_chroma\0"))
                                .as_ptr(),
                            b"av1_loopfilter.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1165 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                _ => {}
            }
        } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_4_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_6_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 | 14 => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"filter_vert_chroma\0"))
                                .as_ptr(),
                            b"av1_loopfilter.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1193 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                _ => {}
            }
        } else {
            if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"filter_vert_chroma\0"))
                        .as_ptr(),
                    b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                    1198 as core::ffi::c_int,
                    b"use_filter_type == USE_SINGLE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_vertical_4.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_4.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_vertical_6.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_vertical_6.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 | 14 => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"filter_vert_chroma\0"))
                                .as_ptr(),
                            b"av1_loopfilter.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1218 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                _ => {}
            }
        }
        return;
    }
    if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_vertical_4_quad.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_vertical_4_quad.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_vertical_6_quad.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_vertical_6_quad.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            8 | 14 => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"filter_vert_chroma\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        1246 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            _ => {}
        }
    } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_vertical_4_dual.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_vertical_4_dual.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_vertical_6_dual.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_vertical_6_dual.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            8 | 14 => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"filter_vert_chroma\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        1274 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            _ => {}
        }
    } else {
        if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"filter_vert_chroma\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                1279 as core::ffi::c_int,
                b"use_filter_type == USE_SINGLE\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_vertical_4.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_vertical_4.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_vertical_6.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_vertical_6.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            8 | 14 => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"filter_vert_chroma\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        1295 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_block_plane_vert(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    plane: core::ffi::c_int,
    plane_ptr: *const MACROBLOCKD_PLANE,
    mi_row: uint32_t,
    mi_col: uint32_t,
) {
    let scale_horz: uint32_t = (*plane_ptr).subsampling_x as uint32_t;
    let scale_vert: uint32_t = (*plane_ptr).subsampling_y as uint32_t;
    let dst_ptr: *mut uint8_t = (*plane_ptr).dst.buf;
    let dst_stride: core::ffi::c_int = (*plane_ptr).dst.stride;
    let plane_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << scale_vert >> 1 as core::ffi::c_int) >> scale_vert;
    let plane_mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << scale_horz >> 1 as core::ffi::c_int) >> scale_horz;
    let y_range: core::ffi::c_int = if ((plane_mi_rows as uint32_t)
        .wrapping_sub(mi_row >> scale_vert) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_vert
    {
        (plane_mi_rows as uint32_t).wrapping_sub(mi_row >> scale_vert)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_vert
    };
    let x_range: core::ffi::c_int = if ((plane_mi_cols as uint32_t)
        .wrapping_sub(mi_col >> scale_horz) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    {
        (plane_mi_cols as uint32_t).wrapping_sub(mi_col >> scale_horz)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    };
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < y_range {
        let mut p: *mut uint8_t = dst_ptr.offset((y * MI_SIZE * dst_stride) as isize);
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < x_range {
            let curr_x: uint32_t = (mi_col.wrapping_mul(MI_SIZE as uint32_t)
                >> scale_horz)
                .wrapping_add((x * MI_SIZE) as uint32_t);
            let curr_y: uint32_t = (mi_row.wrapping_mul(MI_SIZE as uint32_t)
                >> scale_vert)
                .wrapping_add((y * MI_SIZE) as uint32_t);
            let mut advance_units: uint32_t = 0;
            let mut tx_size: TX_SIZE = 0;
            let mut params: AV1_DEBLOCKING_PARAMETERS = AV1_DEBLOCKING_PARAMETERS {
                filter_length: 0,
                lfthr: 0 as *const loop_filter_thresh,
            };
            memset(
                &raw mut params as *mut AV1_DEBLOCKING_PARAMETERS as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<AV1_DEBLOCKING_PARAMETERS>() as size_t,
            );
            tx_size = set_lpf_parameters(
                &raw mut params,
                (1 as core::ffi::c_int as ptrdiff_t) << scale_horz,
                cm,
                xd,
                VERT_EDGE as core::ffi::c_int as EDGE_DIR,
                curr_x,
                curr_y,
                plane,
                plane_ptr as *const macroblockd_plane,
            );
            if tx_size as core::ffi::c_int == TX_INVALID as core::ffi::c_int {
                params.filter_length = 0 as uint8_t;
                tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            }
            filter_vert(
                p,
                dst_stride,
                &raw mut params,
                (*cm).seq_params,
                USE_SINGLE as core::ffi::c_int as USE_FILTER_TYPE,
            );
            advance_units = tx_size_wide_unit[tx_size as usize] as uint32_t;
            x = (x as uint32_t).wrapping_add(advance_units) as core::ffi::c_int
                as core::ffi::c_int;
            p = p.offset(advance_units.wrapping_mul(MI_SIZE as uint32_t) as isize);
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_block_plane_vert_opt(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    plane_ptr: *const MACROBLOCKD_PLANE,
    mi_row: uint32_t,
    mi_col: uint32_t,
    mut params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let dst_ptr: *mut uint8_t = (*plane_ptr).dst.buf;
    let dst_stride: core::ffi::c_int = (*plane_ptr).dst.stride;
    let plane_mi_cols: core::ffi::c_int = (*plane_ptr).dst.width
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) - 1 as core::ffi::c_int
        >> 2 as core::ffi::c_int;
    let plane_mi_rows: core::ffi::c_int = (*plane_ptr).dst.height
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) - 1 as core::ffi::c_int
        >> 2 as core::ffi::c_int;
    let y_range: core::ffi::c_int = if ((plane_mi_rows as uint32_t).wrapping_sub(mi_row)
        as core::ffi::c_int) < (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2
    {
        (plane_mi_rows as uint32_t).wrapping_sub(mi_row) as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2
    };
    let x_range: core::ffi::c_int = if ((plane_mi_cols as uint32_t).wrapping_sub(mi_col)
        as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
    {
        (plane_mi_cols as uint32_t).wrapping_sub(mi_col) as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
    };
    let mode_step: ptrdiff_t = 1 as ptrdiff_t;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < y_range {
        let curr_y: uint32_t = mi_row.wrapping_add(y as uint32_t);
        let x_start: uint32_t = mi_col;
        let x_end: uint32_t = mi_col.wrapping_add(x_range as uint32_t);
        let mut min_block_height: core::ffi::c_int = block_size_high[BLOCK_128X128
            as core::ffi::c_int as usize] as core::ffi::c_int;
        set_lpf_parameters_for_line_luma(
            params_buf,
            tx_buf,
            cm,
            xd,
            VERT_EDGE as core::ffi::c_int as EDGE_DIR,
            x_start,
            curr_y,
            plane_ptr as *const macroblockd_plane,
            x_end,
            mode_step,
            &raw mut min_block_height,
        );
        let mut params: *mut AV1_DEBLOCKING_PARAMETERS = params_buf;
        let mut tx_size: *mut TX_SIZE = tx_buf;
        let mut use_filter_type: USE_FILTER_TYPE = USE_SINGLE as core::ffi::c_int
            as USE_FILTER_TYPE;
        let mut p: *mut uint8_t = dst_ptr.offset((y * MI_SIZE * dst_stride) as isize);
        if y & 3 as core::ffi::c_int == 0 as core::ffi::c_int
            && (y + 3 as core::ffi::c_int) < y_range
            && min_block_height >= 16 as core::ffi::c_int
        {
            use_filter_type = USE_QUAD as core::ffi::c_int as USE_FILTER_TYPE;
            y += 3 as core::ffi::c_int;
        } else if (y + 1 as core::ffi::c_int) < y_range
            && min_block_height >= 8 as core::ffi::c_int
        {
            use_filter_type = USE_DUAL as core::ffi::c_int as USE_FILTER_TYPE;
            y += 1 as core::ffi::c_int;
        }
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < x_range {
            if *tx_size as core::ffi::c_int == TX_INVALID as core::ffi::c_int {
                (*params).filter_length = 0 as uint8_t;
                *tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            }
            filter_vert(p, dst_stride, params, (*cm).seq_params, use_filter_type);
            let advance_units: uint32_t = tx_size_wide_unit[*tx_size as usize]
                as uint32_t;
            x = (x as uint32_t).wrapping_add(advance_units) as core::ffi::c_int
                as core::ffi::c_int;
            p = p.offset(advance_units.wrapping_mul(MI_SIZE as uint32_t) as isize);
            params = params.offset(advance_units as isize);
            tx_size = tx_size.offset(advance_units as isize);
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_block_plane_vert_opt_chroma(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    plane_ptr: *const MACROBLOCKD_PLANE,
    mi_row: uint32_t,
    mi_col: uint32_t,
    mut params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    mut plane: core::ffi::c_int,
    mut joint_filter_chroma: bool,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let scale_horz: uint32_t = (*plane_ptr).subsampling_x as uint32_t;
    let scale_vert: uint32_t = (*plane_ptr).subsampling_y as uint32_t;
    let dst_stride: core::ffi::c_int = (*plane_ptr).dst.stride;
    let mi_cols: core::ffi::c_int = ((*plane_ptr).dst.width << scale_horz) + MI_SIZE
        - 1 as core::ffi::c_int >> MI_SIZE_LOG2;
    let mi_rows: core::ffi::c_int = ((*plane_ptr).dst.height << scale_vert) + MI_SIZE
        - 1 as core::ffi::c_int >> MI_SIZE_LOG2;
    let plane_mi_rows: core::ffi::c_int = mi_rows
        + ((1 as core::ffi::c_int) << scale_vert >> 1 as core::ffi::c_int) >> scale_vert;
    let plane_mi_cols: core::ffi::c_int = mi_cols
        + ((1 as core::ffi::c_int) << scale_horz >> 1 as core::ffi::c_int) >> scale_horz;
    let y_range: core::ffi::c_int = if ((plane_mi_rows as uint32_t)
        .wrapping_sub(mi_row >> scale_vert) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2 >> scale_vert
    {
        (plane_mi_rows as uint32_t).wrapping_sub(mi_row >> scale_vert)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2 >> scale_vert
    };
    let x_range: core::ffi::c_int = if ((plane_mi_cols as uint32_t)
        .wrapping_sub(mi_col >> scale_horz) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    {
        (plane_mi_cols as uint32_t).wrapping_sub(mi_col >> scale_horz)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    };
    let mode_step: ptrdiff_t = (1 as core::ffi::c_int as ptrdiff_t) << scale_horz;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < y_range {
        let curr_y: uint32_t = mi_row.wrapping_add((y << scale_vert) as uint32_t);
        let x_start: uint32_t = mi_col
            .wrapping_add(((0 as core::ffi::c_int) << scale_horz) as uint32_t);
        let x_end: uint32_t = mi_col.wrapping_add((x_range << scale_horz) as uint32_t);
        let mut min_height: core::ffi::c_int = tx_size_high[TX_64X64 as core::ffi::c_int
            as usize];
        set_lpf_parameters_for_line_chroma(
            params_buf,
            tx_buf,
            cm,
            xd,
            VERT_EDGE as core::ffi::c_int as EDGE_DIR,
            x_start,
            curr_y,
            plane_ptr as *const macroblockd_plane,
            x_end,
            mode_step,
            scale_horz as core::ffi::c_int,
            scale_vert as core::ffi::c_int,
            &raw mut min_height,
            plane,
            joint_filter_chroma as core::ffi::c_int,
        );
        let mut params: *mut AV1_DEBLOCKING_PARAMETERS = params_buf;
        let mut tx_size: *mut TX_SIZE = tx_buf;
        let mut use_filter_type: core::ffi::c_int = USE_SINGLE as core::ffi::c_int;
        let mut y_inc: core::ffi::c_int = 0 as core::ffi::c_int;
        if y & 3 as core::ffi::c_int == 0 as core::ffi::c_int
            && (y + 3 as core::ffi::c_int) < y_range
            && min_height >= 16 as core::ffi::c_int
        {
            use_filter_type = USE_QUAD as core::ffi::c_int;
            y_inc = 3 as core::ffi::c_int;
        } else if y % 2 as core::ffi::c_int == 0 as core::ffi::c_int
            && (y + 1 as core::ffi::c_int) < y_range
            && min_height >= 8 as core::ffi::c_int
        {
            use_filter_type = USE_DUAL as core::ffi::c_int;
            y_inc = 1 as core::ffi::c_int;
        }
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < x_range {
            if *tx_size as core::ffi::c_int == TX_INVALID as core::ffi::c_int {
                (*params).filter_length = 0 as uint8_t;
                *tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            }
            let offset: core::ffi::c_int = y * MI_SIZE * dst_stride + x * MI_SIZE;
            if joint_filter_chroma {
                let mut u_dst: *mut uint8_t = ((*plane_ptr
                    .offset(0 as core::ffi::c_int as isize))
                    .dst
                    .buf)
                    .offset(offset as isize);
                let mut v_dst: *mut uint8_t = ((*plane_ptr
                    .offset(1 as core::ffi::c_int as isize))
                    .dst
                    .buf)
                    .offset(offset as isize);
                filter_vert_chroma(
                    u_dst,
                    v_dst,
                    dst_stride,
                    params,
                    (*cm).seq_params,
                    use_filter_type as USE_FILTER_TYPE,
                );
            } else {
                let mut dst_ptr: *mut uint8_t = ((*plane_ptr).dst.buf)
                    .offset(offset as isize);
                filter_vert(
                    dst_ptr,
                    dst_stride,
                    params,
                    (*cm).seq_params,
                    use_filter_type as USE_FILTER_TYPE,
                );
            }
            let advance_units: uint32_t = tx_size_wide_unit[*tx_size as usize]
                as uint32_t;
            x = (x as uint32_t).wrapping_add(advance_units) as core::ffi::c_int
                as core::ffi::c_int;
            params = params.offset(advance_units as isize);
            tx_size = tx_size.offset(advance_units as isize);
        }
        y += y_inc;
        y += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn filter_horz(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut params: *const AV1_DEBLOCKING_PARAMETERS,
    mut seq_params: *const SequenceHeader,
    mut use_filter_type: USE_FILTER_TYPE,
) {
    let mut limits: *const loop_filter_thresh = (*params).lfthr;
    let use_highbitdepth: core::ffi::c_int = (*seq_params).use_highbitdepth
        as core::ffi::c_int;
    let bit_depth: aom_bit_depth_t = (*seq_params).bit_depth;
    if use_highbitdepth != 0 {
        let mut dst_shortptr: *mut uint16_t = ((dst as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        dst_shortptr.offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        dst_shortptr.offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 => {
                    aom_highbd_lpf_horizontal_8_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_8_dual.expect("non-null function pointer")(
                        dst_shortptr.offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                14 => {
                    aom_highbd_lpf_horizontal_14_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_14_dual.expect("non-null function pointer")(
                        dst_shortptr.offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                _ => {}
            }
        } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 => {
                    aom_highbd_lpf_horizontal_8_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                14 => {
                    aom_highbd_lpf_horizontal_14_dual.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                _ => {}
            }
        } else {
            if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 12],
                        [core::ffi::c_char; 12],
                    >(*b"filter_horz\0"))
                        .as_ptr(),
                    b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                    1599 as core::ffi::c_int,
                    b"use_filter_type == USE_SINGLE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_horizontal_4.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_horizontal_6.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 => {
                    aom_highbd_lpf_horizontal_8.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                14 => {
                    aom_highbd_lpf_horizontal_14.expect("non-null function pointer")(
                        dst_shortptr,
                        dst_stride,
                        ((*limits).mblim).as_ptr(),
                        ((*limits).lim).as_ptr(),
                        ((*limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                _ => {}
            }
        }
        return;
    }
    if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_horizontal_4_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_horizontal_6_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            8 => {
                aom_lpf_horizontal_8_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            14 => {
                aom_lpf_horizontal_14_quad.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            _ => {}
        }
    } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_horizontal_4_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_horizontal_6_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            8 => {
                aom_lpf_horizontal_8_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            14 => {
                aom_lpf_horizontal_14_dual.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            _ => {}
        }
    } else {
        if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 12],
                    [core::ffi::c_char; 12],
                >(*b"filter_horz\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                1684 as core::ffi::c_int,
                b"use_filter_type == USE_SINGLE\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_horizontal_4.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_horizontal_6.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            8 => {
                aom_lpf_horizontal_8.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            14 => {
                aom_lpf_horizontal_14.expect("non-null function pointer")(
                    dst,
                    dst_stride,
                    ((*limits).mblim).as_ptr(),
                    ((*limits).lim).as_ptr(),
                    ((*limits).hev_thr).as_ptr(),
                );
            }
            _ => {}
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn filter_horz_chroma(
    mut u_dst: *mut uint8_t,
    mut v_dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut params: *const AV1_DEBLOCKING_PARAMETERS,
    mut seq_params: *const SequenceHeader,
    mut use_filter_type: USE_FILTER_TYPE,
) {
    let mut u_limits: *const loop_filter_thresh = (*params).lfthr;
    let mut v_limits: *const loop_filter_thresh = (*params).lfthr;
    let use_highbitdepth: core::ffi::c_int = (*seq_params).use_highbitdepth
        as core::ffi::c_int;
    let bit_depth: aom_bit_depth_t = (*seq_params).bit_depth;
    if use_highbitdepth != 0 {
        let mut u_dst_shortptr: *mut uint16_t = ((u_dst as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        let mut v_dst_shortptr: *mut uint16_t = ((v_dst as uintptr_t)
            << 1 as core::ffi::c_int) as *mut uint16_t;
        if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        u_dst_shortptr
                            .offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        v_dst_shortptr
                            .offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        u_dst_shortptr
                            .offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        v_dst_shortptr
                            .offset((2 as core::ffi::c_int * MI_SIZE) as isize),
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 | 14 => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"filter_horz_chroma\0"))
                                .as_ptr(),
                            b"av1_loopfilter.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1767 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                _ => {}
            }
        } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_4_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_6_dual.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 | 14 => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"filter_horz_chroma\0"))
                                .as_ptr(),
                            b"av1_loopfilter.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1795 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                _ => {}
            }
        } else {
            if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
                as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 19],
                        [core::ffi::c_char; 19],
                    >(*b"filter_horz_chroma\0"))
                        .as_ptr(),
                    b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                    1800 as core::ffi::c_int,
                    b"use_filter_type == USE_SINGLE\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            match (*params).filter_length as core::ffi::c_int {
                4 => {
                    aom_highbd_lpf_horizontal_4.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_4.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                6 => {
                    aom_highbd_lpf_horizontal_6.expect("non-null function pointer")(
                        u_dst_shortptr,
                        dst_stride,
                        ((*u_limits).mblim).as_ptr(),
                        ((*u_limits).lim).as_ptr(),
                        ((*u_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                    aom_highbd_lpf_horizontal_6.expect("non-null function pointer")(
                        v_dst_shortptr,
                        dst_stride,
                        ((*v_limits).mblim).as_ptr(),
                        ((*v_limits).lim).as_ptr(),
                        ((*v_limits).hev_thr).as_ptr(),
                        bit_depth as core::ffi::c_int,
                    );
                }
                8 | 14 => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 19],
                                [core::ffi::c_char; 19],
                            >(*b"filter_horz_chroma\0"))
                                .as_ptr(),
                            b"av1_loopfilter.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1820 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
                _ => {}
            }
        }
        return;
    }
    if use_filter_type as core::ffi::c_int == USE_QUAD as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_horizontal_4_quad.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_horizontal_4_quad.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_horizontal_6_quad.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_horizontal_6_quad.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            8 | 14 => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"filter_horz_chroma\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        1848 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            _ => {}
        }
    } else if use_filter_type as core::ffi::c_int == USE_DUAL as core::ffi::c_int {
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_horizontal_4_dual.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_horizontal_4_dual.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_horizontal_6_dual.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_horizontal_6_dual.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            8 | 14 => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"filter_horz_chroma\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        1876 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            _ => {}
        }
    } else {
        if !(use_filter_type as core::ffi::c_int == USE_SINGLE as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 19],
                    [core::ffi::c_char; 19],
                >(*b"filter_horz_chroma\0"))
                    .as_ptr(),
                b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                1881 as core::ffi::c_int,
                b"use_filter_type == USE_SINGLE\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        match (*params).filter_length as core::ffi::c_int {
            4 => {
                aom_lpf_horizontal_4.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_horizontal_4.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
            }
            6 => {
                aom_lpf_horizontal_6.expect("non-null function pointer")(
                    u_dst,
                    dst_stride,
                    ((*u_limits).mblim).as_ptr(),
                    ((*u_limits).lim).as_ptr(),
                    ((*u_limits).hev_thr).as_ptr(),
                );
                aom_lpf_horizontal_6.expect("non-null function pointer")(
                    v_dst,
                    dst_stride,
                    ((*v_limits).mblim).as_ptr(),
                    ((*v_limits).lim).as_ptr(),
                    ((*v_limits).hev_thr).as_ptr(),
                );
            }
            8 | 14 => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 19],
                            [core::ffi::c_char; 19],
                        >(*b"filter_horz_chroma\0"))
                            .as_ptr(),
                        b"av1_loopfilter.c\0" as *const u8 as *const core::ffi::c_char,
                        1897 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
            _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_block_plane_horz(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    plane: core::ffi::c_int,
    plane_ptr: *const MACROBLOCKD_PLANE,
    mi_row: uint32_t,
    mi_col: uint32_t,
) {
    let scale_horz: uint32_t = (*plane_ptr).subsampling_x as uint32_t;
    let scale_vert: uint32_t = (*plane_ptr).subsampling_y as uint32_t;
    let dst_ptr: *mut uint8_t = (*plane_ptr).dst.buf;
    let dst_stride: core::ffi::c_int = (*plane_ptr).dst.stride;
    let plane_mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << scale_vert >> 1 as core::ffi::c_int) >> scale_vert;
    let plane_mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << scale_horz >> 1 as core::ffi::c_int) >> scale_horz;
    let y_range: core::ffi::c_int = if ((plane_mi_rows as uint32_t)
        .wrapping_sub(mi_row >> scale_vert) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_vert
    {
        (plane_mi_rows as uint32_t).wrapping_sub(mi_row >> scale_vert)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_vert
    };
    let x_range: core::ffi::c_int = if ((plane_mi_cols as uint32_t)
        .wrapping_sub(mi_col >> scale_horz) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    {
        (plane_mi_cols as uint32_t).wrapping_sub(mi_col >> scale_horz)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    };
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < x_range {
        let mut p: *mut uint8_t = dst_ptr.offset((x * MI_SIZE) as isize);
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < y_range {
            let curr_x: uint32_t = (mi_col.wrapping_mul(MI_SIZE as uint32_t)
                >> scale_horz)
                .wrapping_add((x * MI_SIZE) as uint32_t);
            let curr_y: uint32_t = (mi_row.wrapping_mul(MI_SIZE as uint32_t)
                >> scale_vert)
                .wrapping_add((y * MI_SIZE) as uint32_t);
            let mut advance_units: uint32_t = 0;
            let mut tx_size: TX_SIZE = 0;
            let mut params: AV1_DEBLOCKING_PARAMETERS = AV1_DEBLOCKING_PARAMETERS {
                filter_length: 0,
                lfthr: 0 as *const loop_filter_thresh,
            };
            memset(
                &raw mut params as *mut AV1_DEBLOCKING_PARAMETERS as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<AV1_DEBLOCKING_PARAMETERS>() as size_t,
            );
            tx_size = set_lpf_parameters(
                &raw mut params,
                ((*cm).mi_params.mi_stride << scale_vert) as ptrdiff_t,
                cm,
                xd,
                HORZ_EDGE as core::ffi::c_int as EDGE_DIR,
                curr_x,
                curr_y,
                plane,
                plane_ptr as *const macroblockd_plane,
            );
            if tx_size as core::ffi::c_int == TX_INVALID as core::ffi::c_int {
                params.filter_length = 0 as uint8_t;
                tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            }
            filter_horz(
                p,
                dst_stride,
                &raw mut params,
                (*cm).seq_params,
                USE_SINGLE as core::ffi::c_int as USE_FILTER_TYPE,
            );
            advance_units = tx_size_high_unit[tx_size as usize] as uint32_t;
            y = (y as uint32_t).wrapping_add(advance_units) as core::ffi::c_int
                as core::ffi::c_int;
            p = p
                .offset(
                    advance_units
                        .wrapping_mul(dst_stride as uint32_t)
                        .wrapping_mul(MI_SIZE as uint32_t) as isize,
                );
        }
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_block_plane_horz_opt(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    plane_ptr: *const MACROBLOCKD_PLANE,
    mi_row: uint32_t,
    mi_col: uint32_t,
    mut params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let dst_ptr: *mut uint8_t = (*plane_ptr).dst.buf;
    let dst_stride: core::ffi::c_int = (*plane_ptr).dst.stride;
    let plane_mi_cols: core::ffi::c_int = (*plane_ptr).dst.width
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) - 1 as core::ffi::c_int
        >> 2 as core::ffi::c_int;
    let plane_mi_rows: core::ffi::c_int = (*plane_ptr).dst.height
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int) - 1 as core::ffi::c_int
        >> 2 as core::ffi::c_int;
    let y_range: core::ffi::c_int = if ((plane_mi_rows as uint32_t).wrapping_sub(mi_row)
        as core::ffi::c_int) < (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2
    {
        (plane_mi_rows as uint32_t).wrapping_sub(mi_row) as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2
    };
    let x_range: core::ffi::c_int = if ((plane_mi_cols as uint32_t).wrapping_sub(mi_col)
        as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
    {
        (plane_mi_cols as uint32_t).wrapping_sub(mi_col) as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
    };
    let mode_step: ptrdiff_t = (*cm).mi_params.mi_stride as ptrdiff_t;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < x_range {
        let curr_x: uint32_t = mi_col.wrapping_add(x as uint32_t);
        let y_start: uint32_t = mi_row;
        let y_end: uint32_t = mi_row.wrapping_add(y_range as uint32_t);
        let mut min_block_width: core::ffi::c_int = block_size_high[BLOCK_128X128
            as core::ffi::c_int as usize] as core::ffi::c_int;
        set_lpf_parameters_for_line_luma(
            params_buf,
            tx_buf,
            cm,
            xd,
            HORZ_EDGE as core::ffi::c_int as EDGE_DIR,
            curr_x,
            y_start,
            plane_ptr as *const macroblockd_plane,
            y_end,
            mode_step,
            &raw mut min_block_width,
        );
        let mut params: *mut AV1_DEBLOCKING_PARAMETERS = params_buf;
        let mut tx_size: *mut TX_SIZE = tx_buf;
        let mut filter_type: USE_FILTER_TYPE = USE_SINGLE as core::ffi::c_int
            as USE_FILTER_TYPE;
        let mut p: *mut uint8_t = dst_ptr.offset((x * MI_SIZE) as isize);
        if x & 3 as core::ffi::c_int == 0 as core::ffi::c_int
            && (x + 3 as core::ffi::c_int) < x_range
            && min_block_width >= 16 as core::ffi::c_int
        {
            filter_type = USE_QUAD as core::ffi::c_int as USE_FILTER_TYPE;
            x += 3 as core::ffi::c_int;
        } else if (x + 1 as core::ffi::c_int) < x_range
            && min_block_width >= 8 as core::ffi::c_int
        {
            filter_type = USE_DUAL as core::ffi::c_int as USE_FILTER_TYPE;
            x += 1 as core::ffi::c_int;
        }
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < y_range {
            if *tx_size as core::ffi::c_int == TX_INVALID as core::ffi::c_int {
                (*params).filter_length = 0 as uint8_t;
                *tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            }
            filter_horz(p, dst_stride, params, (*cm).seq_params, filter_type);
            let advance_units: uint32_t = tx_size_high_unit[*tx_size as usize]
                as uint32_t;
            y = (y as uint32_t).wrapping_add(advance_units) as core::ffi::c_int
                as core::ffi::c_int;
            p = p
                .offset(
                    advance_units
                        .wrapping_mul(dst_stride as uint32_t)
                        .wrapping_mul(MI_SIZE as uint32_t) as isize,
                );
            params = params.offset(advance_units as isize);
            tx_size = tx_size.offset(advance_units as isize);
        }
        x += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_block_plane_horz_opt_chroma(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    plane_ptr: *const MACROBLOCKD_PLANE,
    mi_row: uint32_t,
    mi_col: uint32_t,
    mut params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    mut plane: core::ffi::c_int,
    mut joint_filter_chroma: bool,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let scale_horz: uint32_t = (*plane_ptr).subsampling_x as uint32_t;
    let scale_vert: uint32_t = (*plane_ptr).subsampling_y as uint32_t;
    let dst_stride: core::ffi::c_int = (*plane_ptr).dst.stride;
    let mi_cols: core::ffi::c_int = ((*plane_ptr).dst.width << scale_horz) + MI_SIZE
        - 1 as core::ffi::c_int >> MI_SIZE_LOG2;
    let mi_rows: core::ffi::c_int = ((*plane_ptr).dst.height << scale_vert) + MI_SIZE
        - 1 as core::ffi::c_int >> MI_SIZE_LOG2;
    let plane_mi_rows: core::ffi::c_int = mi_rows
        + ((1 as core::ffi::c_int) << scale_vert >> 1 as core::ffi::c_int) >> scale_vert;
    let plane_mi_cols: core::ffi::c_int = mi_cols
        + ((1 as core::ffi::c_int) << scale_horz >> 1 as core::ffi::c_int) >> scale_horz;
    let y_range: core::ffi::c_int = if ((plane_mi_rows as uint32_t)
        .wrapping_sub(mi_row >> scale_vert) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2 >> scale_vert
    {
        (plane_mi_rows as uint32_t).wrapping_sub(mi_row >> scale_vert)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2 >> scale_vert
    };
    let x_range: core::ffi::c_int = if ((plane_mi_cols as uint32_t)
        .wrapping_sub(mi_col >> scale_horz) as core::ffi::c_int)
        < (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    {
        (plane_mi_cols as uint32_t).wrapping_sub(mi_col >> scale_horz)
            as core::ffi::c_int
    } else {
        (1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int
            >> scale_horz
    };
    let mode_step: ptrdiff_t = ((*cm).mi_params.mi_stride << scale_vert) as ptrdiff_t;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < x_range {
        let y_start: uint32_t = mi_row
            .wrapping_add(((0 as core::ffi::c_int) << scale_vert) as uint32_t);
        let curr_x: uint32_t = mi_col.wrapping_add((x << scale_horz) as uint32_t);
        let y_end: uint32_t = mi_row.wrapping_add((y_range << scale_vert) as uint32_t);
        let mut min_width: core::ffi::c_int = tx_size_wide[TX_64X64 as core::ffi::c_int
            as usize];
        set_lpf_parameters_for_line_chroma(
            params_buf,
            tx_buf,
            cm,
            xd,
            HORZ_EDGE as core::ffi::c_int as EDGE_DIR,
            curr_x,
            y_start,
            plane_ptr as *const macroblockd_plane,
            y_end,
            mode_step,
            scale_horz as core::ffi::c_int,
            scale_vert as core::ffi::c_int,
            &raw mut min_width,
            plane,
            joint_filter_chroma as core::ffi::c_int,
        );
        let mut params: *mut AV1_DEBLOCKING_PARAMETERS = params_buf;
        let mut tx_size: *mut TX_SIZE = tx_buf;
        let mut use_filter_type: USE_FILTER_TYPE = USE_SINGLE as core::ffi::c_int
            as USE_FILTER_TYPE;
        let mut x_inc: core::ffi::c_int = 0 as core::ffi::c_int;
        if x & 3 as core::ffi::c_int == 0 as core::ffi::c_int
            && (x + 3 as core::ffi::c_int) < x_range
            && min_width >= 16 as core::ffi::c_int
        {
            use_filter_type = USE_QUAD as core::ffi::c_int as USE_FILTER_TYPE;
            x_inc = 3 as core::ffi::c_int;
        } else if x % 2 as core::ffi::c_int == 0 as core::ffi::c_int
            && (x + 1 as core::ffi::c_int) < x_range
            && min_width >= 8 as core::ffi::c_int
        {
            use_filter_type = USE_DUAL as core::ffi::c_int as USE_FILTER_TYPE;
            x_inc = 1 as core::ffi::c_int;
        }
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < y_range {
            if *tx_size as core::ffi::c_int == TX_INVALID as core::ffi::c_int {
                (*params).filter_length = 0 as uint8_t;
                *tx_size = TX_4X4 as core::ffi::c_int as TX_SIZE;
            }
            let offset: core::ffi::c_int = y * MI_SIZE * dst_stride + x * MI_SIZE;
            if joint_filter_chroma {
                let mut u_dst: *mut uint8_t = ((*plane_ptr
                    .offset(0 as core::ffi::c_int as isize))
                    .dst
                    .buf)
                    .offset(offset as isize);
                let mut v_dst: *mut uint8_t = ((*plane_ptr
                    .offset(1 as core::ffi::c_int as isize))
                    .dst
                    .buf)
                    .offset(offset as isize);
                filter_horz_chroma(
                    u_dst,
                    v_dst,
                    dst_stride,
                    params,
                    (*cm).seq_params,
                    use_filter_type,
                );
            } else {
                let mut dst_ptr: *mut uint8_t = ((*plane_ptr).dst.buf)
                    .offset(offset as isize);
                filter_horz(
                    dst_ptr,
                    dst_stride,
                    params,
                    (*cm).seq_params,
                    use_filter_type,
                );
            }
            let advance_units: core::ffi::c_int = tx_size_high_unit[*tx_size as usize];
            y += advance_units;
            params = params.offset(advance_units as isize);
            tx_size = tx_size.offset(advance_units as isize);
        }
        x += x_inc;
        x += 1;
    }
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
