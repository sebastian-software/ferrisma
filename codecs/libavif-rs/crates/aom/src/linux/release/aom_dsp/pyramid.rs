/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/pyramid.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn av1_resize_plane(
        input: *const uint8_t,
        height: core::ffi::c_int,
        width: core::ffi::c_int,
        in_stride: core::ffi::c_int,
        output: *mut uint8_t,
        height2: core::ffi::c_int,
        width2: core::ffi::c_int,
        out_stride: core::ffi::c_int,
    ) -> bool;
    fn av1_resize_plane_to_half(
        input: *const uint8_t,
        height: core::ffi::c_int,
        width: core::ffi::c_int,
        in_stride: core::ffi::c_int,
        output: *mut uint8_t,
        height2: core::ffi::c_int,
        width2: core::ffi::c_int,
        out_stride: core::ffi::c_int,
    ) -> bool;
    fn should_resize_by_half(
        height: core::ffi::c_int,
        width: core::ffi::c_int,
        height2: core::ffi::c_int,
        width2: core::ffi::c_int,
    ) -> bool;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [core::ffi::c_char; 4],
    pub __align: core::ffi::c_int,
}
pub type ImagePyramid = image_pyramid;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const PYRAMID_PADDING: core::ffi::c_int = 16 as core::ffi::c_int;
pub const PYRAMID_ALIGNMENT: core::ffi::c_int = 32 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_pyramid_alloc_size(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut image_is_16bit: bool,
) -> size_t {
    let msb: core::ffi::c_int = get_msb(
        (if width < height { width } else { height }) as core::ffi::c_uint,
    ) as core::ffi::c_int;
    let n_levels: core::ffi::c_int = if msb - 3 as core::ffi::c_int
        > 1 as core::ffi::c_int
    {
        msb - 3 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut alloc_size: size_t = 0 as size_t;
    alloc_size = (alloc_size as core::ffi::c_ulong)
        .wrapping_add(
            ::core::mem::size_of::<ImagePyramid>() as usize as core::ffi::c_ulong,
        ) as size_t as size_t;
    alloc_size = (alloc_size as core::ffi::c_ulong)
        .wrapping_add(
            (n_levels as usize)
                .wrapping_mul(::core::mem::size_of::<PyramidLayer>() as usize)
                as core::ffi::c_ulong,
        ) as size_t as size_t;
    let mut buffer_size: size_t = 0 as size_t;
    let mut first_px_offset: size_t = (PYRAMID_PADDING + PYRAMID_ALIGNMENT
        - 1 as core::ffi::c_int & !(PYRAMID_ALIGNMENT - 1 as core::ffi::c_int))
        as size_t;
    let mut extra_bytes: size_t = first_px_offset
        .wrapping_sub(PYRAMID_PADDING as size_t);
    buffer_size = buffer_size.wrapping_add(extra_bytes);
    let mut first_allocated_level: core::ffi::c_int = if image_is_16bit
        as core::ffi::c_int != 0
    {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut level: core::ffi::c_int = first_allocated_level;
    while level < n_levels {
        let mut level_width: core::ffi::c_int = width >> level;
        let mut level_height: core::ffi::c_int = height >> level;
        let mut padded_width: core::ffi::c_int = level_width
            + 2 as core::ffi::c_int * PYRAMID_PADDING;
        let mut padded_height: core::ffi::c_int = level_height
            + 2 as core::ffi::c_int * PYRAMID_PADDING;
        let mut level_stride: core::ffi::c_int = padded_width + PYRAMID_ALIGNMENT
            - 1 as core::ffi::c_int & !(PYRAMID_ALIGNMENT - 1 as core::ffi::c_int);
        buffer_size = buffer_size.wrapping_add((level_stride * padded_height) as size_t);
        level += 1;
    }
    alloc_size = alloc_size.wrapping_add(buffer_size);
    return alloc_size;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_alloc_pyramid(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut image_is_16bit: bool,
) -> *mut ImagePyramid {
    let msb: core::ffi::c_int = get_msb(
        (if width < height { width } else { height }) as core::ffi::c_uint,
    ) as core::ffi::c_int;
    let n_levels: core::ffi::c_int = if msb - 3 as core::ffi::c_int
        > 1 as core::ffi::c_int
    {
        msb - 3 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut pyr: *mut ImagePyramid = aom_calloc(
        1 as size_t,
        ::core::mem::size_of::<ImagePyramid>() as size_t,
    ) as *mut ImagePyramid;
    if pyr.is_null() {
        return 0 as *mut ImagePyramid;
    }
    (*pyr).layers = aom_calloc(
        n_levels as size_t,
        ::core::mem::size_of::<PyramidLayer>() as size_t,
    ) as *mut PyramidLayer;
    if ((*pyr).layers).is_null() {
        aom_free(pyr as *mut core::ffi::c_void);
        return 0 as *mut ImagePyramid;
    }
    (*pyr).max_levels = n_levels;
    (*pyr).filled_levels = 0 as core::ffi::c_int;
    let mut buffer_size: size_t = 0 as size_t;
    let mut layer_offsets: *mut size_t = aom_calloc(
        n_levels as size_t,
        ::core::mem::size_of::<size_t>() as size_t,
    ) as *mut size_t;
    if layer_offsets.is_null() {
        aom_free((*pyr).layers as *mut core::ffi::c_void);
        aom_free(pyr as *mut core::ffi::c_void);
        return 0 as *mut ImagePyramid;
    }
    let mut first_px_offset: size_t = (PYRAMID_PADDING + PYRAMID_ALIGNMENT
        - 1 as core::ffi::c_int & !(PYRAMID_ALIGNMENT - 1 as core::ffi::c_int))
        as size_t;
    let mut extra_bytes: size_t = first_px_offset
        .wrapping_sub(PYRAMID_PADDING as size_t);
    buffer_size = buffer_size.wrapping_add(extra_bytes);
    let mut first_allocated_level: core::ffi::c_int = if image_is_16bit
        as core::ffi::c_int != 0
    {
        0 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut level: core::ffi::c_int = first_allocated_level;
    while level < n_levels {
        let mut layer: *mut PyramidLayer = &raw mut *((*pyr).layers).offset(level as isize)
            as *mut PyramidLayer;
        let mut level_width: core::ffi::c_int = width >> level;
        let mut level_height: core::ffi::c_int = height >> level;
        let mut padded_width: core::ffi::c_int = level_width
            + 2 as core::ffi::c_int * PYRAMID_PADDING;
        let mut padded_height: core::ffi::c_int = level_height
            + 2 as core::ffi::c_int * PYRAMID_PADDING;
        let mut level_stride: core::ffi::c_int = padded_width + PYRAMID_ALIGNMENT
            - 1 as core::ffi::c_int & !(PYRAMID_ALIGNMENT - 1 as core::ffi::c_int);
        let mut level_alloc_start: size_t = buffer_size;
        let mut level_start: size_t = level_alloc_start
            .wrapping_add((PYRAMID_PADDING * level_stride) as size_t)
            .wrapping_add(PYRAMID_PADDING as size_t);
        buffer_size = buffer_size.wrapping_add((level_stride * padded_height) as size_t);
        *layer_offsets.offset(level as isize) = level_start;
        (*layer).width = level_width;
        (*layer).height = level_height;
        (*layer).stride = level_stride;
        level += 1;
    }
    (*pyr).buffer_alloc = aom_memalign(
        PYRAMID_ALIGNMENT as size_t,
        buffer_size.wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    ) as *mut uint8_t;
    if ((*pyr).buffer_alloc).is_null() {
        aom_free((*pyr).layers as *mut core::ffi::c_void);
        aom_free(pyr as *mut core::ffi::c_void);
        aom_free(layer_offsets as *mut core::ffi::c_void);
        return 0 as *mut ImagePyramid;
    }
    let mut level_0: core::ffi::c_int = first_allocated_level;
    while level_0 < n_levels {
        let mut layer_0: *mut PyramidLayer = &raw mut *((*pyr).layers)
            .offset(level_0 as isize) as *mut PyramidLayer;
        (*layer_0).buffer = ((*pyr).buffer_alloc)
            .offset(*layer_offsets.offset(level_0 as isize) as isize);
        level_0 += 1;
    }
    pthread_mutex_init(&raw mut (*pyr).mutex, 0 as *const pthread_mutexattr_t);
    aom_free(layer_offsets as *mut core::ffi::c_void);
    return pyr;
}
#[inline]
unsafe extern "C-unwind" fn fill_border(
    mut img_buf: *mut uint8_t,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    stride: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < height {
        let mut row_start: *mut uint8_t = &raw mut *img_buf.offset((row * stride) as isize)
            as *mut uint8_t;
        let mut left_pixel: uint8_t = *row_start.offset(0 as core::ffi::c_int as isize);
        memset(
            row_start.offset(-(PYRAMID_PADDING as isize)) as *mut core::ffi::c_void,
            left_pixel as core::ffi::c_int,
            PYRAMID_PADDING as size_t,
        );
        let mut right_pixel: uint8_t = *row_start
            .offset((width - 1 as core::ffi::c_int) as isize);
        memset(
            row_start.offset(width as isize) as *mut core::ffi::c_void,
            right_pixel as core::ffi::c_int,
            PYRAMID_PADDING as size_t,
        );
        row += 1;
    }
    let mut row_0: core::ffi::c_int = -PYRAMID_PADDING;
    while row_0 < 0 as core::ffi::c_int {
        let mut row_start_0: *mut uint8_t = &raw mut *img_buf
            .offset((row_0 * stride) as isize) as *mut uint8_t;
        memcpy(
            row_start_0.offset(-(PYRAMID_PADDING as isize)) as *mut core::ffi::c_void,
            img_buf.offset(-(PYRAMID_PADDING as isize)) as *const core::ffi::c_void,
            (width + 2 as core::ffi::c_int * PYRAMID_PADDING) as size_t,
        );
        row_0 += 1;
    }
    let mut last_row_start: *mut uint8_t = &raw mut *img_buf
        .offset(((height - 1 as core::ffi::c_int) * stride) as isize) as *mut uint8_t;
    let mut row_1: core::ffi::c_int = height;
    while row_1 < height + PYRAMID_PADDING {
        let mut row_start_1: *mut uint8_t = &raw mut *img_buf
            .offset((row_1 * stride) as isize) as *mut uint8_t;
        memcpy(
            row_start_1.offset(-(PYRAMID_PADDING as isize)) as *mut core::ffi::c_void,
            last_row_start.offset(-(PYRAMID_PADDING as isize))
                as *const core::ffi::c_void,
            (width + 2 as core::ffi::c_int * PYRAMID_PADDING) as size_t,
        );
        row_1 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn fill_pyramid(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut n_levels: core::ffi::c_int,
    mut frame_pyr: *mut ImagePyramid,
) -> core::ffi::c_int {
    let mut already_filled_levels: core::ffi::c_int = (*frame_pyr).filled_levels;
    if already_filled_levels >= n_levels {
        return n_levels;
    }
    let frame_width: core::ffi::c_int = (*frame)
        .c2rust_unnamed_1
        .c2rust_unnamed
        .y_crop_width;
    let frame_height: core::ffi::c_int = (*frame)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .y_crop_height;
    let frame_stride: core::ffi::c_int = (*frame)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .y_stride;
    if already_filled_levels == 0 as core::ffi::c_int {
        let mut first_layer: *mut PyramidLayer = &raw mut *((*frame_pyr).layers)
            .offset(0 as core::ffi::c_int as isize) as *mut PyramidLayer;
        if (*frame).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
            let mut frame_buffer: *mut uint16_t = (((*frame)
                .c2rust_unnamed_4
                .c2rust_unnamed
                .y_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t;
            let mut pyr_buffer: *mut uint8_t = (*first_layer).buffer;
            let mut pyr_stride: core::ffi::c_int = (*first_layer).stride;
            let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
            while y < frame_height {
                let mut frame_row: *mut uint16_t = frame_buffer
                    .offset((y * frame_stride) as isize);
                let mut pyr_row: *mut uint8_t = pyr_buffer
                    .offset((y * pyr_stride) as isize);
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < frame_width {
                    *pyr_row.offset(x as isize) = (*frame_row.offset(x as isize)
                        as core::ffi::c_int >> bit_depth - 8 as core::ffi::c_int)
                        as uint8_t;
                    x += 1;
                }
                y += 1;
            }
            fill_border(pyr_buffer, frame_width, frame_height, pyr_stride);
        } else {
            (*first_layer).buffer = (*frame).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
            (*first_layer).width = frame_width;
            (*first_layer).height = frame_height;
            (*first_layer).stride = frame_stride;
        }
        already_filled_levels = 1 as core::ffi::c_int;
    }
    let mut level: core::ffi::c_int = already_filled_levels;
    while level < n_levels {
        let mut mem_status: bool = false_0 != 0;
        let mut prev_layer: *mut PyramidLayer = &raw mut *((*frame_pyr).layers)
            .offset((level - 1 as core::ffi::c_int) as isize) as *mut PyramidLayer;
        let mut prev_buffer: *mut uint8_t = (*prev_layer).buffer;
        let mut prev_stride: core::ffi::c_int = (*prev_layer).stride;
        let mut this_layer: *mut PyramidLayer = &raw mut *((*frame_pyr).layers)
            .offset(level as isize) as *mut PyramidLayer;
        let mut this_buffer: *mut uint8_t = (*this_layer).buffer;
        let mut this_width: core::ffi::c_int = (*this_layer).width;
        let mut this_height: core::ffi::c_int = (*this_layer).height;
        let mut this_stride: core::ffi::c_int = (*this_layer).stride;
        let input_layer_width: core::ffi::c_int = this_width << 1 as core::ffi::c_int;
        let input_layer_height: core::ffi::c_int = this_height << 1 as core::ffi::c_int;
        if should_resize_by_half(
            input_layer_height,
            input_layer_width,
            this_height,
            this_width,
        ) {
            mem_status = av1_resize_plane_to_half(
                prev_buffer,
                input_layer_height,
                input_layer_width,
                prev_stride,
                this_buffer,
                this_height,
                this_width,
                this_stride,
            );
        } else {
            mem_status = av1_resize_plane(
                prev_buffer,
                input_layer_height,
                input_layer_width,
                prev_stride,
                this_buffer,
                this_height,
                this_width,
                this_stride,
            );
        }
        if !mem_status {
            (*frame_pyr).filled_levels = n_levels;
            return -(1 as core::ffi::c_int);
        }
        fill_border(this_buffer, this_width, this_height, this_stride);
        level += 1;
    }
    (*frame_pyr).filled_levels = n_levels;
    return n_levels;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_compute_pyramid(
    mut frame: *const YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut n_levels: core::ffi::c_int,
    mut pyr: *mut ImagePyramid,
) -> core::ffi::c_int {
    pthread_mutex_lock(&raw mut (*pyr).mutex);
    n_levels = if n_levels < (*pyr).max_levels { n_levels } else { (*pyr).max_levels };
    let mut result: core::ffi::c_int = n_levels;
    if (*pyr).filled_levels < n_levels {
        result = fill_pyramid(frame, bit_depth, n_levels, pyr);
    }
    pthread_mutex_unlock(&raw mut (*pyr).mutex);
    return result;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_invalidate_pyramid(mut pyr: *mut ImagePyramid) {
    if !pyr.is_null() {
        pthread_mutex_lock(&raw mut (*pyr).mutex);
        (*pyr).filled_levels = 0 as core::ffi::c_int;
        pthread_mutex_unlock(&raw mut (*pyr).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_free_pyramid(mut pyr: *mut ImagePyramid) {
    if !pyr.is_null() {
        pthread_mutex_destroy(&raw mut (*pyr).mutex);
        aom_free((*pyr).buffer_alloc as *mut core::ffi::c_void);
        aom_free((*pyr).layers as *mut core::ffi::c_void);
        aom_free(pyr as *mut core::ffi::c_void);
    }
}

extern "C" {
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
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
}
