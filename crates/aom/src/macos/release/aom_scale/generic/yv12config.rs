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

// Mechanically translated from upstream/aom/aom_scale/generic/yv12config.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_img_metadata_alloc(
        type_0: uint32_t,
        data: *const uint8_t,
        sz: size_t,
        insert_flag: aom_metadata_insert_flags_t,
    ) -> *mut aom_metadata_t;
    fn aom_img_metadata_array_alloc(sz: size_t) -> *mut aom_metadata_array_t;
    fn aom_img_metadata_array_free(arr: *mut aom_metadata_array_t);
    fn aom_get_pyramid_alloc_size(
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        image_is_16bit: bool,
    ) -> size_t;
    fn aom_alloc_pyramid(
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        image_is_16bit: bool,
    ) -> *mut ImagePyramid;
    fn aom_free_pyramid(pyr: *mut ImagePyramid);
    fn av1_get_corner_list_size() -> size_t;
    fn av1_alloc_corner_list() -> *mut CornerList;
    fn av1_free_corner_list(corners: *mut CornerList);
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AOM_CODEC_LIST_END: C2RustUnnamed = 9;
pub const AOM_CODEC_INVALID_PARAM: C2RustUnnamed = 8;
pub const AOM_CODEC_CORRUPT_FRAME: C2RustUnnamed = 7;
pub const AOM_CODEC_UNSUP_FEATURE: C2RustUnnamed = 6;
pub const AOM_CODEC_UNSUP_BITSTREAM: C2RustUnnamed = 5;
pub const AOM_CODEC_INCAPABLE: C2RustUnnamed = 4;
pub const AOM_CODEC_ABI_MISMATCH: C2RustUnnamed = 3;
pub const AOM_CODEC_MEM_ERROR: C2RustUnnamed = 2;
pub const AOM_CODEC_ERROR: C2RustUnnamed = 1;
pub const AOM_CODEC_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_codec_frame_buffer_t = aom_codec_frame_buffer;
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
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub c2rust_unnamed_0: C2RustUnnamed_8,
    pub c2rust_unnamed_1: C2RustUnnamed_6,
    pub c2rust_unnamed_2: C2RustUnnamed_4,
    pub c2rust_unnamed_3: C2RustUnnamed_2,
    pub c2rust_unnamed_4: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub c2rust_unnamed: C2RustUnnamed_7,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c2rust_unnamed: C2RustUnnamed_11,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type CornerList = corner_list;
pub type ImagePyramid = image_pyramid;
pub const REF_FRAMES: C2RustUnnamed_12 = 8;
pub type C2RustUnnamed_12 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_12 = 7;
pub const BWD_REFS: C2RustUnnamed_12 = 3;
pub const FWD_REFS: C2RustUnnamed_12 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_12 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_12 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_12 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_12 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_12 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_12 = 4;
pub const LAST3_FRAME: C2RustUnnamed_12 = 3;
pub const LAST2_FRAME: C2RustUnnamed_12 = 2;
pub const LAST_FRAME: C2RustUnnamed_12 = 1;
pub const INTRA_FRAME: C2RustUnnamed_12 = 0;
pub const NONE_FRAME: C2RustUnnamed_12 = -1;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn aom_calc_y_stride(
    mut aligned_width: core::ffi::c_int,
    mut border: core::ffi::c_int,
) -> core::ffi::c_int {
    return aligned_width + 2 as core::ffi::c_int * border + 31 as core::ffi::c_int
        & !(31 as core::ffi::c_int);
}
pub const AOM_MAX_ALLOCABLE_MEMORY: core::ffi::c_long = 8589934592 as core::ffi::c_long;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_free_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
) -> core::ffi::c_int {
    if !ybf.is_null() {
        if (*ybf).buffer_alloc_sz > 0 as size_t {
            aom_free((*ybf).buffer_alloc as *mut core::ffi::c_void);
        }
        if !((*ybf).y_pyramid).is_null() {
            aom_free_pyramid((*ybf).y_pyramid as *mut ImagePyramid);
        }
        if !((*ybf).corners).is_null() {
            av1_free_corner_list((*ybf).corners as *mut CornerList);
        }
        aom_remove_metadata_from_frame_buffer(ybf);
        memset(
            ybf as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<YV12_BUFFER_CONFIG>() as size_t,
        );
        return 0 as core::ffi::c_int;
    }
    return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
}
unsafe extern "C-unwind" fn realloc_frame_buffer_aligned(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
    mut use_highbitdepth: core::ffi::c_int,
    mut border: core::ffi::c_int,
    mut byte_alignment: core::ffi::c_int,
    mut fb: *mut aom_codec_frame_buffer_t,
    mut cb: aom_get_frame_buffer_cb_fn_t,
    mut cb_priv: *mut core::ffi::c_void,
    y_stride: core::ffi::c_int,
    yplane_size: uint64_t,
    uvplane_size: uint64_t,
    aligned_width: core::ffi::c_int,
    aligned_height: core::ffi::c_int,
    uv_width: core::ffi::c_int,
    uv_height: core::ffi::c_int,
    uv_stride: core::ffi::c_int,
    uv_border_w: core::ffi::c_int,
    uv_border_h: core::ffi::c_int,
    mut alloc_pyramid: bool,
    mut alloc_y_plane_only: core::ffi::c_int,
) -> core::ffi::c_int {
    if !ybf.is_null() {
        let aom_byte_align: core::ffi::c_int = if byte_alignment == 0 as core::ffi::c_int
        {
            1 as core::ffi::c_int
        } else {
            byte_alignment
        };
        let frame_size: uint64_t = ((1 as core::ffi::c_int + use_highbitdepth)
            as uint64_t)
            .wrapping_mul(
                yplane_size.wrapping_add((2 as uint64_t).wrapping_mul(uvplane_size)),
            );
        let mut buf: *mut uint8_t = 0 as *mut uint8_t;
        let mut alloc_size: uint64_t = frame_size;
        if alloc_pyramid {
            alloc_size = alloc_size
                .wrapping_add(
                    aom_get_pyramid_alloc_size(width, height, use_highbitdepth != 0)
                        as uint64_t,
                );
            alloc_size = alloc_size.wrapping_add(av1_get_corner_list_size() as uint64_t);
        }
        if alloc_size
            > (AOM_MAX_ALLOCABLE_MEMORY
                / REF_FRAMES as core::ffi::c_int as core::ffi::c_long) as uint64_t
        {
            return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
        }
        if cb.is_some() {
            let align_addr_extra_size: core::ffi::c_int = 31 as core::ffi::c_int;
            let external_frame_size: uint64_t = frame_size
                .wrapping_add(align_addr_extra_size as uint64_t);
            if external_frame_size != external_frame_size as size_t as uint64_t {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
            if cb
                .expect(
                    "non-null function pointer",
                )(cb_priv, external_frame_size as size_t, fb) < 0 as core::ffi::c_int
            {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
            if ((*fb).data).is_null() || ((*fb).size as uint64_t) < external_frame_size {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
            (*ybf).buffer_alloc = (((*fb).data as uintptr_t)
                .wrapping_add(
                    (32 as core::ffi::c_int - 1 as core::ffi::c_int) as uintptr_t,
                ) & !((32 as core::ffi::c_int - 1 as core::ffi::c_int) as uintptr_t))
                as *mut core::ffi::c_void as *mut uint8_t;
        } else if frame_size > (*ybf).buffer_alloc_sz as uint64_t {
            aom_free((*ybf).buffer_alloc as *mut core::ffi::c_void);
            (*ybf).buffer_alloc = 0 as *mut uint8_t;
            (*ybf).buffer_alloc_sz = 0 as size_t;
            if frame_size != frame_size as size_t as uint64_t {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
            (*ybf).buffer_alloc = aom_memalign(32 as size_t, frame_size as size_t)
                as *mut uint8_t;
            if ((*ybf).buffer_alloc).is_null() {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
            (*ybf).buffer_alloc_sz = frame_size as size_t;
            memset(
                (*ybf).buffer_alloc as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (*ybf).buffer_alloc_sz,
            );
        }
        (*ybf).c2rust_unnamed_1.c2rust_unnamed.y_crop_width = width;
        (*ybf).c2rust_unnamed_2.c2rust_unnamed.y_crop_height = height;
        (*ybf).c2rust_unnamed.c2rust_unnamed.y_width = aligned_width;
        (*ybf).c2rust_unnamed_0.c2rust_unnamed.y_height = aligned_height;
        (*ybf).c2rust_unnamed_3.c2rust_unnamed.y_stride = y_stride;
        (*ybf).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width = width + ss_x >> ss_x;
        (*ybf).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height = height + ss_y >> ss_y;
        (*ybf).c2rust_unnamed.c2rust_unnamed.uv_width = uv_width;
        (*ybf).c2rust_unnamed_0.c2rust_unnamed.uv_height = uv_height;
        (*ybf).c2rust_unnamed_3.c2rust_unnamed.uv_stride = uv_stride;
        (*ybf).border = border;
        (*ybf).frame_size = frame_size as size_t;
        (*ybf).subsampling_x = ss_x;
        (*ybf).subsampling_y = ss_y;
        buf = (*ybf).buffer_alloc;
        if use_highbitdepth != 0 {
            buf = ((*ybf).buffer_alloc as uintptr_t >> 1 as core::ffi::c_int)
                as *mut uint8_t;
            (*ybf).flags = YV12_FLAG_HIGHBITDEPTH;
        } else {
            (*ybf).flags = 0 as core::ffi::c_int;
        }
        (*ybf).c2rust_unnamed_4.c2rust_unnamed.y_buffer = ((buf
            .offset((border * y_stride) as isize)
            .offset(border as isize) as uintptr_t)
            .wrapping_add((aom_byte_align - 1 as core::ffi::c_int) as uintptr_t)
            & !((aom_byte_align - 1 as core::ffi::c_int) as uintptr_t))
            as *mut core::ffi::c_void as *mut uint8_t;
        if alloc_y_plane_only == 0 {
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.u_buffer = ((buf
                .offset(yplane_size as isize)
                .offset((uv_border_h * uv_stride) as isize)
                .offset(uv_border_w as isize) as uintptr_t)
                .wrapping_add((aom_byte_align - 1 as core::ffi::c_int) as uintptr_t)
                & !((aom_byte_align - 1 as core::ffi::c_int) as uintptr_t))
                as *mut core::ffi::c_void as *mut uint8_t;
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.v_buffer = ((buf
                .offset(yplane_size as isize)
                .offset(uvplane_size as isize)
                .offset((uv_border_h * uv_stride) as isize)
                .offset(uv_border_w as isize) as uintptr_t)
                .wrapping_add((aom_byte_align - 1 as core::ffi::c_int) as uintptr_t)
                & !((aom_byte_align - 1 as core::ffi::c_int) as uintptr_t))
                as *mut core::ffi::c_void as *mut uint8_t;
        } else {
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.u_buffer = 0 as *mut uint8_t;
            (*ybf).c2rust_unnamed_4.c2rust_unnamed.v_buffer = 0 as *mut uint8_t;
        }
        (*ybf).use_external_reference_buffers = 0 as core::ffi::c_int;
        if !((*ybf).y_pyramid).is_null() {
            aom_free_pyramid((*ybf).y_pyramid as *mut ImagePyramid);
            (*ybf).y_pyramid = 0 as *mut image_pyramid;
        }
        if !((*ybf).corners).is_null() {
            av1_free_corner_list((*ybf).corners as *mut CornerList);
            (*ybf).corners = 0 as *mut corner_list;
        }
        if alloc_pyramid {
            (*ybf).y_pyramid = aom_alloc_pyramid(width, height, use_highbitdepth != 0)
                as *mut image_pyramid;
            if ((*ybf).y_pyramid).is_null() {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
            (*ybf).corners = av1_alloc_corner_list() as *mut corner_list;
            if ((*ybf).corners).is_null() {
                return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
            }
        }
        (*ybf).corrupted = 0 as core::ffi::c_int;
        return 0 as core::ffi::c_int;
    }
    return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
}
unsafe extern "C-unwind" fn calc_stride_and_planesize(
    ss_x: core::ffi::c_int,
    ss_y: core::ffi::c_int,
    aligned_width: core::ffi::c_int,
    aligned_height: core::ffi::c_int,
    border: core::ffi::c_int,
    byte_alignment: core::ffi::c_int,
    mut alloc_y_plane_only: core::ffi::c_int,
    mut y_stride: *mut core::ffi::c_int,
    mut uv_stride: *mut core::ffi::c_int,
    mut yplane_size: *mut uint64_t,
    mut uvplane_size: *mut uint64_t,
    uv_height: core::ffi::c_int,
) -> core::ffi::c_int {
    if border & 0x1f as core::ffi::c_int != 0 {
        return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
    }
    *y_stride = aom_calc_y_stride(aligned_width, border);
    *yplane_size = ((aligned_height + 2 as core::ffi::c_int * border) as uint64_t)
        .wrapping_mul(*y_stride as uint64_t)
        .wrapping_add(byte_alignment as uint64_t);
    if alloc_y_plane_only == 0 {
        *uv_stride = *y_stride >> ss_x;
        *uvplane_size = ((uv_height + 2 as core::ffi::c_int * (border >> ss_y))
            as uint64_t)
            .wrapping_mul(*uv_stride as uint64_t)
            .wrapping_add(byte_alignment as uint64_t);
    } else {
        *uv_stride = 0 as core::ffi::c_int;
        *uvplane_size = 0 as uint64_t;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_realloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
    mut use_highbitdepth: core::ffi::c_int,
    mut border: core::ffi::c_int,
    mut byte_alignment: core::ffi::c_int,
    mut fb: *mut aom_codec_frame_buffer_t,
    mut cb: aom_get_frame_buffer_cb_fn_t,
    mut cb_priv: *mut core::ffi::c_void,
    mut alloc_pyramid: bool,
    mut alloc_y_plane_only: core::ffi::c_int,
) -> core::ffi::c_int {
    if !ybf.is_null() {
        let mut y_stride: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut uv_stride: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut yplane_size: uint64_t = 0 as uint64_t;
        let mut uvplane_size: uint64_t = 0 as uint64_t;
        let aligned_width: core::ffi::c_int = width + 7 as core::ffi::c_int
            & !(7 as core::ffi::c_int);
        let aligned_height: core::ffi::c_int = height + 7 as core::ffi::c_int
            & !(7 as core::ffi::c_int);
        let uv_width: core::ffi::c_int = aligned_width >> ss_x;
        let uv_height: core::ffi::c_int = aligned_height >> ss_y;
        let uv_border_w: core::ffi::c_int = border >> ss_x;
        let uv_border_h: core::ffi::c_int = border >> ss_y;
        let mut error: core::ffi::c_int = calc_stride_and_planesize(
            ss_x,
            ss_y,
            aligned_width,
            aligned_height,
            border,
            byte_alignment,
            alloc_y_plane_only,
            &raw mut y_stride,
            &raw mut uv_stride,
            &raw mut yplane_size,
            &raw mut uvplane_size,
            uv_height,
        );
        if error != 0 {
            return error;
        }
        return realloc_frame_buffer_aligned(
            ybf,
            width,
            height,
            ss_x,
            ss_y,
            use_highbitdepth,
            border,
            byte_alignment,
            fb,
            cb,
            cb_priv,
            y_stride,
            yplane_size,
            uvplane_size,
            aligned_width,
            aligned_height,
            uv_width,
            uv_height,
            uv_stride,
            uv_border_w,
            uv_border_h,
            alloc_pyramid,
            alloc_y_plane_only,
        );
    }
    return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_alloc_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
    mut use_highbitdepth: core::ffi::c_int,
    mut border: core::ffi::c_int,
    mut byte_alignment: core::ffi::c_int,
    mut alloc_pyramid: bool,
    mut alloc_y_plane_only: core::ffi::c_int,
) -> core::ffi::c_int {
    if !ybf.is_null() {
        aom_free_frame_buffer(ybf);
        return aom_realloc_frame_buffer(
            ybf,
            width,
            height,
            ss_x,
            ss_y,
            use_highbitdepth,
            border,
            byte_alignment,
            0 as *mut aom_codec_frame_buffer_t,
            None,
            NULL,
            alloc_pyramid,
            alloc_y_plane_only,
        );
    }
    return AOM_CODEC_MEM_ERROR as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_remove_metadata_from_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
) {
    if !ybf.is_null() && !((*ybf).metadata).is_null() {
        aom_img_metadata_array_free((*ybf).metadata);
        (*ybf).metadata = 0 as *mut aom_metadata_array_t;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_copy_metadata_to_frame_buffer(
    mut ybf: *mut YV12_BUFFER_CONFIG,
    mut arr: *const aom_metadata_array_t,
) -> core::ffi::c_int {
    if ybf.is_null() || arr.is_null() || ((*arr).metadata_array).is_null() {
        return -(1 as core::ffi::c_int);
    }
    if (*ybf).metadata == arr as *mut aom_metadata_array_t {
        return 0 as core::ffi::c_int;
    }
    aom_remove_metadata_from_frame_buffer(ybf);
    (*ybf).metadata = aom_img_metadata_array_alloc((*arr).sz);
    if ((*ybf).metadata).is_null() {
        return -(1 as core::ffi::c_int);
    }
    let mut i: size_t = 0 as size_t;
    while i < (*(*ybf).metadata).sz {
        let fresh0 = &raw mut *((*(*ybf).metadata).metadata_array).offset(i as isize);
        *fresh0 = aom_img_metadata_alloc(
            (**((*arr).metadata_array).offset(i as isize)).type_0,
            (**((*arr).metadata_array).offset(i as isize)).payload,
            (**((*arr).metadata_array).offset(i as isize)).sz,
            (**((*arr).metadata_array).offset(i as isize)).insert_flag,
        );
        if (*((*(*ybf).metadata).metadata_array).offset(i as isize)).is_null() {
            aom_img_metadata_array_free((*ybf).metadata);
            (*ybf).metadata = 0 as *mut aom_metadata_array_t;
            return -(1 as core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }
    (*(*ybf).metadata).sz = (*arr).sz;
    return 0 as core::ffi::c_int;
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
