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

// Mechanically translated from upstream/aom/aom_dsp/flow_estimation/corner_match.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_compute_pyramid(
        frame: *const YV12_BUFFER_CONFIG,
        bit_depth: core::ffi::c_int,
        n_levels: core::ffi::c_int,
        pyr: *mut ImagePyramid,
    ) -> core::ffi::c_int;
    fn av1_compute_corner_list(
        frame: *const YV12_BUFFER_CONFIG,
        bit_depth: core::ffi::c_int,
        downsample_level: core::ffi::c_int,
        corners: *mut CornerList,
    ) -> bool;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_compute_flow_at_point_c(
        src: *const uint8_t,
        ref_0: *const uint8_t,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        stride: core::ffi::c_int,
        u: *mut core::ffi::c_double,
        v: *mut core::ffi::c_double,
    );
    fn ransac(
        matched_points: *const Correspondence,
        npoints: core::ffi::c_int,
        type_0: TransformationType,
        motion_models: *mut MotionModel,
        num_desired_motions: core::ffi::c_int,
        mem_alloc_failed: *mut bool,
    ) -> bool;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub type TransformationType = uint8_t;
pub type ImagePyramid = image_pyramid;
pub type CornerList = corner_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MotionModel {
    pub params: [core::ffi::c_double; 6],
    pub inliers: *mut core::ffi::c_int,
    pub num_inliers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Correspondence {
    pub x: core::ffi::c_double,
    pub y: core::ffi::c_double,
    pub rx: core::ffi::c_double,
    pub ry: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PointInfo {
    pub x: core::ffi::c_int,
    pub y: core::ffi::c_int,
    pub mean: core::ffi::c_double,
    pub one_over_stddev: core::ffi::c_double,
    pub best_match_idx: core::ffi::c_int,
    pub best_match_corr: core::ffi::c_double,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const THRESHOLD_NCC: core::ffi::c_double = 0.75f64;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_compute_mean_stddev_c(
    mut frame: *const core::ffi::c_uchar,
    mut stride: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut mean: *mut core::ffi::c_double,
    mut one_over_stddev: *mut core::ffi::c_double,
) -> bool {
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut sumsq: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MATCH_SZ {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < MATCH_SZ {
            sum
                += *frame
                    .offset(
                        ((i + y - MATCH_SZ_BY2) * stride + (j + x - MATCH_SZ_BY2))
                            as isize,
                    ) as core::ffi::c_int;
            sumsq
                += *frame
                    .offset(
                        ((i + y - MATCH_SZ_BY2) * stride + (j + x - MATCH_SZ_BY2))
                            as isize,
                    ) as core::ffi::c_int
                    * *frame
                        .offset(
                            ((i + y - MATCH_SZ_BY2) * stride + (j + x - MATCH_SZ_BY2))
                                as isize,
                        ) as core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
    *mean = sum as core::ffi::c_double / MATCH_SZ as core::ffi::c_double;
    let variance: core::ffi::c_double = sumsq as core::ffi::c_double - *mean * *mean;
    if variance < MIN_FEATURE_VARIANCE as core::ffi::c_double {
        *one_over_stddev = 0.0f64;
        return false_0 != 0;
    }
    *one_over_stddev = 1.0f64 / sqrt(variance);
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_compute_correlation_c(
    mut frame1: *const core::ffi::c_uchar,
    mut stride1: core::ffi::c_int,
    mut x1: core::ffi::c_int,
    mut y1: core::ffi::c_int,
    mut mean1: core::ffi::c_double,
    mut one_over_stddev1: core::ffi::c_double,
    mut frame2: *const core::ffi::c_uchar,
    mut stride2: core::ffi::c_int,
    mut x2: core::ffi::c_int,
    mut y2: core::ffi::c_int,
    mut mean2: core::ffi::c_double,
    mut one_over_stddev2: core::ffi::c_double,
) -> core::ffi::c_double {
    let mut v1: core::ffi::c_int = 0;
    let mut v2: core::ffi::c_int = 0;
    let mut cross: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MATCH_SZ {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < MATCH_SZ {
            v1 = *frame1
                .offset(
                    ((i + y1 - MATCH_SZ_BY2) * stride1 + (j + x1 - MATCH_SZ_BY2))
                        as isize,
                ) as core::ffi::c_int;
            v2 = *frame2
                .offset(
                    ((i + y2 - MATCH_SZ_BY2) * stride2 + (j + x2 - MATCH_SZ_BY2))
                        as isize,
                ) as core::ffi::c_int;
            cross += v1 * v2;
            j += 1;
        }
        i += 1;
    }
    let mut covariance: core::ffi::c_double = cross as core::ffi::c_double
        - mean1 * mean2;
    let mut correlation: core::ffi::c_double = covariance
        * (one_over_stddev1 * one_over_stddev2);
    return correlation;
}
unsafe extern "C-unwind" fn is_eligible_point(
    mut pointx: core::ffi::c_int,
    mut pointy: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return (pointx >= MATCH_SZ_BY2 && pointy >= MATCH_SZ_BY2
        && pointx + MATCH_SZ_BY2 < width && pointy + MATCH_SZ_BY2 < height)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn is_eligible_distance(
    mut point1x: core::ffi::c_int,
    mut point1y: core::ffi::c_int,
    mut point2x: core::ffi::c_int,
    mut point2y: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    let thresh: core::ffi::c_int = (if width < height { height } else { width })
        >> 4 as core::ffi::c_int;
    return ((point1x - point2x) * (point1x - point2x)
        + (point1y - point2y) * (point1y - point2y) <= thresh * thresh)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn determine_correspondence(
    mut src: *const core::ffi::c_uchar,
    mut src_corners: *const core::ffi::c_int,
    mut num_src_corners: core::ffi::c_int,
    mut ref_0: *const core::ffi::c_uchar,
    mut ref_corners: *const core::ffi::c_int,
    mut num_ref_corners: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut ref_stride: core::ffi::c_int,
    mut correspondences: *mut Correspondence,
) -> core::ffi::c_int {
    let mut src_point_count: core::ffi::c_int = 0;
    let mut ref_point_count: core::ffi::c_int = 0;
    let mut src_point_info: *mut PointInfo = 0 as *mut PointInfo;
    let mut ref_point_info: *mut PointInfo = 0 as *mut PointInfo;
    let mut num_correspondences: core::ffi::c_int = 0 as core::ffi::c_int;
    src_point_info = aom_calloc(
        num_src_corners as size_t,
        ::core::mem::size_of::<PointInfo>() as size_t,
    ) as *mut PointInfo;
    if !src_point_info.is_null() {
        ref_point_info = aom_calloc(
            num_ref_corners as size_t,
            ::core::mem::size_of::<PointInfo>() as size_t,
        ) as *mut PointInfo;
        if !ref_point_info.is_null() {
            src_point_count = 0 as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < num_src_corners {
                let mut src_x: core::ffi::c_int = *src_corners
                    .offset((2 as core::ffi::c_int * i) as isize);
                let mut src_y: core::ffi::c_int = *src_corners
                    .offset(
                        (2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize,
                    );
                if !(is_eligible_point(src_x, src_y, width, height) == 0) {
                    let mut point: *mut PointInfo = &raw mut *src_point_info
                        .offset(src_point_count as isize) as *mut PointInfo;
                    (*point).x = src_x;
                    (*point).y = src_y;
                    (*point).best_match_corr = THRESHOLD_NCC;
                    if aom_compute_mean_stddev_c(
                        src,
                        src_stride,
                        src_x,
                        src_y,
                        &raw mut (*point).mean,
                        &raw mut (*point).one_over_stddev,
                    ) {
                        src_point_count += 1;
                    }
                }
                i += 1;
            }
            if !(src_point_count == 0 as core::ffi::c_int) {
                ref_point_count = 0 as core::ffi::c_int;
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j < num_ref_corners {
                    let mut ref_x: core::ffi::c_int = *ref_corners
                        .offset((2 as core::ffi::c_int * j) as isize);
                    let mut ref_y: core::ffi::c_int = *ref_corners
                        .offset(
                            (2 as core::ffi::c_int * j + 1 as core::ffi::c_int) as isize,
                        );
                    if !(is_eligible_point(ref_x, ref_y, width, height) == 0) {
                        let mut point_0: *mut PointInfo = &raw mut *ref_point_info
                            .offset(ref_point_count as isize) as *mut PointInfo;
                        (*point_0).x = ref_x;
                        (*point_0).y = ref_y;
                        (*point_0).best_match_corr = THRESHOLD_NCC;
                        if aom_compute_mean_stddev_c(
                            ref_0,
                            ref_stride,
                            ref_x,
                            ref_y,
                            &raw mut (*point_0).mean,
                            &raw mut (*point_0).one_over_stddev,
                        ) {
                            ref_point_count += 1;
                        }
                    }
                    j += 1;
                }
                if !(ref_point_count == 0 as core::ffi::c_int) {
                    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while i_0 < src_point_count {
                        let mut src_point: *mut PointInfo = &raw mut *src_point_info
                            .offset(i_0 as isize) as *mut PointInfo;
                        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
                        while j_0 < ref_point_count {
                            let mut ref_point: *mut PointInfo = &raw mut *ref_point_info
                                .offset(j_0 as isize) as *mut PointInfo;
                            if !(is_eligible_distance(
                                (*src_point).x,
                                (*src_point).y,
                                (*ref_point).x,
                                (*ref_point).y,
                                width,
                                height,
                            ) == 0)
                            {
                                let mut corr: core::ffi::c_double = aom_compute_correlation_c(
                                    src,
                                    src_stride,
                                    (*src_point).x,
                                    (*src_point).y,
                                    (*src_point).mean,
                                    (*src_point).one_over_stddev,
                                    ref_0,
                                    ref_stride,
                                    (*ref_point).x,
                                    (*ref_point).y,
                                    (*ref_point).mean,
                                    (*ref_point).one_over_stddev,
                                );
                                if corr > (*src_point).best_match_corr {
                                    (*src_point).best_match_idx = j_0;
                                    (*src_point).best_match_corr = corr;
                                }
                                if corr > (*ref_point).best_match_corr {
                                    (*ref_point).best_match_idx = i_0;
                                    (*ref_point).best_match_corr = corr;
                                }
                            }
                            j_0 += 1;
                        }
                        i_0 += 1;
                    }
                    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
                    while i_1 < src_point_count {
                        let mut point_1: *mut PointInfo = &raw mut *src_point_info
                            .offset(i_1 as isize) as *mut PointInfo;
                        if !((*point_1).best_match_corr < THRESHOLD_NCC) {
                            let mut match_point: *mut PointInfo = &raw mut *ref_point_info
                                .offset((*point_1).best_match_idx as isize)
                                as *mut PointInfo;
                            if (*match_point).best_match_idx == i_1 {
                                let sx: core::ffi::c_int = (*point_1).x;
                                let sy: core::ffi::c_int = (*point_1).y;
                                let rx: core::ffi::c_int = (*match_point).x;
                                let ry: core::ffi::c_int = (*match_point).y;
                                let mut u: core::ffi::c_double = (rx - sx)
                                    as core::ffi::c_double;
                                let mut v: core::ffi::c_double = (ry - sy)
                                    as core::ffi::c_double;
                                let patch_tl_x: core::ffi::c_int = sx
                                    - DISFLOW_PATCH_CENTER;
                                let patch_tl_y: core::ffi::c_int = sy
                                    - DISFLOW_PATCH_CENTER;
                                aom_compute_flow_at_point_c(
                                    src as *const uint8_t,
                                    ref_0 as *const uint8_t,
                                    patch_tl_x,
                                    patch_tl_y,
                                    width,
                                    height,
                                    src_stride,
                                    &raw mut u,
                                    &raw mut v,
                                );
                                let mut correspondence: *mut Correspondence = &raw mut *correspondences
                                    .offset(num_correspondences as isize)
                                    as *mut Correspondence;
                                (*correspondence).x = sx as core::ffi::c_double;
                                (*correspondence).y = sy as core::ffi::c_double;
                                (*correspondence).rx = sx as core::ffi::c_double + u;
                                (*correspondence).ry = sy as core::ffi::c_double + v;
                                num_correspondences += 1;
                            }
                        }
                        i_1 += 1;
                    }
                }
            }
        }
    }
    aom_free(src_point_info as *mut core::ffi::c_void);
    aom_free(ref_point_info as *mut core::ffi::c_void);
    return num_correspondences;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_global_motion_feature_match(
    mut type_0: TransformationType,
    mut src: *mut YV12_BUFFER_CONFIG,
    mut ref_0: *mut YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut downsample_level: core::ffi::c_int,
    mut motion_models: *mut MotionModel,
    mut num_motion_models: core::ffi::c_int,
    mut mem_alloc_failed: *mut bool,
) -> bool {
    let mut num_correspondences: core::ffi::c_int = 0;
    let mut correspondences: *mut Correspondence = 0 as *mut Correspondence;
    let mut src_pyramid: *mut ImagePyramid = (*src).y_pyramid as *mut ImagePyramid;
    let mut src_corners: *mut CornerList = (*src).corners as *mut CornerList;
    let mut ref_pyramid: *mut ImagePyramid = (*ref_0).y_pyramid as *mut ImagePyramid;
    let mut ref_corners: *mut CornerList = (*ref_0).corners as *mut CornerList;
    if aom_compute_pyramid(src, bit_depth, 1 as core::ffi::c_int, src_pyramid)
        < 0 as core::ffi::c_int
    {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    if !av1_compute_corner_list(src, bit_depth, downsample_level, src_corners) {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    if aom_compute_pyramid(ref_0, bit_depth, 1 as core::ffi::c_int, ref_pyramid)
        < 0 as core::ffi::c_int
    {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    if !av1_compute_corner_list(src, bit_depth, downsample_level, ref_corners) {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    let mut src_buffer: *const uint8_t = (*((*src_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .buffer;
    let src_width: core::ffi::c_int = (*((*src_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .width;
    let src_height: core::ffi::c_int = (*((*src_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .height;
    let src_stride: core::ffi::c_int = (*((*src_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .stride;
    let mut ref_buffer: *const uint8_t = (*((*ref_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .buffer;
    if (*((*ref_pyramid).layers).offset(0 as core::ffi::c_int as isize)).width
        == src_width
    {} else {
        __assert_fail(
            b"ref_pyramid->layers[0].width == src_width\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/flow_estimation/corner_match.c\0" as *const u8
                as *const core::ffi::c_char,
            289 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 149],
                [core::ffi::c_char; 149],
            >(
                *b"_Bool av1_compute_global_motion_feature_match(TransformationType, YV12_BUFFER_CONFIG *, YV12_BUFFER_CONFIG *, int, int, MotionModel *, int, _Bool *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*((*ref_pyramid).layers).offset(0 as core::ffi::c_int as isize)).height
        == src_height
    {} else {
        __assert_fail(
            b"ref_pyramid->layers[0].height == src_height\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/flow_estimation/corner_match.c\0" as *const u8
                as *const core::ffi::c_char,
            290 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 149],
                [core::ffi::c_char; 149],
            >(
                *b"_Bool av1_compute_global_motion_feature_match(TransformationType, YV12_BUFFER_CONFIG *, YV12_BUFFER_CONFIG *, int, int, MotionModel *, int, _Bool *)\0",
            ))
                .as_ptr(),
        );
    };
    let ref_stride: core::ffi::c_int = (*((*ref_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .stride;
    correspondences = aom_malloc(
        ((*src_corners).num_corners as size_t)
            .wrapping_mul(::core::mem::size_of::<Correspondence>() as size_t),
    ) as *mut Correspondence;
    if correspondences.is_null() {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    num_correspondences = determine_correspondence(
        src_buffer as *const core::ffi::c_uchar,
        ((*src_corners).corners).as_mut_ptr(),
        (*src_corners).num_corners,
        ref_buffer as *const core::ffi::c_uchar,
        ((*ref_corners).corners).as_mut_ptr(),
        (*ref_corners).num_corners,
        src_width,
        src_height,
        src_stride,
        ref_stride,
        correspondences,
    );
    let mut result: bool = ransac(
        correspondences,
        num_correspondences,
        type_0,
        motion_models,
        num_motion_models,
        mem_alloc_failed,
    );
    aom_free(correspondences as *mut core::ffi::c_void);
    return result;
}
pub const MATCH_SZ: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MATCH_SZ_BY2: core::ffi::c_int = (MATCH_SZ - 1 as core::ffi::c_int)
    / 2 as core::ffi::c_int;
pub const MATCH_SZ_SQ: core::ffi::c_int = MATCH_SZ * MATCH_SZ;
pub const MIN_FEATURE_VARIANCE: core::ffi::c_int = 1 as core::ffi::c_int * MATCH_SZ_SQ;
pub const DISFLOW_PATCH_SIZE_LOG2: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DISFLOW_PATCH_SIZE: core::ffi::c_int = (1 as core::ffi::c_int)
    << DISFLOW_PATCH_SIZE_LOG2;
pub const DISFLOW_PATCH_CENTER: core::ffi::c_int = DISFLOW_PATCH_SIZE
    / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
