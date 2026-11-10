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

// Mechanically translated from upstream/aom/aom_scale/generic/yv12extend.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type corner_list;
    pub type image_pyramid;
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
    fn aom_free_frame_buffer(ybf: *mut YV12_BUFFER_CONFIG) -> core::ffi::c_int;
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
pub type size_t = usize;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
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
pub type uint16_t = __uint16_t;
pub type __uint16_t = u16;
pub type uintptr_t = usize;
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
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
unsafe extern "C-unwind" fn extend_plane(
    src: *mut uint8_t,
    mut src_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut extend_top: core::ffi::c_int,
    mut extend_left: core::ffi::c_int,
    mut extend_bottom: core::ffi::c_int,
    mut extend_right: core::ffi::c_int,
    mut v_start: core::ffi::c_int,
    mut v_end: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let linesize: core::ffi::c_int = extend_left + extend_right + width;
    let mut src_ptr1: *mut uint8_t = src.offset((v_start * src_stride) as isize);
    let mut src_ptr2: *mut uint8_t = src
        .offset((v_start * src_stride) as isize)
        .offset(width as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut dst_ptr1: *mut uint8_t = src
        .offset((v_start * src_stride) as isize)
        .offset(-(extend_left as isize));
    let mut dst_ptr2: *mut uint8_t = src_ptr2.offset(1 as core::ffi::c_int as isize);
    i = v_start;
    while i < v_end {
        memset(
            dst_ptr1 as *mut core::ffi::c_void,
            *src_ptr1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_left as size_t,
        );
        memset(
            dst_ptr2 as *mut core::ffi::c_void,
            *src_ptr2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_right as size_t,
        );
        src_ptr1 = src_ptr1.offset(src_stride as isize);
        src_ptr2 = src_ptr2.offset(src_stride as isize);
        dst_ptr1 = dst_ptr1.offset(src_stride as isize);
        dst_ptr2 = dst_ptr2.offset(src_stride as isize);
        i += 1;
    }
    src_ptr1 = src.offset(-(extend_left as isize));
    dst_ptr1 = src_ptr1.offset((src_stride * -extend_top) as isize);
    i = 0 as core::ffi::c_int;
    while i < extend_top {
        memcpy(
            dst_ptr1 as *mut core::ffi::c_void,
            src_ptr1 as *const core::ffi::c_void,
            linesize as size_t,
        );
        dst_ptr1 = dst_ptr1.offset(src_stride as isize);
        i += 1;
    }
    src_ptr2 = src_ptr1.offset((src_stride * (height - 1 as core::ffi::c_int)) as isize);
    dst_ptr2 = src_ptr2;
    i = 0 as core::ffi::c_int;
    while i < extend_bottom {
        dst_ptr2 = dst_ptr2.offset(src_stride as isize);
        memcpy(
            dst_ptr2 as *mut core::ffi::c_void,
            src_ptr2 as *const core::ffi::c_void,
            linesize as size_t,
        );
        i += 1;
    }
}
unsafe extern "C-unwind" fn extend_plane_high(
    src8: *mut uint8_t,
    mut src_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut extend_top: core::ffi::c_int,
    mut extend_left: core::ffi::c_int,
    mut extend_bottom: core::ffi::c_int,
    mut extend_right: core::ffi::c_int,
    mut v_start: core::ffi::c_int,
    mut v_end: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let linesize: core::ffi::c_int = extend_left + extend_right + width;
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut src_ptr1: *mut uint16_t = src.offset((v_start * src_stride) as isize);
    let mut src_ptr2: *mut uint16_t = src
        .offset((v_start * src_stride) as isize)
        .offset(width as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut dst_ptr1: *mut uint16_t = src
        .offset((v_start * src_stride) as isize)
        .offset(-(extend_left as isize));
    let mut dst_ptr2: *mut uint16_t = src_ptr2.offset(1 as core::ffi::c_int as isize);
    i = v_start;
    while i < v_end {
        aom_memset16(
            dst_ptr1 as *mut core::ffi::c_void,
            *src_ptr1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_left as size_t,
        );
        aom_memset16(
            dst_ptr2 as *mut core::ffi::c_void,
            *src_ptr2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_right as size_t,
        );
        src_ptr1 = src_ptr1.offset(src_stride as isize);
        src_ptr2 = src_ptr2.offset(src_stride as isize);
        dst_ptr1 = dst_ptr1.offset(src_stride as isize);
        dst_ptr2 = dst_ptr2.offset(src_stride as isize);
        i += 1;
    }
    src_ptr1 = src.offset(-(extend_left as isize));
    dst_ptr1 = src_ptr1.offset((src_stride * -extend_top) as isize);
    i = 0 as core::ffi::c_int;
    while i < extend_top {
        memcpy(
            dst_ptr1 as *mut core::ffi::c_void,
            src_ptr1 as *const core::ffi::c_void,
            (linesize as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        dst_ptr1 = dst_ptr1.offset(src_stride as isize);
        i += 1;
    }
    src_ptr2 = src_ptr1.offset((src_stride * (height - 1 as core::ffi::c_int)) as isize);
    dst_ptr2 = src_ptr2;
    i = 0 as core::ffi::c_int;
    while i < extend_bottom {
        dst_ptr2 = dst_ptr2.offset(src_stride as isize);
        memcpy(
            dst_ptr2 as *mut core::ffi::c_void,
            src_ptr2 as *const core::ffi::c_void,
            (linesize as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_extend_frame_borders_plane_row_c(
    mut ybf: *const YV12_BUFFER_CONFIG,
    mut plane: core::ffi::c_int,
    mut v_start: core::ffi::c_int,
    mut v_end: core::ffi::c_int,
) {
    let ext_size: core::ffi::c_int = (*ybf).border;
    let ss_x: core::ffi::c_int = (*ybf).subsampling_x;
    let ss_y: core::ffi::c_int = (*ybf).subsampling_y;
    let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int) as core::ffi::c_int;
    let top: core::ffi::c_int = ext_size
        >> (if is_uv != 0 { ss_y } else { 0 as core::ffi::c_int });
    let left: core::ffi::c_int = ext_size
        >> (if is_uv != 0 { ss_x } else { 0 as core::ffi::c_int });
    let bottom: core::ffi::c_int = top + (*ybf).c2rust_unnamed_0.heights[is_uv as usize]
        - (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize];
    let right: core::ffi::c_int = left + (*ybf).c2rust_unnamed.widths[is_uv as usize]
        - (*ybf).c2rust_unnamed_1.crop_widths[is_uv as usize];
    let extend_top_border: core::ffi::c_int = (v_start == 0 as core::ffi::c_int)
        as core::ffi::c_int;
    let extend_bottom_border: core::ffi::c_int = (v_end
        == (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize]) as core::ffi::c_int;
    if (*ybf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        extend_plane_high(
            (*ybf).c2rust_unnamed_4.buffers[plane as usize],
            (*ybf).c2rust_unnamed_3.strides[is_uv as usize],
            (*ybf).c2rust_unnamed_1.crop_widths[is_uv as usize],
            (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize],
            if extend_top_border != 0 { top } else { 0 as core::ffi::c_int },
            left,
            if extend_bottom_border != 0 { bottom } else { 0 as core::ffi::c_int },
            right,
            v_start,
            v_end,
        );
        return;
    }
    extend_plane(
        (*ybf).c2rust_unnamed_4.buffers[plane as usize],
        (*ybf).c2rust_unnamed_3.strides[is_uv as usize],
        (*ybf).c2rust_unnamed_1.crop_widths[is_uv as usize],
        (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize],
        if extend_top_border != 0 { top } else { 0 as core::ffi::c_int },
        left,
        if extend_bottom_border != 0 { bottom } else { 0 as core::ffi::c_int },
        right,
        v_start,
        v_end,
    );
}
unsafe extern "C-unwind" fn extend_frame(
    ybf: *mut YV12_BUFFER_CONFIG,
    mut ext_size: core::ffi::c_int,
    num_planes: core::ffi::c_int,
) {
    let ss_x: core::ffi::c_int = (*ybf).subsampling_x;
    let ss_y: core::ffi::c_int = (*ybf).subsampling_y;
    if (*ybf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane < num_planes {
            let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int)
                as core::ffi::c_int;
            let top: core::ffi::c_int = ext_size
                >> (if is_uv != 0 { ss_y } else { 0 as core::ffi::c_int });
            let left: core::ffi::c_int = ext_size
                >> (if is_uv != 0 { ss_x } else { 0 as core::ffi::c_int });
            let bottom: core::ffi::c_int = top
                + (*ybf).c2rust_unnamed_0.heights[is_uv as usize]
                - (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize];
            let right: core::ffi::c_int = left
                + (*ybf).c2rust_unnamed.widths[is_uv as usize]
                - (*ybf).c2rust_unnamed_1.crop_widths[is_uv as usize];
            extend_plane_high(
                (*ybf).c2rust_unnamed_4.buffers[plane as usize],
                (*ybf).c2rust_unnamed_3.strides[is_uv as usize],
                (*ybf).c2rust_unnamed_1.crop_widths[is_uv as usize],
                (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize],
                top,
                left,
                bottom,
                right,
                0 as core::ffi::c_int,
                (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize],
            );
            plane += 1;
        }
        return;
    }
    let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane_0 < num_planes {
        let is_uv_0: core::ffi::c_int = (plane_0 > 0 as core::ffi::c_int)
            as core::ffi::c_int;
        let top_0: core::ffi::c_int = ext_size
            >> (if is_uv_0 != 0 { ss_y } else { 0 as core::ffi::c_int });
        let left_0: core::ffi::c_int = ext_size
            >> (if is_uv_0 != 0 { ss_x } else { 0 as core::ffi::c_int });
        let bottom_0: core::ffi::c_int = top_0
            + (*ybf).c2rust_unnamed_0.heights[is_uv_0 as usize]
            - (*ybf).c2rust_unnamed_2.crop_heights[is_uv_0 as usize];
        let right_0: core::ffi::c_int = left_0
            + (*ybf).c2rust_unnamed.widths[is_uv_0 as usize]
            - (*ybf).c2rust_unnamed_1.crop_widths[is_uv_0 as usize];
        extend_plane(
            (*ybf).c2rust_unnamed_4.buffers[plane_0 as usize],
            (*ybf).c2rust_unnamed_3.strides[is_uv_0 as usize],
            (*ybf).c2rust_unnamed_1.crop_widths[is_uv_0 as usize],
            (*ybf).c2rust_unnamed_2.crop_heights[is_uv_0 as usize],
            top_0,
            left_0,
            bottom_0,
            right_0,
            0 as core::ffi::c_int,
            (*ybf).c2rust_unnamed_2.crop_heights[is_uv_0 as usize],
        );
        plane_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_extend_frame_borders_c(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    num_planes: core::ffi::c_int,
) {
    extend_frame(ybf, (*ybf).border, num_planes);
}
unsafe extern "C-unwind" fn memcpy_short_addr(
    mut dst8: *mut uint8_t,
    mut src8: *const uint8_t,
    mut num: core::ffi::c_int,
) {
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    memcpy(
        dst as *mut core::ffi::c_void,
        src as *const core::ffi::c_void,
        (num as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_copy_frame_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    num_planes: core::ffi::c_int,
) {
    if (*src_bc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane < num_planes {
            let mut plane_src: *const uint8_t = (*src_bc)
                .c2rust_unnamed_4
                .buffers[plane as usize];
            let mut plane_dst: *mut uint8_t = (*dst_bc)
                .c2rust_unnamed_4
                .buffers[plane as usize];
            let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int)
                as core::ffi::c_int;
            let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
            while row < (*src_bc).c2rust_unnamed_0.heights[is_uv as usize] {
                memcpy_short_addr(
                    plane_dst,
                    plane_src,
                    (*src_bc).c2rust_unnamed.widths[is_uv as usize],
                );
                plane_src = plane_src
                    .offset((*src_bc).c2rust_unnamed_3.strides[is_uv as usize] as isize);
                plane_dst = plane_dst
                    .offset((*dst_bc).c2rust_unnamed_3.strides[is_uv as usize] as isize);
                row += 1;
            }
            plane += 1;
        }
        aom_extend_frame_borders_c(dst_bc, num_planes);
        return;
    }
    let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane_0 < num_planes {
        let mut plane_src_0: *const uint8_t = (*src_bc)
            .c2rust_unnamed_4
            .buffers[plane_0 as usize];
        let mut plane_dst_0: *mut uint8_t = (*dst_bc)
            .c2rust_unnamed_4
            .buffers[plane_0 as usize];
        let is_uv_0: core::ffi::c_int = (plane_0 > 0 as core::ffi::c_int)
            as core::ffi::c_int;
        let mut row_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while row_0 < (*src_bc).c2rust_unnamed_0.heights[is_uv_0 as usize] {
            memcpy(
                plane_dst_0 as *mut core::ffi::c_void,
                plane_src_0 as *const core::ffi::c_void,
                (*src_bc).c2rust_unnamed.widths[is_uv_0 as usize] as size_t,
            );
            plane_src_0 = plane_src_0
                .offset((*src_bc).c2rust_unnamed_3.strides[is_uv_0 as usize] as isize);
            plane_dst_0 = plane_dst_0
                .offset((*dst_bc).c2rust_unnamed_3.strides[is_uv_0 as usize] as isize);
            row_0 += 1;
        }
        plane_0 += 1;
    }
    aom_extend_frame_borders_c(dst_bc, num_planes);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_copy_y_c(
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
    mut use_crop: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0;
    let mut width: core::ffi::c_int = if use_crop != 0 {
        (*src_ybc).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
    } else {
        (*src_ybc).c2rust_unnamed.c2rust_unnamed.y_width
    };
    let mut height: core::ffi::c_int = if use_crop != 0 {
        (*src_ybc).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
    } else {
        (*src_ybc).c2rust_unnamed_0.c2rust_unnamed.y_height
    };
    let mut src: *const uint8_t = (*src_ybc).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
    let mut dst: *mut uint8_t = (*dst_ybc).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
    if (*src_ybc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut src16: *const uint16_t = ((src as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut dst16: *mut uint16_t = ((dst as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        row = 0 as core::ffi::c_int;
        while row < height {
            memcpy(
                dst16 as *mut core::ffi::c_void,
                src16 as *const core::ffi::c_void,
                (width as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src16 = src16
                .offset((*src_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
            dst16 = dst16
                .offset((*dst_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
            row += 1;
        }
        return;
    }
    row = 0 as core::ffi::c_int;
    while row < height {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            width as size_t,
        );
        src = src.offset((*src_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
        dst = dst.offset((*dst_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_copy_u_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    mut use_crop: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0;
    let mut width: core::ffi::c_int = if use_crop != 0 {
        (*src_bc).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
    } else {
        (*src_bc).c2rust_unnamed.c2rust_unnamed.uv_width
    };
    let mut height: core::ffi::c_int = if use_crop != 0 {
        (*src_bc).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
    } else {
        (*src_bc).c2rust_unnamed_0.c2rust_unnamed.uv_height
    };
    let mut src: *const uint8_t = (*src_bc).c2rust_unnamed_4.c2rust_unnamed.u_buffer;
    let mut dst: *mut uint8_t = (*dst_bc).c2rust_unnamed_4.c2rust_unnamed.u_buffer;
    if (*src_bc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut src16: *const uint16_t = ((src as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut dst16: *mut uint16_t = ((dst as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        row = 0 as core::ffi::c_int;
        while row < height {
            memcpy(
                dst16 as *mut core::ffi::c_void,
                src16 as *const core::ffi::c_void,
                (width as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src16 = src16
                .offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            dst16 = dst16
                .offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            row += 1;
        }
        return;
    }
    row = 0 as core::ffi::c_int;
    while row < height {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            width as size_t,
        );
        src = src.offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        dst = dst.offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_copy_v_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    mut use_crop: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0;
    let mut width: core::ffi::c_int = if use_crop != 0 {
        (*src_bc).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
    } else {
        (*src_bc).c2rust_unnamed.c2rust_unnamed.uv_width
    };
    let mut height: core::ffi::c_int = if use_crop != 0 {
        (*src_bc).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
    } else {
        (*src_bc).c2rust_unnamed_0.c2rust_unnamed.uv_height
    };
    let mut src: *const uint8_t = (*src_bc).c2rust_unnamed_4.c2rust_unnamed.v_buffer;
    let mut dst: *mut uint8_t = (*dst_bc).c2rust_unnamed_4.c2rust_unnamed.v_buffer;
    if (*src_bc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut src16: *const uint16_t = ((src as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut dst16: *mut uint16_t = ((dst as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        row = 0 as core::ffi::c_int;
        while row < height {
            memcpy(
                dst16 as *mut core::ffi::c_void,
                src16 as *const core::ffi::c_void,
                (width as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src16 = src16
                .offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            dst16 = dst16
                .offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            row += 1;
        }
        return;
    }
    row = 0 as core::ffi::c_int;
    while row < height {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            width as size_t,
        );
        src = src.offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        dst = dst.offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_partial_copy_y_c(
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut hstart1: core::ffi::c_int,
    mut hend1: core::ffi::c_int,
    mut vstart1: core::ffi::c_int,
    mut vend1: core::ffi::c_int,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
    mut hstart2: core::ffi::c_int,
    mut vstart2: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0;
    let mut src: *const uint8_t = (*src_ybc).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
    let mut dst: *mut uint8_t = (*dst_ybc).c2rust_unnamed_4.c2rust_unnamed.y_buffer;
    if (*src_ybc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut src16: *const uint16_t = ((src
            .offset(
                (vstart1 * (*src_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize,
            )
            .offset(hstart1 as isize) as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut dst16: *mut uint16_t = ((dst
            .offset(
                (vstart2 * (*dst_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize,
            )
            .offset(hstart2 as isize) as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        row = vstart1;
        while row < vend1 {
            memcpy(
                dst16 as *mut core::ffi::c_void,
                src16 as *const core::ffi::c_void,
                ((hend1 - hstart1) as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src16 = src16
                .offset((*src_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
            dst16 = dst16
                .offset((*dst_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
            row += 1;
        }
        return;
    }
    src = src
        .offset((vstart1 * (*src_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
        .offset(hstart1 as isize);
    dst = dst
        .offset((vstart2 * (*dst_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
        .offset(hstart2 as isize);
    row = vstart1;
    while row < vend1 {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (hend1 - hstart1) as size_t,
        );
        src = src.offset((*src_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
        dst = dst.offset((*dst_ybc).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_partial_coloc_copy_y_c(
    mut src_ybc: *const YV12_BUFFER_CONFIG,
    mut dst_ybc: *mut YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut hend: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut vend: core::ffi::c_int,
) {
    aom_yv12_partial_copy_y_c(
        src_ybc,
        hstart,
        hend,
        vstart,
        vend,
        dst_ybc,
        hstart,
        vstart,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_partial_copy_u_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut hstart1: core::ffi::c_int,
    mut hend1: core::ffi::c_int,
    mut vstart1: core::ffi::c_int,
    mut vend1: core::ffi::c_int,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    mut hstart2: core::ffi::c_int,
    mut vstart2: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0;
    let mut src: *const uint8_t = (*src_bc).c2rust_unnamed_4.c2rust_unnamed.u_buffer;
    let mut dst: *mut uint8_t = (*dst_bc).c2rust_unnamed_4.c2rust_unnamed.u_buffer;
    if (*src_bc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut src16: *const uint16_t = ((src
            .offset(
                (vstart1 * (*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize,
            )
            .offset(hstart1 as isize) as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut dst16: *mut uint16_t = ((dst
            .offset(
                (vstart2 * (*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize,
            )
            .offset(hstart2 as isize) as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        row = vstart1;
        while row < vend1 {
            memcpy(
                dst16 as *mut core::ffi::c_void,
                src16 as *const core::ffi::c_void,
                ((hend1 - hstart1) as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src16 = src16
                .offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            dst16 = dst16
                .offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            row += 1;
        }
        return;
    }
    src = src
        .offset((vstart1 * (*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
        .offset(hstart1 as isize);
    dst = dst
        .offset((vstart2 * (*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
        .offset(hstart2 as isize);
    row = vstart1;
    while row < vend1 {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (hend1 - hstart1) as size_t,
        );
        src = src.offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        dst = dst.offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_partial_coloc_copy_u_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut hend: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut vend: core::ffi::c_int,
) {
    aom_yv12_partial_copy_u_c(
        src_bc,
        hstart,
        hend,
        vstart,
        vend,
        dst_bc,
        hstart,
        vstart,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_partial_copy_v_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut hstart1: core::ffi::c_int,
    mut hend1: core::ffi::c_int,
    mut vstart1: core::ffi::c_int,
    mut vend1: core::ffi::c_int,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    mut hstart2: core::ffi::c_int,
    mut vstart2: core::ffi::c_int,
) {
    let mut row: core::ffi::c_int = 0;
    let mut src: *const uint8_t = (*src_bc).c2rust_unnamed_4.c2rust_unnamed.v_buffer;
    let mut dst: *mut uint8_t = (*dst_bc).c2rust_unnamed_4.c2rust_unnamed.v_buffer;
    if (*src_bc).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut src16: *const uint16_t = ((src
            .offset(
                (vstart1 * (*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize,
            )
            .offset(hstart1 as isize) as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut dst16: *mut uint16_t = ((dst
            .offset(
                (vstart2 * (*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize,
            )
            .offset(hstart2 as isize) as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        row = vstart1;
        while row < vend1 {
            memcpy(
                dst16 as *mut core::ffi::c_void,
                src16 as *const core::ffi::c_void,
                ((hend1 - hstart1) as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src16 = src16
                .offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            dst16 = dst16
                .offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
            row += 1;
        }
        return;
    }
    src = src
        .offset((vstart1 * (*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
        .offset(hstart1 as isize);
    dst = dst
        .offset((vstart2 * (*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
        .offset(hstart2 as isize);
    row = vstart1;
    while row < vend1 {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (hend1 - hstart1) as size_t,
        );
        src = src.offset((*src_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        dst = dst.offset((*dst_bc).c2rust_unnamed_3.c2rust_unnamed.uv_stride as isize);
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_partial_coloc_copy_v_c(
    mut src_bc: *const YV12_BUFFER_CONFIG,
    mut dst_bc: *mut YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut hend: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut vend: core::ffi::c_int,
) {
    aom_yv12_partial_copy_v_c(
        src_bc,
        hstart,
        hend,
        vstart,
        vend,
        dst_bc,
        hstart,
        vstart,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_yv12_realloc_with_new_border_c(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut new_border: core::ffi::c_int,
    mut byte_alignment: core::ffi::c_int,
    mut alloc_pyramid: bool,
    mut num_planes: core::ffi::c_int,
) -> core::ffi::c_int {
    if !ybf.is_null() {
        if new_border == (*ybf).border {
            return 0 as core::ffi::c_int;
        }
        let mut new_buf: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
            &raw mut new_buf as *mut YV12_BUFFER_CONFIG as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<YV12_BUFFER_CONFIG>() as size_t,
        );
        let error: core::ffi::c_int = aom_alloc_frame_buffer(
            &raw mut new_buf,
            (*ybf).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
            (*ybf).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
            (*ybf).subsampling_x,
            (*ybf).subsampling_y,
            (*ybf).flags & YV12_FLAG_HIGHBITDEPTH,
            new_border,
            byte_alignment,
            alloc_pyramid,
            0 as core::ffi::c_int,
        ) as core::ffi::c_int;
        if error != 0 {
            return error;
        }
        aom_yv12_copy_frame_c(ybf, &raw mut new_buf, num_planes);
        aom_free_frame_buffer(ybf);
        *ybf = new_buf;
        return 0 as core::ffi::c_int;
    }
    return -(2 as core::ffi::c_int);
}

extern "C" {
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
