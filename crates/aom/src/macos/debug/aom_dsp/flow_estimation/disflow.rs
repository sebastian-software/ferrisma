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

// Dense Inverse Search flow algorithm
// Paper: https://arxiv.org/abs/1603.03590

// Mechanically translated from upstream/aom/aom_dsp/flow_estimation/disflow.c.
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
    fn ransac(
        matched_points: *const Correspondence,
        npoints: core::ffi::c_int,
        type_0: TransformationType,
        motion_models: *mut MotionModel,
        num_desired_motions: core::ffi::c_int,
        mem_alloc_failed: *mut bool,
    ) -> bool;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
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
pub type ImagePyramid = image_pyramid;
pub type CornerList = corner_list;
pub type TransformationType = uint8_t;
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
pub struct FlowField {
    pub buf0: *mut core::ffi::c_double,
    pub u: *mut core::ffi::c_double,
    pub v: *mut core::ffi::c_double,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
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
#[inline]
unsafe extern "C-unwind" fn fclamp(
    mut value: core::ffi::c_double,
    mut low: core::ffi::c_double,
    mut high: core::ffi::c_double,
) -> core::ffi::c_double {
    return if value < low { low } else if value > high { high } else { value };
}
pub const DISFLOW_PYRAMID_LEVELS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const DISFLOW_PATCH_SIZE_LOG2: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DISFLOW_PATCH_SIZE: core::ffi::c_int = (1 as core::ffi::c_int)
    << DISFLOW_PATCH_SIZE_LOG2;
pub const DISFLOW_PATCH_CENTER: core::ffi::c_int = DISFLOW_PATCH_SIZE
    / 2 as core::ffi::c_int - 1 as core::ffi::c_int;
pub const DISFLOW_DERIV_SCALE_LOG2: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DISFLOW_STEP_SIZE: core::ffi::c_double = 1.0f64;
pub const DISFLOW_STEP_SIZE_THRESOLD: core::ffi::c_double = 1.0f64 / 8.0f64;
pub const DISFLOW_MAX_ITR: core::ffi::c_int = 4 as core::ffi::c_int;
pub const DISFLOW_INTERP_BITS: core::ffi::c_int = 14 as core::ffi::c_int;
pub const DOWNSAMPLE_SHIFT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const DOWNSAMPLE_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << DOWNSAMPLE_SHIFT;
pub const FLOW_UPSCALE_TAPS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const FLOW_BORDER_INNER: core::ffi::c_int = DISFLOW_PATCH_SIZE
    >> 1 as core::ffi::c_int >> DOWNSAMPLE_SHIFT;
pub const FLOW_BORDER_OUTER: core::ffi::c_int = FLOW_UPSCALE_TAPS
    / 2 as core::ffi::c_int;
pub const UPSAMPLE_CENTER_OFFSET: core::ffi::c_int = (DOWNSAMPLE_FACTOR
    - 1 as core::ffi::c_int) / 2 as core::ffi::c_int;
static mut flow_upscale_filter: [[core::ffi::c_double; 4]; 2] = [
    [
        -(3 as core::ffi::c_int) as core::ffi::c_double / 128.0f64,
        29 as core::ffi::c_int as core::ffi::c_double / 128.0f64,
        111 as core::ffi::c_int as core::ffi::c_double / 128.0f64,
        -(9 as core::ffi::c_int) as core::ffi::c_double / 128.0f64,
    ],
    [
        -(9 as core::ffi::c_int) as core::ffi::c_double / 128.0f64,
        111 as core::ffi::c_int as core::ffi::c_double / 128.0f64,
        29 as core::ffi::c_int as core::ffi::c_double / 128.0f64,
        -(3 as core::ffi::c_int) as core::ffi::c_double / 128.0f64,
    ],
];
#[inline]
unsafe extern "C-unwind" fn get_cubic_kernel_dbl(
    mut x: core::ffi::c_double,
    mut kernel: *mut core::ffi::c_double,
) {
    if !(0 as core::ffi::c_int as core::ffi::c_double <= x
        && x <= 1 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_cubic_kernel_dbl\0"))
                .as_ptr(),
            b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
            73 as core::ffi::c_int,
            b"0 <= x && x <= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut x2: core::ffi::c_double = x * x;
    let mut x3: core::ffi::c_double = x2 * x;
    *kernel.offset(0 as core::ffi::c_int as isize) = -0.5f64 * x + x2 - 0.5f64 * x3;
    *kernel.offset(1 as core::ffi::c_int as isize) = 1.0f64 - 2.5f64 * x2 + 1.5f64 * x3;
    *kernel.offset(2 as core::ffi::c_int as isize) = 0.5f64 * x + 2.0f64 * x2
        - 1.5f64 * x3;
    *kernel.offset(3 as core::ffi::c_int as isize) = -0.5f64 * x2 + 0.5f64 * x3;
}
#[inline]
unsafe extern "C-unwind" fn get_cubic_kernel_int(
    mut x: core::ffi::c_double,
    mut kernel: *mut core::ffi::c_int,
) {
    let mut kernel_dbl: [core::ffi::c_double; 4] = [0.; 4];
    get_cubic_kernel_dbl(x, kernel_dbl.as_mut_ptr());
    *kernel.offset(0 as core::ffi::c_int as isize) = rint(
        kernel_dbl[0 as core::ffi::c_int as usize]
            * ((1 as core::ffi::c_int) << DISFLOW_INTERP_BITS) as core::ffi::c_double,
    ) as core::ffi::c_int;
    *kernel.offset(1 as core::ffi::c_int as isize) = rint(
        kernel_dbl[1 as core::ffi::c_int as usize]
            * ((1 as core::ffi::c_int) << DISFLOW_INTERP_BITS) as core::ffi::c_double,
    ) as core::ffi::c_int;
    *kernel.offset(2 as core::ffi::c_int as isize) = rint(
        kernel_dbl[2 as core::ffi::c_int as usize]
            * ((1 as core::ffi::c_int) << DISFLOW_INTERP_BITS) as core::ffi::c_double,
    ) as core::ffi::c_int;
    *kernel.offset(3 as core::ffi::c_int as isize) = rint(
        kernel_dbl[3 as core::ffi::c_int as usize]
            * ((1 as core::ffi::c_int) << DISFLOW_INTERP_BITS) as core::ffi::c_double,
    ) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_cubic_value_dbl(
    mut p: *const core::ffi::c_double,
    mut kernel: *const core::ffi::c_double,
) -> core::ffi::c_double {
    return *kernel.offset(0 as core::ffi::c_int as isize)
        * *p.offset(0 as core::ffi::c_int as isize)
        + *kernel.offset(1 as core::ffi::c_int as isize)
            * *p.offset(1 as core::ffi::c_int as isize)
        + *kernel.offset(2 as core::ffi::c_int as isize)
            * *p.offset(2 as core::ffi::c_int as isize)
        + *kernel.offset(3 as core::ffi::c_int as isize)
            * *p.offset(3 as core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C-unwind" fn get_cubic_value_int(
    mut p: *const core::ffi::c_int,
    mut kernel: *const core::ffi::c_int,
) -> core::ffi::c_int {
    return *kernel.offset(0 as core::ffi::c_int as isize)
        * *p.offset(0 as core::ffi::c_int as isize)
        + *kernel.offset(1 as core::ffi::c_int as isize)
            * *p.offset(1 as core::ffi::c_int as isize)
        + *kernel.offset(2 as core::ffi::c_int as isize)
            * *p.offset(2 as core::ffi::c_int as isize)
        + *kernel.offset(3 as core::ffi::c_int as isize)
            * *p.offset(3 as core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C-unwind" fn bicubic_interp_one(
    mut arr: *const core::ffi::c_double,
    mut stride: core::ffi::c_int,
    mut h_kernel: *const core::ffi::c_double,
    mut v_kernel: *const core::ffi::c_double,
) -> core::ffi::c_double {
    let mut tmp: [core::ffi::c_double; 4] = [0.; 4];
    let mut i: core::ffi::c_int = -(1 as core::ffi::c_int);
    while i < 3 as core::ffi::c_int {
        tmp[(i + 1 as core::ffi::c_int) as usize] = get_cubic_value_dbl(
            &*arr.offset((i * stride - 1 as core::ffi::c_int) as isize),
            h_kernel,
        );
        i += 1;
    }
    return get_cubic_value_dbl(tmp.as_mut_ptr(), v_kernel);
}
unsafe extern "C-unwind" fn determine_disflow_correspondence(
    mut src_pyr: *const ImagePyramid,
    mut ref_pyr: *const ImagePyramid,
    mut corners: *mut CornerList,
    mut flow: *const FlowField,
    mut correspondences: *mut Correspondence,
) -> core::ffi::c_int {
    let width: core::ffi::c_int = (*flow).width;
    let height: core::ffi::c_int = (*flow).height;
    let stride: core::ffi::c_int = (*flow).stride;
    let mut num_correspondences: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*corners).num_corners {
        let x0: core::ffi::c_int = (*corners)
            .corners[(2 as core::ffi::c_int * i) as usize];
        let y0: core::ffi::c_int = (*corners)
            .corners[(2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as usize];
        let x: core::ffi::c_int = x0 - UPSAMPLE_CENTER_OFFSET;
        let y: core::ffi::c_int = y0 - UPSAMPLE_CENTER_OFFSET;
        let flow_x: core::ffi::c_int = x >> DOWNSAMPLE_SHIFT;
        let flow_sub_x: core::ffi::c_double = (x
            & DOWNSAMPLE_FACTOR - 1 as core::ffi::c_int) as core::ffi::c_double
            / DOWNSAMPLE_FACTOR as core::ffi::c_double;
        let flow_y: core::ffi::c_int = y >> DOWNSAMPLE_SHIFT;
        let flow_sub_y: core::ffi::c_double = (y
            & DOWNSAMPLE_FACTOR - 1 as core::ffi::c_int) as core::ffi::c_double
            / DOWNSAMPLE_FACTOR as core::ffi::c_double;
        if !(flow_x < 1 as core::ffi::c_int || flow_x + 2 as core::ffi::c_int >= width) {
            if !(flow_y < 1 as core::ffi::c_int
                || flow_y + 2 as core::ffi::c_int >= height)
            {
                let mut h_kernel: [core::ffi::c_double; 4] = [0.; 4];
                let mut v_kernel: [core::ffi::c_double; 4] = [0.; 4];
                get_cubic_kernel_dbl(flow_sub_x, h_kernel.as_mut_ptr());
                get_cubic_kernel_dbl(flow_sub_y, v_kernel.as_mut_ptr());
                let mut flow_u: core::ffi::c_double = bicubic_interp_one(
                    &raw mut *((*flow).u).offset((flow_y * stride + flow_x) as isize),
                    stride,
                    h_kernel.as_mut_ptr() as *const core::ffi::c_double,
                    v_kernel.as_mut_ptr() as *const core::ffi::c_double,
                );
                let mut flow_v: core::ffi::c_double = bicubic_interp_one(
                    &raw mut *((*flow).v).offset((flow_y * stride + flow_x) as isize),
                    stride,
                    h_kernel.as_mut_ptr() as *const core::ffi::c_double,
                    v_kernel.as_mut_ptr() as *const core::ffi::c_double,
                );
                let patch_tl_x: core::ffi::c_int = x0 - DISFLOW_PATCH_CENTER;
                let patch_tl_y: core::ffi::c_int = y0 - DISFLOW_PATCH_CENTER;
                aom_compute_flow_at_point_c(
                    (*((*src_pyr).layers).offset(0 as core::ffi::c_int as isize)).buffer,
                    (*((*ref_pyr).layers).offset(0 as core::ffi::c_int as isize)).buffer,
                    patch_tl_x,
                    patch_tl_y,
                    (*((*src_pyr).layers).offset(0 as core::ffi::c_int as isize)).width,
                    (*((*src_pyr).layers).offset(0 as core::ffi::c_int as isize)).height,
                    (*((*src_pyr).layers).offset(0 as core::ffi::c_int as isize)).stride,
                    &raw mut flow_u,
                    &raw mut flow_v,
                );
                (*correspondences.offset(num_correspondences as isize)).x = x0
                    as core::ffi::c_double;
                (*correspondences.offset(num_correspondences as isize)).y = y0
                    as core::ffi::c_double;
                (*correspondences.offset(num_correspondences as isize)).rx = x0
                    as core::ffi::c_double + flow_u;
                (*correspondences.offset(num_correspondences as isize)).ry = y0
                    as core::ffi::c_double + flow_v;
                num_correspondences += 1;
            }
        }
        i += 1;
    }
    return num_correspondences;
}
#[inline]
unsafe extern "C-unwind" fn compute_flow_vector(
    mut src: *const uint8_t,
    mut ref_0: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut u: core::ffi::c_double,
    mut v: core::ffi::c_double,
    mut dx: *const int16_t,
    mut dy: *const int16_t,
    mut b: *mut core::ffi::c_int,
) {
    memset(
        b as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (2 as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    );
    let u_int: core::ffi::c_int = floor(u) as core::ffi::c_int;
    let v_int: core::ffi::c_int = floor(v) as core::ffi::c_int;
    let u_frac: core::ffi::c_double = u - floor(u) as core::ffi::c_double;
    let v_frac: core::ffi::c_double = v - floor(v) as core::ffi::c_double;
    let mut h_kernel: [core::ffi::c_int; 4] = [0; 4];
    let mut v_kernel: [core::ffi::c_int; 4] = [0; 4];
    get_cubic_kernel_int(u_frac, h_kernel.as_mut_ptr());
    get_cubic_kernel_int(v_frac, v_kernel.as_mut_ptr());
    let mut tmp_: [core::ffi::c_int; 88] = [0; 88];
    let mut tmp: *mut core::ffi::c_int = tmp_
        .as_mut_ptr()
        .offset(DISFLOW_PATCH_SIZE as isize);
    let x0: core::ffi::c_int = clamp(x + u_int, -(9 as core::ffi::c_int), width)
        as core::ffi::c_int;
    let y0: core::ffi::c_int = clamp(y + v_int, -(9 as core::ffi::c_int), height)
        as core::ffi::c_int;
    let mut i: core::ffi::c_int = -(1 as core::ffi::c_int);
    while i < DISFLOW_PATCH_SIZE + 2 as core::ffi::c_int {
        let y_w: core::ffi::c_int = y0 + i;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < DISFLOW_PATCH_SIZE {
            let x_w: core::ffi::c_int = x0 + j;
            let mut arr: [core::ffi::c_int; 4] = [0; 4];
            arr[0 as core::ffi::c_int as usize] = *ref_0
                .offset((y_w * stride + (x_w - 1 as core::ffi::c_int)) as isize)
                as core::ffi::c_int;
            arr[1 as core::ffi::c_int as usize] = *ref_0
                .offset((y_w * stride + (x_w + 0 as core::ffi::c_int)) as isize)
                as core::ffi::c_int;
            arr[2 as core::ffi::c_int as usize] = *ref_0
                .offset((y_w * stride + (x_w + 1 as core::ffi::c_int)) as isize)
                as core::ffi::c_int;
            arr[3 as core::ffi::c_int as usize] = *ref_0
                .offset((y_w * stride + (x_w + 2 as core::ffi::c_int)) as isize)
                as core::ffi::c_int;
            *tmp.offset((i * DISFLOW_PATCH_SIZE + j) as isize) = get_cubic_value_int(
                arr.as_mut_ptr(),
                h_kernel.as_mut_ptr() as *const core::ffi::c_int,
            )
                + ((1 as core::ffi::c_int)
                    << 14 as core::ffi::c_int - 6 as core::ffi::c_int
                    >> 1 as core::ffi::c_int)
                >> 14 as core::ffi::c_int - 6 as core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < DISFLOW_PATCH_SIZE {
        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_0 < DISFLOW_PATCH_SIZE {
            let mut p: *const core::ffi::c_int = &raw mut *tmp
                .offset((i_0 * DISFLOW_PATCH_SIZE + j_0) as isize)
                as *mut core::ffi::c_int;
            let arr_0: [core::ffi::c_int; 4] = [
                *p.offset(-DISFLOW_PATCH_SIZE as isize),
                *p.offset(0 as core::ffi::c_int as isize),
                *p.offset(DISFLOW_PATCH_SIZE as isize),
                *p.offset((2 as core::ffi::c_int * DISFLOW_PATCH_SIZE) as isize),
            ];
            let result: core::ffi::c_int = get_cubic_value_int(
                arr_0.as_ptr(),
                v_kernel.as_mut_ptr() as *const core::ffi::c_int,
            ) as core::ffi::c_int;
            let round_bits: core::ffi::c_int = DISFLOW_INTERP_BITS
                + 6 as core::ffi::c_int - DISFLOW_DERIV_SCALE_LOG2;
            let warped: core::ffi::c_int = result
                + ((1 as core::ffi::c_int) << round_bits >> 1 as core::ffi::c_int)
                >> round_bits;
            let src_px: core::ffi::c_int = (*src
                .offset((x + j_0 + (y + i_0) * stride) as isize) as core::ffi::c_int)
                << 3 as core::ffi::c_int;
            let dt: core::ffi::c_int = warped - src_px;
            *b.offset(0 as core::ffi::c_int as isize)
                += *dx.offset((i_0 * DISFLOW_PATCH_SIZE + j_0) as isize)
                    as core::ffi::c_int * dt;
            *b.offset(1 as core::ffi::c_int as isize)
                += *dy.offset((i_0 * DISFLOW_PATCH_SIZE + j_0) as isize)
                    as core::ffi::c_int * dt;
            j_0 += 1;
        }
        i_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn sobel_filter(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut int16_t,
    mut dst_stride: core::ffi::c_int,
    mut dir: core::ffi::c_int,
) {
    let mut tmp_: [int16_t; 80] = [0; 80];
    let mut tmp: *mut int16_t = tmp_.as_mut_ptr().offset(DISFLOW_PATCH_SIZE as isize);
    static mut sobel_a: [int16_t; 3] = [
        1 as core::ffi::c_int as int16_t,
        0 as core::ffi::c_int as int16_t,
        -(1 as core::ffi::c_int) as int16_t,
    ];
    static mut sobel_b: [int16_t; 3] = [
        1 as core::ffi::c_int as int16_t,
        2 as core::ffi::c_int as int16_t,
        1 as core::ffi::c_int as int16_t,
    ];
    let taps: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut h_kernel: *const int16_t = if dir != 0 {
        core::ptr::addr_of!(sobel_a).cast::<int16_t>()
    } else {
        core::ptr::addr_of!(sobel_b).cast::<int16_t>()
    };
    let mut y: core::ffi::c_int = -(1 as core::ffi::c_int);
    while y < DISFLOW_PATCH_SIZE + 1 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < DISFLOW_PATCH_SIZE {
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < taps {
                sum
                    += *h_kernel.offset(k as isize) as core::ffi::c_int
                        * *src
                            .offset(
                                (y * src_stride + (x + k - 1 as core::ffi::c_int)) as isize,
                            ) as core::ffi::c_int;
                k += 1;
            }
            *tmp.offset((y * DISFLOW_PATCH_SIZE + x) as isize) = sum as int16_t;
            x += 1;
        }
        y += 1;
    }
    let mut v_kernel: *const int16_t = if dir != 0 {
        core::ptr::addr_of!(sobel_b).cast::<int16_t>()
    } else {
        core::ptr::addr_of!(sobel_a).cast::<int16_t>()
    };
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < DISFLOW_PATCH_SIZE {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < DISFLOW_PATCH_SIZE {
            let mut sum_0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while k_0 < taps {
                sum_0
                    += *v_kernel.offset(k_0 as isize) as core::ffi::c_int
                        * *tmp
                            .offset(
                                ((y_0 + k_0 - 1 as core::ffi::c_int) * DISFLOW_PATCH_SIZE
                                    + x_0) as isize,
                            ) as core::ffi::c_int;
                k_0 += 1;
            }
            *dst.offset((y_0 * dst_stride + x_0) as isize) = sum_0 as int16_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn compute_flow_matrix(
    mut dx: *const int16_t,
    mut dx_stride: core::ffi::c_int,
    mut dy: *const int16_t,
    mut dy_stride: core::ffi::c_int,
    mut M: *mut core::ffi::c_double,
) {
    let mut tmp: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < DISFLOW_PATCH_SIZE {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < DISFLOW_PATCH_SIZE {
            tmp[0 as core::ffi::c_int as usize]
                += *dx.offset((i * dx_stride + j) as isize) as core::ffi::c_int
                    * *dx.offset((i * dx_stride + j) as isize) as core::ffi::c_int;
            tmp[1 as core::ffi::c_int as usize]
                += *dx.offset((i * dx_stride + j) as isize) as core::ffi::c_int
                    * *dy.offset((i * dy_stride + j) as isize) as core::ffi::c_int;
            tmp[3 as core::ffi::c_int as usize]
                += *dy.offset((i * dy_stride + j) as isize) as core::ffi::c_int
                    * *dy.offset((i * dy_stride + j) as isize) as core::ffi::c_int;
            j += 1;
        }
        i += 1;
    }
    tmp[0 as core::ffi::c_int as usize] += 1 as core::ffi::c_int;
    tmp[3 as core::ffi::c_int as usize] += 1 as core::ffi::c_int;
    tmp[2 as core::ffi::c_int as usize] = tmp[1 as core::ffi::c_int as usize];
    *M.offset(0 as core::ffi::c_int as isize) = tmp[0 as core::ffi::c_int as usize]
        as core::ffi::c_double;
    *M.offset(1 as core::ffi::c_int as isize) = tmp[1 as core::ffi::c_int as usize]
        as core::ffi::c_double;
    *M.offset(2 as core::ffi::c_int as isize) = tmp[2 as core::ffi::c_int as usize]
        as core::ffi::c_double;
    *M.offset(3 as core::ffi::c_int as isize) = tmp[3 as core::ffi::c_int as usize]
        as core::ffi::c_double;
}
#[inline]
unsafe extern "C-unwind" fn invert_2x2(
    mut M: *const core::ffi::c_double,
    mut M_inv: *mut core::ffi::c_double,
) {
    let mut det: core::ffi::c_double = *M.offset(0 as core::ffi::c_int as isize)
        * *M.offset(3 as core::ffi::c_int as isize)
        - *M.offset(1 as core::ffi::c_int as isize)
            * *M.offset(2 as core::ffi::c_int as isize);
    if !(det >= 1 as core::ffi::c_int as core::ffi::c_double) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"invert_2x2\0"))
                .as_ptr(),
            b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
            404 as core::ffi::c_int,
            b"det >= 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let det_inv: core::ffi::c_double = 1 as core::ffi::c_int as core::ffi::c_double
        / det;
    *M_inv.offset(0 as core::ffi::c_int as isize) = *M
        .offset(3 as core::ffi::c_int as isize) * det_inv;
    *M_inv.offset(1 as core::ffi::c_int as isize) = -*M
        .offset(1 as core::ffi::c_int as isize) * det_inv;
    *M_inv.offset(2 as core::ffi::c_int as isize) = -*M
        .offset(2 as core::ffi::c_int as isize) * det_inv;
    *M_inv.offset(3 as core::ffi::c_int as isize) = *M
        .offset(0 as core::ffi::c_int as isize) * det_inv;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_compute_flow_at_point_c(
    mut src: *const uint8_t,
    mut ref_0: *const uint8_t,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut u: *mut core::ffi::c_double,
    mut v: *mut core::ffi::c_double,
) {
    let mut M: [core::ffi::c_double; 4] = [0.; 4];
    let mut M_inv: [core::ffi::c_double; 4] = [0.; 4];
    let mut b: [core::ffi::c_int; 2] = [0; 2];
    let mut dx: [int16_t; 64] = [0; 64];
    let mut dy: [int16_t; 64] = [0; 64];
    let mut src_patch: *const uint8_t = &*src.offset((y * stride + x) as isize)
        as *const uint8_t;
    sobel_filter(
        src_patch,
        stride,
        dx.as_mut_ptr(),
        DISFLOW_PATCH_SIZE,
        1 as core::ffi::c_int,
    );
    sobel_filter(
        src_patch,
        stride,
        dy.as_mut_ptr(),
        DISFLOW_PATCH_SIZE,
        0 as core::ffi::c_int,
    );
    compute_flow_matrix(
        dx.as_mut_ptr(),
        DISFLOW_PATCH_SIZE,
        dy.as_mut_ptr(),
        DISFLOW_PATCH_SIZE,
        M.as_mut_ptr(),
    );
    invert_2x2(M.as_mut_ptr(), M_inv.as_mut_ptr());
    let mut itr: core::ffi::c_int = 0 as core::ffi::c_int;
    while itr < DISFLOW_MAX_ITR {
        compute_flow_vector(
            src,
            ref_0,
            width,
            height,
            stride,
            x,
            y,
            *u,
            *v,
            dx.as_mut_ptr(),
            dy.as_mut_ptr(),
            b.as_mut_ptr(),
        );
        let step_u: core::ffi::c_double = M_inv[0 as core::ffi::c_int as usize]
            * b[0 as core::ffi::c_int as usize] as core::ffi::c_double
            + M_inv[1 as core::ffi::c_int as usize]
                * b[1 as core::ffi::c_int as usize] as core::ffi::c_double;
        let step_v: core::ffi::c_double = M_inv[2 as core::ffi::c_int as usize]
            * b[0 as core::ffi::c_int as usize] as core::ffi::c_double
            + M_inv[3 as core::ffi::c_int as usize]
                * b[1 as core::ffi::c_int as usize] as core::ffi::c_double;
        *u
            += fclamp(
                step_u * DISFLOW_STEP_SIZE,
                -(2 as core::ffi::c_int) as core::ffi::c_double,
                2 as core::ffi::c_int as core::ffi::c_double,
            );
        *v
            += fclamp(
                step_v * DISFLOW_STEP_SIZE,
                -(2 as core::ffi::c_int) as core::ffi::c_double,
                2 as core::ffi::c_int as core::ffi::c_double,
            );
        if fabs(step_u) + fabs(step_v) < DISFLOW_STEP_SIZE_THRESOLD {
            break;
        }
        itr += 1;
    }
}
unsafe extern "C-unwind" fn fill_flow_field_borders(
    mut flow: *mut core::ffi::c_double,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
) {
    let left_index: core::ffi::c_int = FLOW_BORDER_INNER;
    let right_index: core::ffi::c_int = width - FLOW_BORDER_INNER
        - 1 as core::ffi::c_int;
    let top_index: core::ffi::c_int = FLOW_BORDER_INNER;
    let bottom_index: core::ffi::c_int = height - FLOW_BORDER_INNER
        - 1 as core::ffi::c_int;
    let mut i: core::ffi::c_int = top_index;
    while i <= bottom_index {
        let mut row: *mut core::ffi::c_double = flow.offset((i * stride) as isize);
        let left: core::ffi::c_double = *row.offset(left_index as isize);
        let mut j: core::ffi::c_int = -FLOW_BORDER_OUTER;
        while j < left_index {
            *row.offset(j as isize) = left;
            j += 1;
        }
        i += 1 as core::ffi::c_int;
    }
    let mut i_0: core::ffi::c_int = top_index;
    while i_0 <= bottom_index {
        let mut row_0: *mut core::ffi::c_double = flow.offset((i_0 * stride) as isize);
        let right: core::ffi::c_double = *row_0.offset(right_index as isize);
        let mut j_0: core::ffi::c_int = right_index + 1 as core::ffi::c_int;
        while j_0 < width + FLOW_BORDER_OUTER {
            *row_0.offset(j_0 as isize) = right;
            j_0 += 1;
        }
        i_0 += 1 as core::ffi::c_int;
    }
    let mut top_row: *const core::ffi::c_double = flow
        .offset((top_index * stride) as isize)
        .offset(-(FLOW_BORDER_OUTER as isize));
    let mut i_1: core::ffi::c_int = -FLOW_BORDER_OUTER;
    while i_1 < top_index {
        let mut row_1: *mut core::ffi::c_double = flow
            .offset((i_1 * stride) as isize)
            .offset(-(FLOW_BORDER_OUTER as isize));
        let mut length: size_t = (width + 2 as core::ffi::c_int * FLOW_BORDER_OUTER)
            as size_t;
        memcpy(
            row_1 as *mut core::ffi::c_void,
            top_row as *const core::ffi::c_void,
            length.wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
        );
        i_1 += 1;
    }
    let mut bottom_row: *const core::ffi::c_double = flow
        .offset((bottom_index * stride) as isize)
        .offset(-(FLOW_BORDER_OUTER as isize));
    let mut i_2: core::ffi::c_int = bottom_index + 1 as core::ffi::c_int;
    while i_2 < height + FLOW_BORDER_OUTER {
        let mut row_2: *mut core::ffi::c_double = flow
            .offset((i_2 * stride) as isize)
            .offset(-(FLOW_BORDER_OUTER as isize));
        let mut length_0: size_t = (width + 2 as core::ffi::c_int * FLOW_BORDER_OUTER)
            as size_t;
        memcpy(
            row_2 as *mut core::ffi::c_void,
            bottom_row as *const core::ffi::c_void,
            length_0
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
        );
        i_2 += 1;
    }
}
unsafe extern "C-unwind" fn upscale_flow_component(
    mut flow: *mut core::ffi::c_double,
    mut cur_width: core::ffi::c_int,
    mut cur_height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut tmpbuf: *mut core::ffi::c_double,
) {
    let half_len: core::ffi::c_int = FLOW_UPSCALE_TAPS / 2 as core::ffi::c_int;
    if !(half_len <= 4 as core::ffi::c_int / 2 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"upscale_flow_component\0"))
                .as_ptr(),
            b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
            548 as core::ffi::c_int,
            b"half_len <= FLOW_BORDER_OUTER\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < cur_height {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < cur_width {
            let mut left: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut k: core::ffi::c_int = -half_len;
            while k < half_len {
                left
                    += *flow.offset((i * stride + (j + k)) as isize)
                        * flow_upscale_filter[0 as core::ffi::c_int
                            as usize][(k + half_len) as usize];
                k += 1;
            }
            *tmpbuf
                .offset(
                    (i * stride + (2 as core::ffi::c_int * j + 0 as core::ffi::c_int))
                        as isize,
                ) = 2.0f64 * left;
            let mut right: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut k_0: core::ffi::c_int = -(half_len - 1 as core::ffi::c_int);
            while k_0 < half_len + 1 as core::ffi::c_int {
                right
                    += *flow.offset((i * stride + (j + k_0)) as isize)
                        * flow_upscale_filter[1 as core::ffi::c_int
                            as usize][(k_0 + (half_len - 1 as core::ffi::c_int))
                            as usize];
                k_0 += 1;
            }
            *tmpbuf
                .offset(
                    (i * stride + (2 as core::ffi::c_int * j + 1 as core::ffi::c_int))
                        as isize,
                ) = 2.0f64 * right;
            j += 1;
        }
        i += 1;
    }
    let mut top_row: *const core::ffi::c_double = &raw mut *tmpbuf
        .offset(0 as core::ffi::c_int as isize) as *mut core::ffi::c_double;
    let mut i_0: core::ffi::c_int = -FLOW_BORDER_OUTER;
    while i_0 < 0 as core::ffi::c_int {
        let mut row: *mut core::ffi::c_double = &raw mut *tmpbuf
            .offset((i_0 * stride) as isize) as *mut core::ffi::c_double;
        memcpy(
            row as *mut core::ffi::c_void,
            top_row as *const core::ffi::c_void,
            ((2 as core::ffi::c_int * cur_width) as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
        );
        i_0 += 1;
    }
    let mut bottom_row: *const core::ffi::c_double = &raw mut *tmpbuf
        .offset(((cur_height - 1 as core::ffi::c_int) * stride) as isize)
        as *mut core::ffi::c_double;
    let mut i_1: core::ffi::c_int = cur_height;
    while i_1 < cur_height + FLOW_BORDER_OUTER {
        let mut row_0: *mut core::ffi::c_double = &raw mut *tmpbuf
            .offset((i_1 * stride) as isize) as *mut core::ffi::c_double;
        memcpy(
            row_0 as *mut core::ffi::c_void,
            bottom_row as *const core::ffi::c_void,
            ((2 as core::ffi::c_int * cur_width) as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
        );
        i_1 += 1;
    }
    let mut upscaled_width: core::ffi::c_int = cur_width * 2 as core::ffi::c_int;
    let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_2 < cur_height {
        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_0 < upscaled_width {
            let mut top: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut k_1: core::ffi::c_int = -half_len;
            while k_1 < half_len {
                top
                    += *tmpbuf.offset(((i_2 + k_1) * stride + j_0) as isize)
                        * flow_upscale_filter[0 as core::ffi::c_int
                            as usize][(k_1 + half_len) as usize];
                k_1 += 1;
            }
            *flow.offset((2 as core::ffi::c_int * i_2 * stride + j_0) as isize) = top;
            let mut bottom: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut k_2: core::ffi::c_int = -(half_len - 1 as core::ffi::c_int);
            while k_2 < half_len + 1 as core::ffi::c_int {
                bottom
                    += *tmpbuf.offset(((i_2 + k_2) * stride + j_0) as isize)
                        * flow_upscale_filter[1 as core::ffi::c_int
                            as usize][(k_2 + (half_len - 1 as core::ffi::c_int))
                            as usize];
                k_2 += 1;
            }
            *flow
                .offset(
                    ((2 as core::ffi::c_int * i_2 + 1 as core::ffi::c_int) * stride
                        + j_0) as isize,
                ) = bottom;
            j_0 += 1;
        }
        i_2 += 1;
    }
}
unsafe extern "C-unwind" fn compute_flow_field(
    mut src_pyr: *const ImagePyramid,
    mut ref_pyr: *const ImagePyramid,
    mut n_levels: core::ffi::c_int,
    mut flow: *mut FlowField,
) -> bool {
    let mut current_block: u64;
    let mut mem_status: bool = true_0 != 0;
    let mut flow_u: *mut core::ffi::c_double = (*flow).u;
    let mut flow_v: *mut core::ffi::c_double = (*flow).v;
    let mut tmpbuf0: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut tmpbuf: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    if n_levels < 2 as core::ffi::c_int {
        tmpbuf0 = 0 as *mut core::ffi::c_double;
        tmpbuf = 0 as *mut core::ffi::c_double;
        current_block = 0;
    } else {
        let layer1_height: core::ffi::c_int = (*((*src_pyr).layers)
            .offset(1 as core::ffi::c_int as isize))
            .height >> DOWNSAMPLE_SHIFT;
        let tmpbuf_size: size_t = ((layer1_height
            + 2 as core::ffi::c_int * FLOW_BORDER_OUTER) * (*flow).stride) as size_t;
        tmpbuf0 = aom_malloc(
            tmpbuf_size
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
        ) as *mut core::ffi::c_double;
        if tmpbuf0.is_null() {
            mem_status = false_0 != 0;
            current_block = 1;
        } else {
            tmpbuf = tmpbuf0.offset((FLOW_BORDER_OUTER * (*flow).stride) as isize);
            current_block = 0;
        }
    }
    match current_block {
        0 => {
            let mut level: core::ffi::c_int = n_levels - 1 as core::ffi::c_int;
            while level >= 1 as core::ffi::c_int {
                let mut cur_layer: *const PyramidLayer = &raw mut *((*src_pyr).layers)
                    .offset(level as isize) as *mut PyramidLayer;
                let cur_width: core::ffi::c_int = (*cur_layer).width;
                let cur_height: core::ffi::c_int = (*cur_layer).height;
                let cur_stride: core::ffi::c_int = (*cur_layer).stride;
                let mut src_buffer: *const uint8_t = (*cur_layer).buffer;
                let mut ref_buffer: *const uint8_t = (*((*ref_pyr).layers)
                    .offset(level as isize))
                    .buffer;
                let cur_flow_width: core::ffi::c_int = cur_width >> DOWNSAMPLE_SHIFT;
                let cur_flow_height: core::ffi::c_int = cur_height >> DOWNSAMPLE_SHIFT;
                let cur_flow_stride: core::ffi::c_int = (*flow).stride;
                let mut i: core::ffi::c_int = FLOW_BORDER_INNER;
                while i < cur_flow_height - FLOW_BORDER_INNER {
                    let mut j: core::ffi::c_int = FLOW_BORDER_INNER;
                    while j < cur_flow_width - FLOW_BORDER_INNER {
                        let flow_field_idx: core::ffi::c_int = i * cur_flow_stride + j;
                        let patch_center_x: core::ffi::c_int = (j << DOWNSAMPLE_SHIFT)
                            + UPSAMPLE_CENTER_OFFSET;
                        let patch_center_y: core::ffi::c_int = (i << DOWNSAMPLE_SHIFT)
                            + UPSAMPLE_CENTER_OFFSET;
                        let patch_tl_x: core::ffi::c_int = patch_center_x
                            - DISFLOW_PATCH_CENTER;
                        let patch_tl_y: core::ffi::c_int = patch_center_y
                            - DISFLOW_PATCH_CENTER;
                        if !(patch_tl_x >= 0 as core::ffi::c_int) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 19],
                                    [core::ffi::c_char; 19],
                                >(*b"compute_flow_field\0"))
                                    .as_ptr(),
                                b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
                                669 as core::ffi::c_int,
                                b"patch_tl_x >= 0\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        } else {};
                        if !(patch_tl_y >= 0 as core::ffi::c_int) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 19],
                                    [core::ffi::c_char; 19],
                                >(*b"compute_flow_field\0"))
                                    .as_ptr(),
                                b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
                                670 as core::ffi::c_int,
                                b"patch_tl_y >= 0\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        } else {};
                        aom_compute_flow_at_point_c(
                            src_buffer,
                            ref_buffer,
                            patch_tl_x,
                            patch_tl_y,
                            cur_width,
                            cur_height,
                            cur_stride,
                            &raw mut *flow_u.offset(flow_field_idx as isize),
                            &raw mut *flow_v.offset(flow_field_idx as isize),
                        );
                        j += 1 as core::ffi::c_int;
                    }
                    i += 1 as core::ffi::c_int;
                }
                fill_flow_field_borders(
                    flow_u,
                    cur_flow_width,
                    cur_flow_height,
                    cur_flow_stride,
                );
                fill_flow_field_borders(
                    flow_v,
                    cur_flow_width,
                    cur_flow_height,
                    cur_flow_stride,
                );
                if level > 0 as core::ffi::c_int {
                    let upscale_flow_width: core::ffi::c_int = cur_flow_width
                        << 1 as core::ffi::c_int;
                    let upscale_flow_height: core::ffi::c_int = cur_flow_height
                        << 1 as core::ffi::c_int;
                    let upscale_stride: core::ffi::c_int = (*flow).stride;
                    upscale_flow_component(
                        flow_u,
                        cur_flow_width,
                        cur_flow_height,
                        cur_flow_stride,
                        tmpbuf,
                    );
                    upscale_flow_component(
                        flow_v,
                        cur_flow_width,
                        cur_flow_height,
                        cur_flow_stride,
                        tmpbuf,
                    );
                    let mut next_layer: *const PyramidLayer = &raw mut *((*src_pyr).layers)
                        .offset((level - 1 as core::ffi::c_int) as isize)
                        as *mut PyramidLayer;
                    let next_flow_width: core::ffi::c_int = (*next_layer).width
                        >> DOWNSAMPLE_SHIFT;
                    let next_flow_height: core::ffi::c_int = (*next_layer).height
                        >> DOWNSAMPLE_SHIFT;
                    if next_flow_width > upscale_flow_width {
                        if !(next_flow_width
                            == upscale_flow_width + 1 as core::ffi::c_int)
                            as core::ffi::c_int as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 19],
                                    [core::ffi::c_char; 19],
                                >(*b"compute_flow_field\0"))
                                    .as_ptr(),
                                b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
                                705 as core::ffi::c_int,
                                b"next_flow_width == upscale_flow_width + 1\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        } else {};
                        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                        while i_0 < upscale_flow_height {
                            let index: core::ffi::c_int = i_0 * upscale_stride
                                + upscale_flow_width;
                            *flow_u.offset(index as isize) = *flow_u
                                .offset((index - 1 as core::ffi::c_int) as isize);
                            *flow_v.offset(index as isize) = *flow_v
                                .offset((index - 1 as core::ffi::c_int) as isize);
                            i_0 += 1;
                        }
                    }
                    if next_flow_height > upscale_flow_height {
                        if !(next_flow_height
                            == upscale_flow_height + 1 as core::ffi::c_int)
                            as core::ffi::c_int as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 19],
                                    [core::ffi::c_char; 19],
                                >(*b"compute_flow_field\0"))
                                    .as_ptr(),
                                b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
                                715 as core::ffi::c_int,
                                b"next_flow_height == upscale_flow_height + 1\0"
                                    as *const u8 as *const core::ffi::c_char,
                            );
                        } else {};
                        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
                        while j_0 < next_flow_width {
                            let index_0: core::ffi::c_int = upscale_flow_height
                                * upscale_stride + j_0;
                            *flow_u.offset(index_0 as isize) = *flow_u
                                .offset((index_0 - upscale_stride) as isize);
                            *flow_v.offset(index_0 as isize) = *flow_v
                                .offset((index_0 - upscale_stride) as isize);
                            j_0 += 1;
                        }
                    }
                }
                level -= 1;
            }
        }
        _ => {}
    }
    aom_free(tmpbuf0 as *mut core::ffi::c_void);
    return mem_status;
}
unsafe extern "C-unwind" fn alloc_flow_field(
    mut frame_width: core::ffi::c_int,
    mut frame_height: core::ffi::c_int,
) -> *mut FlowField {
    let mut flow: *mut FlowField = aom_malloc(
        ::core::mem::size_of::<FlowField>() as size_t,
    ) as *mut FlowField;
    if flow.is_null() {
        return 0 as *mut FlowField;
    }
    (*flow).width = frame_width >> DOWNSAMPLE_SHIFT;
    (*flow).height = frame_height >> DOWNSAMPLE_SHIFT;
    (*flow).stride = (*flow).width + 2 as core::ffi::c_int * FLOW_BORDER_OUTER;
    let flow_size: size_t = ((*flow).stride as size_t)
        .wrapping_mul(
            ((*flow).height + 2 as core::ffi::c_int * FLOW_BORDER_OUTER) as size_t,
        );
    (*flow).buf0 = aom_calloc(
        (2 as size_t).wrapping_mul(flow_size),
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    if ((*flow).buf0).is_null() {
        aom_free(flow as *mut core::ffi::c_void);
        return 0 as *mut FlowField;
    }
    (*flow).u = ((*flow).buf0)
        .offset((FLOW_BORDER_OUTER * (*flow).stride) as isize)
        .offset(FLOW_BORDER_OUTER as isize);
    (*flow).v = ((*flow).u).offset(flow_size as isize);
    return flow;
}
unsafe extern "C-unwind" fn free_flow_field(mut flow: *mut FlowField) {
    aom_free((*flow).buf0 as *mut core::ffi::c_void);
    aom_free(flow as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_compute_global_motion_disflow(
    mut type_0: TransformationType,
    mut src: *mut YV12_BUFFER_CONFIG,
    mut ref_0: *mut YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut downsample_level: core::ffi::c_int,
    mut motion_models: *mut MotionModel,
    mut num_motion_models: core::ffi::c_int,
    mut mem_alloc_failed: *mut bool,
) -> bool {
    let mut src_pyramid: *mut ImagePyramid = (*src).y_pyramid as *mut ImagePyramid;
    let mut src_corners: *mut CornerList = (*src).corners as *mut CornerList;
    let mut ref_pyramid: *mut ImagePyramid = (*ref_0).y_pyramid as *mut ImagePyramid;
    let src_layers: core::ffi::c_int = aom_compute_pyramid(
        src,
        bit_depth,
        DISFLOW_PYRAMID_LEVELS,
        src_pyramid,
    ) as core::ffi::c_int;
    let ref_layers: core::ffi::c_int = aom_compute_pyramid(
        ref_0,
        bit_depth,
        DISFLOW_PYRAMID_LEVELS,
        ref_pyramid,
    ) as core::ffi::c_int;
    if src_layers < 0 as core::ffi::c_int || ref_layers < 0 as core::ffi::c_int {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    if !av1_compute_corner_list(src, bit_depth, downsample_level, src_corners) {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    if !(src_layers == ref_layers) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_compute_global_motion_disflow\0"))
                .as_ptr(),
            b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
            788 as core::ffi::c_int,
            b"src_layers == ref_layers\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let src_width: core::ffi::c_int = (*((*src_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .width;
    let src_height: core::ffi::c_int = (*((*src_pyramid).layers)
        .offset(0 as core::ffi::c_int as isize))
        .height;
    if !((*((*ref_pyramid).layers).offset(0 as core::ffi::c_int as isize)).width
        == src_width) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_compute_global_motion_disflow\0"))
                .as_ptr(),
            b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
            792 as core::ffi::c_int,
            b"ref_pyramid->layers[0].width == src_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*((*ref_pyramid).layers).offset(0 as core::ffi::c_int as isize)).height
        == src_height) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"av1_compute_global_motion_disflow\0"))
                .as_ptr(),
            b"disflow.c\0" as *const u8 as *const core::ffi::c_char,
            793 as core::ffi::c_int,
            b"ref_pyramid->layers[0].height == src_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut flow: *mut FlowField = alloc_flow_field(src_width, src_height);
    if flow.is_null() {
        *mem_alloc_failed = true_0 != 0;
        return false_0 != 0;
    }
    if !compute_flow_field(src_pyramid, ref_pyramid, src_layers, flow) {
        *mem_alloc_failed = true_0 != 0;
        free_flow_field(flow);
        return false_0 != 0;
    }
    let mut correspondences: *mut Correspondence = aom_malloc(
        ((*src_corners).num_corners as size_t)
            .wrapping_mul(::core::mem::size_of::<Correspondence>() as size_t),
    ) as *mut Correspondence;
    if correspondences.is_null() {
        *mem_alloc_failed = true_0 != 0;
        free_flow_field(flow);
        return false_0 != 0;
    }
    let num_correspondences: core::ffi::c_int = determine_disflow_correspondence(
        src_pyramid,
        ref_pyramid,
        src_corners,
        flow,
        correspondences,
    ) as core::ffi::c_int;
    let mut result: bool = ransac(
        correspondences,
        num_correspondences,
        type_0,
        motion_models,
        num_motion_models,
        mem_alloc_failed,
    );
    aom_free(correspondences as *mut core::ffi::c_void);
    free_flow_field(flow);
    return result;
}

extern "C" {
    fn fabs(_: core::ffi::c_double) -> core::ffi::c_double;
    fn floor(_: core::ffi::c_double) -> core::ffi::c_double;
    fn rint(_: core::ffi::c_double) -> core::ffi::c_double;
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
