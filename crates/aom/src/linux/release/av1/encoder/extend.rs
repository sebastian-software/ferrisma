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

// Mechanically translated from upstream/aom/av1/encoder/extend.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type image_pyramid;
    pub type corner_list;
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
unsafe extern "C-unwind" fn copy_and_extend_plane(
    mut src: *const uint8_t,
    mut src_pitch: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_pitch: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut extend_top: core::ffi::c_int,
    mut extend_left: core::ffi::c_int,
    mut extend_bottom: core::ffi::c_int,
    mut extend_right: core::ffi::c_int,
    mut chroma_step: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut linesize: core::ffi::c_int = 0;
    let mut src_ptr1: *const uint8_t = src;
    let mut src_ptr2: *const uint8_t = src
        .offset(((w - 1 as core::ffi::c_int) * chroma_step) as isize);
    let mut dst_ptr1: *mut uint8_t = dst.offset(-(extend_left as isize));
    let mut dst_ptr2: *mut uint8_t = dst.offset(w as isize);
    i = 0 as core::ffi::c_int;
    while i < h {
        memset(
            dst_ptr1 as *mut core::ffi::c_void,
            *src_ptr1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_left as size_t,
        );
        if chroma_step == 1 as core::ffi::c_int {
            memcpy(
                dst_ptr1.offset(extend_left as isize) as *mut core::ffi::c_void,
                src_ptr1 as *const core::ffi::c_void,
                w as size_t,
            );
        } else {
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < w {
                *dst_ptr1.offset((extend_left + j) as isize) = *src_ptr1
                    .offset((chroma_step * j) as isize);
                j += 1;
            }
        }
        memset(
            dst_ptr2 as *mut core::ffi::c_void,
            *src_ptr2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_right as size_t,
        );
        src_ptr1 = src_ptr1.offset(src_pitch as isize);
        src_ptr2 = src_ptr2.offset(src_pitch as isize);
        dst_ptr1 = dst_ptr1.offset(dst_pitch as isize);
        dst_ptr2 = dst_ptr2.offset(dst_pitch as isize);
        i += 1;
    }
    src_ptr1 = dst.offset(-(extend_left as isize));
    src_ptr2 = dst
        .offset((dst_pitch * (h - 1 as core::ffi::c_int)) as isize)
        .offset(-(extend_left as isize));
    dst_ptr1 = dst
        .offset((dst_pitch * -extend_top) as isize)
        .offset(-(extend_left as isize));
    dst_ptr2 = dst.offset((dst_pitch * h) as isize).offset(-(extend_left as isize));
    linesize = extend_left + extend_right + w;
    i = 0 as core::ffi::c_int;
    while i < extend_top {
        memcpy(
            dst_ptr1 as *mut core::ffi::c_void,
            src_ptr1 as *const core::ffi::c_void,
            linesize as size_t,
        );
        dst_ptr1 = dst_ptr1.offset(dst_pitch as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < extend_bottom {
        memcpy(
            dst_ptr2 as *mut core::ffi::c_void,
            src_ptr2 as *const core::ffi::c_void,
            linesize as size_t,
        );
        dst_ptr2 = dst_ptr2.offset(dst_pitch as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn highbd_copy_and_extend_plane(
    mut src8: *const uint8_t,
    mut src_pitch: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_pitch: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut extend_top: core::ffi::c_int,
    mut extend_left: core::ffi::c_int,
    mut extend_bottom: core::ffi::c_int,
    mut extend_right: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut linesize: core::ffi::c_int = 0;
    let mut src: *mut uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut src_ptr1: *const uint16_t = src;
    let mut src_ptr2: *const uint16_t = src
        .offset(w as isize)
        .offset(-(1 as core::ffi::c_int as isize));
    let mut dst_ptr1: *mut uint16_t = dst.offset(-(extend_left as isize));
    let mut dst_ptr2: *mut uint16_t = dst.offset(w as isize);
    i = 0 as core::ffi::c_int;
    while i < h {
        aom_memset16(
            dst_ptr1 as *mut core::ffi::c_void,
            *src_ptr1.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_left as size_t,
        );
        memcpy(
            dst_ptr1.offset(extend_left as isize) as *mut core::ffi::c_void,
            src_ptr1 as *const core::ffi::c_void,
            (w as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        aom_memset16(
            dst_ptr2 as *mut core::ffi::c_void,
            *src_ptr2.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            extend_right as size_t,
        );
        src_ptr1 = src_ptr1.offset(src_pitch as isize);
        src_ptr2 = src_ptr2.offset(src_pitch as isize);
        dst_ptr1 = dst_ptr1.offset(dst_pitch as isize);
        dst_ptr2 = dst_ptr2.offset(dst_pitch as isize);
        i += 1;
    }
    src_ptr1 = dst.offset(-(extend_left as isize));
    src_ptr2 = dst
        .offset((dst_pitch * (h - 1 as core::ffi::c_int)) as isize)
        .offset(-(extend_left as isize));
    dst_ptr1 = dst
        .offset((dst_pitch * -extend_top) as isize)
        .offset(-(extend_left as isize));
    dst_ptr2 = dst.offset((dst_pitch * h) as isize).offset(-(extend_left as isize));
    linesize = extend_left + extend_right + w;
    i = 0 as core::ffi::c_int;
    while i < extend_top {
        memcpy(
            dst_ptr1 as *mut core::ffi::c_void,
            src_ptr1 as *const core::ffi::c_void,
            (linesize as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        dst_ptr1 = dst_ptr1.offset(dst_pitch as isize);
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < extend_bottom {
        memcpy(
            dst_ptr2 as *mut core::ffi::c_void,
            src_ptr2 as *const core::ffi::c_void,
            (linesize as size_t)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        dst_ptr2 = dst_ptr2.offset(dst_pitch as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_copy_and_extend_frame(
    mut src: *const YV12_BUFFER_CONFIG,
    mut dst: *mut YV12_BUFFER_CONFIG,
) {
    let et_y: core::ffi::c_int = (*dst).border;
    let el_y: core::ffi::c_int = (*dst).border;
    let er_y: core::ffi::c_int = (if (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        + (*dst).border
        > (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
            + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
            & !(((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
    {
        (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width + (*dst).border
    } else {
        (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
            + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
            & !(((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
    }) - (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width;
    let eb_y: core::ffi::c_int = (if (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        + (*dst).border
        > (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
            + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
            & !(((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
    {
        (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height + (*dst).border
    } else {
        (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
            + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
            & !(((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                - 1 as core::ffi::c_int)
    }) - (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height;
    let uv_width_subsampling: core::ffi::c_int = (*src).subsampling_x;
    let uv_height_subsampling: core::ffi::c_int = (*src).subsampling_y;
    let et_uv: core::ffi::c_int = et_y >> uv_height_subsampling;
    let el_uv: core::ffi::c_int = el_y >> uv_width_subsampling;
    let eb_uv: core::ffi::c_int = eb_y >> uv_height_subsampling;
    let er_uv: core::ffi::c_int = er_y >> uv_width_subsampling;
    if (*src).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        highbd_copy_and_extend_plane(
            (*src).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
            (*src).c2rust_unnamed_3.c2rust_unnamed.y_stride,
            (*dst).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
            (*dst).c2rust_unnamed_3.c2rust_unnamed.y_stride,
            (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
            (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
            et_y,
            el_y,
            eb_y,
            er_y,
        );
        if (*src).monochrome == 0 {
            highbd_copy_and_extend_plane(
                (*src).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
                (*src).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
                (*dst).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
                (*dst).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
                (*src).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
                (*src).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
                et_uv,
                el_uv,
                eb_uv,
                er_uv,
            );
            highbd_copy_and_extend_plane(
                (*src).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
                (*src).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
                (*dst).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
                (*dst).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
                (*src).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
                (*src).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
                et_uv,
                el_uv,
                eb_uv,
                er_uv,
            );
        }
        return;
    }
    copy_and_extend_plane(
        (*src).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
        (*src).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*dst).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
        (*dst).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*src).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
        (*src).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
        et_y,
        el_y,
        eb_y,
        er_y,
        1 as core::ffi::c_int,
    );
    if (*src).monochrome == 0 {
        let chroma_step: core::ffi::c_int = if !((*src)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer)
            .is_null()
        {
            1 as core::ffi::c_int
        } else {
            2 as core::ffi::c_int
        };
        let mut src_v_buffer: *const uint8_t = if !((*src)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer)
            .is_null()
        {
            (*src).c2rust_unnamed_4.c2rust_unnamed.v_buffer
        } else {
            ((*src).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
                .offset(1 as core::ffi::c_int as isize)
        };
        copy_and_extend_plane(
            (*src).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
            (*src).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
            (*dst).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
            (*dst).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
            (*src).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
            (*src).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
            et_uv,
            el_uv,
            eb_uv,
            er_uv,
            chroma_step,
        );
        copy_and_extend_plane(
            src_v_buffer,
            (*src).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
            (*dst).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
            (*dst).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
            (*src).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
            (*src).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
            et_uv,
            el_uv,
            eb_uv,
            er_uv,
            chroma_step,
        );
    }
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
