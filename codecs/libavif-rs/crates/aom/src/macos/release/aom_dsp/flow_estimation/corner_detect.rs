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

// Mechanically translated from upstream/aom/aom_dsp/flow_estimation/corner_detect.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_fast9_detect_nonmax(
        im: *const byte,
        xsize: core::ffi::c_int,
        ysize: core::ffi::c_int,
        stride: core::ffi::c_int,
        b: core::ffi::c_int,
        ret_scores: *mut *mut core::ffi::c_int,
        ret_num_corners: *mut core::ffi::c_int,
    ) -> *mut xy;
    fn aom_compute_pyramid(
        frame: *const YV12_BUFFER_CONFIG,
        bit_depth: core::ffi::c_int,
        n_levels: core::ffi::c_int,
        pyr: *mut ImagePyramid,
    ) -> core::ffi::c_int;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xy {
    pub x: core::ffi::c_int,
    pub y: core::ffi::c_int,
}
pub type byte = core::ffi::c_uchar;
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
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
pub type ImagePyramid = image_pyramid;
pub type CornerList = corner_list;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_CORNERS: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const FAST_BARRIER: core::ffi::c_int = 18 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_corner_list_size() -> size_t {
    return ::core::mem::size_of::<CornerList>() as size_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_alloc_corner_list() -> *mut CornerList {
    let mut corners: *mut CornerList = aom_calloc(
        1 as size_t,
        ::core::mem::size_of::<CornerList>() as size_t,
    ) as *mut CornerList;
    if corners.is_null() {
        return 0 as *mut CornerList;
    }
    (*corners).valid = false_0 != 0;
    pthread_mutex_init(&raw mut (*corners).mutex, 0 as *const pthread_mutexattr_t);
    return corners;
}
unsafe extern "C-unwind" fn compute_corner_list(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut downsample_level: core::ffi::c_int,
    mut corners: *mut CornerList,
) -> bool {
    let mut pyr: *mut ImagePyramid = (*frame).y_pyramid as *mut ImagePyramid;
    let layers: core::ffi::c_int = aom_compute_pyramid(
        frame,
        bit_depth,
        downsample_level + 1 as core::ffi::c_int,
        pyr,
    ) as core::ffi::c_int;
    if layers < 0 as core::ffi::c_int {
        return false_0 != 0;
    }
    downsample_level = layers - 1 as core::ffi::c_int;
    let mut buf: *const uint8_t = (*((*pyr).layers).offset(downsample_level as isize))
        .buffer;
    let mut width: core::ffi::c_int = (*((*pyr).layers)
        .offset(downsample_level as isize))
        .width;
    let mut height: core::ffi::c_int = (*((*pyr).layers)
        .offset(downsample_level as isize))
        .height;
    let mut stride: core::ffi::c_int = (*((*pyr).layers)
        .offset(downsample_level as isize))
        .stride;
    let mut scores: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut num_corners: core::ffi::c_int = 0;
    let frame_corners_xy: *mut xy = aom_fast9_detect_nonmax(
        buf as *const byte,
        width,
        height,
        stride,
        FAST_BARRIER,
        &raw mut scores,
        &raw mut num_corners,
    ) as *mut xy;
    if num_corners < 0 as core::ffi::c_int {
        return false_0 != 0;
    }
    if num_corners <= MAX_CORNERS {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < num_corners {
            (*corners)
                .corners[(2 as core::ffi::c_int * i + 0 as core::ffi::c_int) as usize] = (*frame_corners_xy
                .offset(i as isize))
                .x * ((1 as core::ffi::c_int) << downsample_level);
            (*corners)
                .corners[(2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as usize] = (*frame_corners_xy
                .offset(i as isize))
                .y * ((1 as core::ffi::c_int) << downsample_level);
            i += 1;
        }
        (*corners).num_corners = num_corners;
    } else {
        let mut histogram: [core::ffi::c_int; 256] = [0; 256];
        memset(
            &raw mut histogram as *mut [core::ffi::c_int; 256] as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[core::ffi::c_int; 256]>() as size_t,
        );
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < num_corners {
            histogram[*scores.offset(i_0 as isize) as usize] += 1 as core::ffi::c_int;
            i_0 += 1;
        }
        let mut threshold: core::ffi::c_int = -(1 as core::ffi::c_int);
        let mut found_corners: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut bucket: core::ffi::c_int = 255 as core::ffi::c_int;
        while bucket >= 0 as core::ffi::c_int {
            if found_corners + histogram[bucket as usize] > MAX_CORNERS {
                threshold = bucket;
                break;
            } else {
                found_corners += histogram[bucket as usize];
                bucket -= 1;
            }
        }
        let mut copied_corners: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < num_corners {
            if *scores.offset(i_1 as isize) > threshold {
                (*corners)
                    .corners[(2 as core::ffi::c_int * copied_corners
                    + 0 as core::ffi::c_int) as usize] = (*frame_corners_xy
                    .offset(i_1 as isize))
                    .x * ((1 as core::ffi::c_int) << downsample_level);
                (*corners)
                    .corners[(2 as core::ffi::c_int * copied_corners
                    + 1 as core::ffi::c_int) as usize] = (*frame_corners_xy
                    .offset(i_1 as isize))
                    .y * ((1 as core::ffi::c_int) << downsample_level);
                copied_corners += 1 as core::ffi::c_int;
            }
            i_1 += 1;
        }
        (*corners).num_corners = copied_corners;
    }
    free(scores as *mut core::ffi::c_void);
    free(frame_corners_xy as *mut core::ffi::c_void);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_corner_list(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut downsample_level: core::ffi::c_int,
    mut corners: *mut CornerList,
) -> bool {
    pthread_mutex_lock(&raw mut (*corners).mutex);
    if !(*corners).valid {
        (*corners).valid = compute_corner_list(
            frame,
            bit_depth,
            downsample_level,
            corners,
        );
    }
    let mut valid: bool = (*corners).valid;
    pthread_mutex_unlock(&raw mut (*corners).mutex);
    return valid;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_invalidate_corner_list(mut corners: *mut CornerList) {
    if !corners.is_null() {
        pthread_mutex_lock(&raw mut (*corners).mutex);
        (*corners).valid = false_0 != 0;
        pthread_mutex_unlock(&raw mut (*corners).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_free_corner_list(mut corners: *mut CornerList) {
    if !corners.is_null() {
        pthread_mutex_destroy(&raw mut (*corners).mutex);
        aom_free(corners as *mut core::ffi::c_void);
    }
}

extern "C" {
    fn free(_: *mut core::ffi::c_void);
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
}
