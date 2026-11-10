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

// Mechanically translated from upstream/aom/aom/src/aom_image.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
pub type aom_img_fmt = core::ffi::c_uint;
pub const AOM_IMG_FMT_I44416: aom_img_fmt = 2310;
pub const AOM_IMG_FMT_I42216: aom_img_fmt = 2309;
pub const AOM_IMG_FMT_YV1216: aom_img_fmt = 2817;
pub const AOM_IMG_FMT_I42016: aom_img_fmt = 2306;
pub const AOM_IMG_FMT_NV12: aom_img_fmt = 263;
pub const AOM_IMG_FMT_I444: aom_img_fmt = 262;
pub const AOM_IMG_FMT_I422: aom_img_fmt = 261;
pub const AOM_IMG_FMT_AOMI420: aom_img_fmt = 260;
pub const AOM_IMG_FMT_AOMYV12: aom_img_fmt = 771;
pub const AOM_IMG_FMT_I420: aom_img_fmt = 258;
pub const AOM_IMG_FMT_YV12: aom_img_fmt = 769;
pub const AOM_IMG_FMT_NONE: aom_img_fmt = 0;
pub type aom_img_fmt_t = aom_img_fmt;
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
pub struct aom_image {
    pub fmt: aom_img_fmt_t,
    pub cp: aom_color_primaries_t,
    pub tc: aom_transfer_characteristics_t,
    pub mc: aom_matrix_coefficients_t,
    pub monochrome: core::ffi::c_int,
    pub csp: aom_chroma_sample_position_t,
    pub range: aom_color_range_t,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub bit_depth: core::ffi::c_uint,
    pub d_w: core::ffi::c_uint,
    pub d_h: core::ffi::c_uint,
    pub r_w: core::ffi::c_uint,
    pub r_h: core::ffi::c_uint,
    pub x_chroma_shift: core::ffi::c_uint,
    pub y_chroma_shift: core::ffi::c_uint,
    pub planes: [*mut core::ffi::c_uchar; 3],
    pub stride: [core::ffi::c_int; 3],
    pub sz: size_t,
    pub bps: core::ffi::c_int,
    pub temporal_id: core::ffi::c_int,
    pub spatial_id: core::ffi::c_int,
    pub user_priv: *mut core::ffi::c_void,
    pub img_data: *mut core::ffi::c_uchar,
    pub img_data_owner: core::ffi::c_int,
    pub self_allocd: core::ffi::c_int,
    pub metadata: *mut aom_metadata_array_t,
    pub fb_priv: *mut core::ffi::c_void,
}
pub type aom_image_t = aom_image;
pub type aom_alloc_img_data_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut core::ffi::c_void, size_t) -> *mut core::ffi::c_void,
>;
pub const OBU_METADATA_TYPE_TIMECODE: C2RustUnnamed = 5;
pub const OBU_METADATA_TYPE_SCALABILITY: C2RustUnnamed = 3;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const OBU_METADATA_TYPE_ITUT_T35: C2RustUnnamed = 4;
pub const OBU_METADATA_TYPE_HDR_MDCV: C2RustUnnamed = 2;
pub const OBU_METADATA_TYPE_HDR_CLL: C2RustUnnamed = 1;
pub const OBU_METADATA_TYPE_AOM_RESERVED_0: C2RustUnnamed = 0;
pub const __ASSERT_FUNCTION: [core::ffi::c_char; 197] = unsafe {
    ::core::mem::transmute::<
        [u8; 197],
        [core::ffi::c_char; 197],
    >(
        *b"aom_image_t *img_alloc_helper(aom_image_t *, aom_img_fmt_t, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned char *, aom_alloc_img_data_cb_fn_t, void *)\0",
    )
};
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_IMG_FMT_PLANAR: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const AOM_IMG_FMT_UV_FLIP: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const AOM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const AOM_PLANE_PACKED: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const AOM_MIF_LAYER_SPECIFIC: core::ffi::c_int = 0x10 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn align_image_dimension(
    mut d: core::ffi::c_uint,
    mut subsampling: core::ffi::c_uint,
    mut size_align: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut align: core::ffi::c_uint = 0;
    align = (((1 as core::ffi::c_int) << subsampling) - 1 as core::ffi::c_int)
        as core::ffi::c_uint;
    align = if size_align.wrapping_sub(1 as core::ffi::c_uint) > align {
        size_align.wrapping_sub(1 as core::ffi::c_uint)
    } else {
        align
    };
    return d.wrapping_add(align) & !align;
}
unsafe extern "C-unwind" fn is_valid_img_fmt(mut fmt: aom_img_fmt_t) -> bool {
    match fmt as core::ffi::c_uint {
        769 | 258 | 261 | 262 | 263 | 2306 | 2817 | 2309 | 2310 => return true_0 != 0,
        _ => return false_0 != 0,
    };
}
unsafe extern "C-unwind" fn img_alloc_helper(
    mut img: *mut aom_image_t,
    mut fmt: aom_img_fmt_t,
    mut d_w: core::ffi::c_uint,
    mut d_h: core::ffi::c_uint,
    mut buf_align: core::ffi::c_uint,
    mut stride_align: core::ffi::c_uint,
    mut size_align: core::ffi::c_uint,
    mut border: core::ffi::c_uint,
    mut img_data: *mut core::ffi::c_uchar,
    mut alloc_cb: aom_alloc_img_data_cb_fn_t,
    mut cb_priv: *mut core::ffi::c_void,
) -> *mut aom_image_t {
    let mut w: core::ffi::c_uint = 0;
    let mut h: core::ffi::c_uint = 0;
    let mut s: uint64_t = 0;
    let mut stride_in_bytes: core::ffi::c_int = 0;
    let mut ret: core::ffi::c_int = 0;
    let mut current_block: u64;
    let mut xcs: core::ffi::c_uint = 0;
    let mut ycs: core::ffi::c_uint = 0;
    let mut bps: core::ffi::c_uint = 0;
    let mut bit_depth: core::ffi::c_uint = 0;
    if !img.is_null() {
        memset(
            img as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_image_t>() as size_t,
        );
    }
    if is_valid_img_fmt(fmt) {
        if !(d_w > 0x8000000 as core::ffi::c_uint || d_h > 0x8000000 as core::ffi::c_uint
            || buf_align > 65536 as core::ffi::c_uint
            || stride_align > 65536 as core::ffi::c_uint
            || size_align > 65536 as core::ffi::c_uint
            || border > 65536 as core::ffi::c_uint)
        {
            if buf_align == 0 {
                buf_align = 1 as core::ffi::c_uint;
            }
            if !(buf_align & buf_align.wrapping_sub(1 as core::ffi::c_uint) != 0) {
                if stride_align == 0 {
                    stride_align = 1 as core::ffi::c_uint;
                }
                if !(stride_align & stride_align.wrapping_sub(1 as core::ffi::c_uint)
                    != 0)
                {
                    if size_align == 0 {
                        size_align = 1 as core::ffi::c_uint;
                    }
                    if !(size_align & size_align.wrapping_sub(1 as core::ffi::c_uint)
                        != 0)
                    {
                        match fmt as core::ffi::c_uint {
                            258 | 769 | 263 => {
                                bps = 12 as core::ffi::c_uint;
                            }
                            261 => {
                                bps = 16 as core::ffi::c_uint;
                            }
                            262 => {
                                bps = 24 as core::ffi::c_uint;
                            }
                            2817 | 2306 => {
                                bps = 24 as core::ffi::c_uint;
                            }
                            2309 => {
                                bps = 32 as core::ffi::c_uint;
                            }
                            2310 => {
                                bps = 48 as core::ffi::c_uint;
                            }
                            _ => {
                                bps = 16 as core::ffi::c_uint;
                            }
                        }
                        bit_depth = (if fmt as core::ffi::c_uint
                            & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0
                        {
                            16 as core::ffi::c_int
                        } else {
                            8 as core::ffi::c_int
                        }) as core::ffi::c_uint;
                        match fmt as core::ffi::c_uint {
                            258 | 769 | 263 | 261 | 2306 | 2817 | 2309 => {
                                xcs = 1 as core::ffi::c_uint;
                            }
                            _ => {
                                xcs = 0 as core::ffi::c_uint;
                            }
                        }
                        match fmt as core::ffi::c_uint {
                            258 | 769 | 263 | 2817 | 2306 => {
                                ycs = 1 as core::ffi::c_uint;
                            }
                            _ => {
                                ycs = 0 as core::ffi::c_uint;
                            }
                        }
                        w = align_image_dimension(d_w, xcs, size_align)
                            as core::ffi::c_uint;
                        if d_w <= w {} else {
                            __assert_fail(
                                b"d_w <= w\0" as *const u8 as *const core::ffi::c_char,
                                b"upstream/aom/aom/src/aom_image.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                129 as core::ffi::c_uint,
                                __ASSERT_FUNCTION.as_ptr(),
                            );
                        };
                        h = align_image_dimension(d_h, ycs, size_align)
                            as core::ffi::c_uint;
                        if d_h <= h {} else {
                            __assert_fail(
                                b"d_h <= h\0" as *const u8 as *const core::ffi::c_char,
                                b"upstream/aom/aom/src/aom_image.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                131 as core::ffi::c_uint,
                                __ASSERT_FUNCTION.as_ptr(),
                            );
                        };
                        s = (w as uint64_t)
                            .wrapping_add(
                                (2 as core::ffi::c_uint).wrapping_mul(border) as uint64_t,
                            );
                        s = if fmt as core::ffi::c_uint
                            & AOM_IMG_FMT_PLANAR as core::ffi::c_uint != 0
                        {
                            s
                        } else {
                            s.wrapping_mul(bps as uint64_t)
                                .wrapping_div(bit_depth as uint64_t)
                        };
                        s = s
                            .wrapping_mul(bit_depth as uint64_t)
                            .wrapping_div(8 as uint64_t);
                        s = s
                            .wrapping_add(stride_align as uint64_t)
                            .wrapping_sub(1 as uint64_t)
                            & !(stride_align as uint64_t).wrapping_sub(1 as uint64_t);
                        if !(s > INT_MAX as uint64_t) {
                            stride_in_bytes = s as core::ffi::c_int;
                            if img.is_null() {
                                img = calloc(
                                    1 as size_t,
                                    ::core::mem::size_of::<aom_image_t>() as size_t,
                                ) as *mut aom_image_t;
                                if img.is_null() {
                                    current_block = 0;
                                } else {
                                    (*img).self_allocd = 1 as core::ffi::c_int;
                                    current_block = 1;
                                }
                            } else {
                                current_block = 1;
                            }
                            match current_block {
                                0 => {}
                                _ => {
                                    (*img).img_data = img_data;
                                    if img_data.is_null() {
                                        let alloc_size: uint64_t = if fmt as core::ffi::c_uint
                                            & AOM_IMG_FMT_PLANAR as core::ffi::c_uint != 0
                                        {
                                            (h
                                                .wrapping_add((2 as core::ffi::c_uint).wrapping_mul(border))
                                                as uint64_t)
                                                .wrapping_mul(stride_in_bytes as uint64_t)
                                                .wrapping_mul(bps as uint64_t)
                                                .wrapping_div(bit_depth as uint64_t)
                                        } else {
                                            (h
                                                .wrapping_add((2 as core::ffi::c_uint).wrapping_mul(border))
                                                as uint64_t)
                                                .wrapping_mul(stride_in_bytes as uint64_t)
                                        };
                                        if alloc_size != alloc_size {
                                            current_block = 0;
                                        } else {
                                            if alloc_cb.is_some() {
                                                let padded_alloc_size: size_t = (alloc_size as size_t)
                                                    .wrapping_add(buf_align as size_t)
                                                    .wrapping_sub(1 as size_t);
                                                (*img).img_data = alloc_cb
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(cb_priv, padded_alloc_size) as *mut uint8_t
                                                    as *mut core::ffi::c_uchar;
                                                if !((*img).img_data).is_null() {
                                                    (*img).img_data = (((*img).img_data as uintptr_t)
                                                        .wrapping_add(
                                                            buf_align.wrapping_sub(1 as core::ffi::c_uint) as uintptr_t,
                                                        )
                                                        & !(buf_align.wrapping_sub(1 as core::ffi::c_uint)
                                                            as uintptr_t)) as *mut core::ffi::c_void as *mut uint8_t
                                                        as *mut core::ffi::c_uchar;
                                                }
                                                (*img).img_data_owner = 0 as core::ffi::c_int;
                                            } else {
                                                (*img).img_data = aom_memalign(
                                                    buf_align as size_t,
                                                    alloc_size as size_t,
                                                ) as *mut uint8_t as *mut core::ffi::c_uchar;
                                                (*img).img_data_owner = 1 as core::ffi::c_int;
                                            }
                                            (*img).sz = alloc_size as size_t;
                                            current_block = 2;
                                        }
                                    } else {
                                        current_block = 2;
                                    }
                                    match current_block {
                                        0 => {}
                                        _ => {
                                            if !((*img).img_data).is_null() {
                                                (*img).fmt = fmt;
                                                (*img).bit_depth = bit_depth;
                                                (*img).w = w;
                                                (*img).h = h;
                                                (*img).x_chroma_shift = xcs;
                                                (*img).y_chroma_shift = ycs;
                                                (*img).bps = bps as core::ffi::c_int;
                                                (*img).stride[AOM_PLANE_Y as usize] = stride_in_bytes;
                                                (*img).stride[AOM_PLANE_V as usize] = stride_in_bytes
                                                    >> xcs;
                                                (*img).stride[AOM_PLANE_U as usize] = (*img)
                                                    .stride[AOM_PLANE_V as usize];
                                                if fmt as core::ffi::c_uint
                                                    == AOM_IMG_FMT_NV12 as core::ffi::c_int as core::ffi::c_uint
                                                {
                                                    (*img).stride[AOM_PLANE_U as usize]
                                                        *= 2 as core::ffi::c_int;
                                                    (*img).stride[AOM_PLANE_V as usize] = 0 as core::ffi::c_int;
                                                }
                                                (*img).cp = AOM_CICP_CP_UNSPECIFIED;
                                                (*img).tc = AOM_CICP_TC_UNSPECIFIED;
                                                (*img).mc = AOM_CICP_MC_UNSPECIFIED;
                                                ret = aom_img_set_rect(
                                                    img,
                                                    0 as core::ffi::c_uint,
                                                    0 as core::ffi::c_uint,
                                                    d_w,
                                                    d_h,
                                                    border,
                                                );
                                                if ret == 0 as core::ffi::c_int {} else {
                                                    __assert_fail(
                                                        b"ret == 0\0" as *const u8 as *const core::ffi::c_char,
                                                        b"upstream/aom/aom/src/aom_image.c\0" as *const u8
                                                            as *const core::ffi::c_char,
                                                        202 as core::ffi::c_uint,
                                                        __ASSERT_FUNCTION.as_ptr(),
                                                    );
                                                };
                                                return img;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    aom_img_free(img);
    return 0 as *mut aom_image_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_alloc(
    mut img: *mut aom_image_t,
    mut fmt: aom_img_fmt_t,
    mut d_w: core::ffi::c_uint,
    mut d_h: core::ffi::c_uint,
    mut align: core::ffi::c_uint,
) -> *mut aom_image_t {
    return img_alloc_helper(
        img,
        fmt,
        d_w,
        d_h,
        align,
        align,
        1 as core::ffi::c_uint,
        0 as core::ffi::c_uint,
        0 as *mut core::ffi::c_uchar,
        None,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_alloc_with_cb(
    mut img: *mut aom_image_t,
    mut fmt: aom_img_fmt_t,
    mut d_w: core::ffi::c_uint,
    mut d_h: core::ffi::c_uint,
    mut align: core::ffi::c_uint,
    mut alloc_cb: aom_alloc_img_data_cb_fn_t,
    mut cb_priv: *mut core::ffi::c_void,
) -> *mut aom_image_t {
    return img_alloc_helper(
        img,
        fmt,
        d_w,
        d_h,
        align,
        align,
        1 as core::ffi::c_uint,
        0 as core::ffi::c_uint,
        0 as *mut core::ffi::c_uchar,
        alloc_cb,
        cb_priv,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_wrap(
    mut img: *mut aom_image_t,
    mut fmt: aom_img_fmt_t,
    mut d_w: core::ffi::c_uint,
    mut d_h: core::ffi::c_uint,
    mut stride_align: core::ffi::c_uint,
    mut img_data: *mut core::ffi::c_uchar,
) -> *mut aom_image_t {
    return img_alloc_helper(
        img,
        fmt,
        d_w,
        d_h,
        1 as core::ffi::c_uint,
        stride_align,
        1 as core::ffi::c_uint,
        0 as core::ffi::c_uint,
        img_data,
        None,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_alloc_with_border(
    mut img: *mut aom_image_t,
    mut fmt: aom_img_fmt_t,
    mut d_w: core::ffi::c_uint,
    mut d_h: core::ffi::c_uint,
    mut align: core::ffi::c_uint,
    mut size_align: core::ffi::c_uint,
    mut border: core::ffi::c_uint,
) -> *mut aom_image_t {
    return img_alloc_helper(
        img,
        fmt,
        d_w,
        d_h,
        align,
        align,
        size_align,
        border,
        0 as *mut core::ffi::c_uchar,
        None,
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_set_rect(
    mut img: *mut aom_image_t,
    mut x: core::ffi::c_uint,
    mut y: core::ffi::c_uint,
    mut w: core::ffi::c_uint,
    mut h: core::ffi::c_uint,
    mut border: core::ffi::c_uint,
) -> core::ffi::c_int {
    if x <= UINT_MAX.wrapping_sub(w) && x.wrapping_add(w) <= (*img).w
        && y <= UINT_MAX.wrapping_sub(h) && y.wrapping_add(h) <= (*img).h
    {
        (*img).d_w = w;
        (*img).d_h = h;
        x = x.wrapping_add(border);
        y = y.wrapping_add(border);
        if (*img).fmt as core::ffi::c_uint & AOM_IMG_FMT_PLANAR as core::ffi::c_uint == 0
        {
            (*img).planes[AOM_PLANE_PACKED as usize] = ((*img).img_data)
                .offset(
                    x
                        .wrapping_mul((*img).bps as core::ffi::c_uint)
                        .wrapping_div(8 as core::ffi::c_uint) as isize,
                )
                .offset(
                    y
                        .wrapping_mul(
                            (*img).stride[AOM_PLANE_PACKED as usize] as core::ffi::c_uint,
                        ) as isize,
                );
        } else {
            let bytes_per_sample: core::ffi::c_int = if (*img).fmt as core::ffi::c_uint
                & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0
            {
                2 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            };
            let mut data: *mut core::ffi::c_uchar = (*img).img_data;
            (*img).planes[AOM_PLANE_Y as usize] = data
                .offset(x.wrapping_mul(bytes_per_sample as core::ffi::c_uint) as isize)
                .offset(
                    y
                        .wrapping_mul(
                            (*img).stride[AOM_PLANE_Y as usize] as core::ffi::c_uint,
                        ) as isize,
                );
            data = data
                .offset(
                    ((*img).h as size_t)
                        .wrapping_add(
                            (2 as core::ffi::c_uint).wrapping_mul(border) as size_t,
                        )
                        .wrapping_mul((*img).stride[AOM_PLANE_Y as usize] as size_t)
                        as isize,
                );
            let mut uv_border_h: core::ffi::c_uint = border >> (*img).y_chroma_shift;
            let mut uv_x: core::ffi::c_uint = x >> (*img).x_chroma_shift;
            let mut uv_y: core::ffi::c_uint = y >> (*img).y_chroma_shift;
            if (*img).fmt as core::ffi::c_uint
                == AOM_IMG_FMT_NV12 as core::ffi::c_int as core::ffi::c_uint
            {
                (*img).planes[AOM_PLANE_U as usize] = data
                    .offset(
                        uv_x
                            .wrapping_mul(bytes_per_sample as core::ffi::c_uint)
                            .wrapping_mul(2 as core::ffi::c_uint) as isize,
                    )
                    .offset(
                        uv_y
                            .wrapping_mul(
                                (*img).stride[AOM_PLANE_U as usize] as core::ffi::c_uint,
                            ) as isize,
                    );
                (*img).planes[AOM_PLANE_V as usize] = 0 as *mut core::ffi::c_uchar;
            } else if (*img).fmt as core::ffi::c_uint
                & AOM_IMG_FMT_UV_FLIP as core::ffi::c_uint == 0
            {
                (*img).planes[AOM_PLANE_U as usize] = data
                    .offset(
                        uv_x.wrapping_mul(bytes_per_sample as core::ffi::c_uint) as isize,
                    )
                    .offset(
                        uv_y
                            .wrapping_mul(
                                (*img).stride[AOM_PLANE_U as usize] as core::ffi::c_uint,
                            ) as isize,
                    );
                data = data
                    .offset(
                        (((*img).h >> (*img).y_chroma_shift) as size_t)
                            .wrapping_add(
                                (2 as core::ffi::c_uint).wrapping_mul(uv_border_h) as size_t,
                            )
                            .wrapping_mul((*img).stride[AOM_PLANE_U as usize] as size_t)
                            as isize,
                    );
                (*img).planes[AOM_PLANE_V as usize] = data
                    .offset(
                        uv_x.wrapping_mul(bytes_per_sample as core::ffi::c_uint) as isize,
                    )
                    .offset(
                        uv_y
                            .wrapping_mul(
                                (*img).stride[AOM_PLANE_V as usize] as core::ffi::c_uint,
                            ) as isize,
                    );
            } else {
                (*img).planes[AOM_PLANE_V as usize] = data
                    .offset(
                        uv_x.wrapping_mul(bytes_per_sample as core::ffi::c_uint) as isize,
                    )
                    .offset(
                        uv_y
                            .wrapping_mul(
                                (*img).stride[AOM_PLANE_V as usize] as core::ffi::c_uint,
                            ) as isize,
                    );
                data = data
                    .offset(
                        (((*img).h >> (*img).y_chroma_shift) as size_t)
                            .wrapping_add(
                                (2 as core::ffi::c_uint).wrapping_mul(uv_border_h) as size_t,
                            )
                            .wrapping_mul((*img).stride[AOM_PLANE_V as usize] as size_t)
                            as isize,
                    );
                (*img).planes[AOM_PLANE_U as usize] = data
                    .offset(
                        uv_x.wrapping_mul(bytes_per_sample as core::ffi::c_uint) as isize,
                    )
                    .offset(
                        uv_y
                            .wrapping_mul(
                                (*img).stride[AOM_PLANE_U as usize] as core::ffi::c_uint,
                            ) as isize,
                    );
            }
        }
        return 0 as core::ffi::c_int;
    }
    return -(1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_flip(mut img: *mut aom_image_t) {
    let chroma_height: core::ffi::c_uint = ((*img).d_h)
        .wrapping_add((*img).y_chroma_shift) >> (*img).y_chroma_shift;
    if !((*img).planes[AOM_PLANE_Y as usize]).is_null() {
        (*img).planes[AOM_PLANE_Y as usize] = ((*img).planes[AOM_PLANE_Y as usize])
            .offset(
                (((*img).d_h).wrapping_sub(1 as core::ffi::c_uint) as ptrdiff_t
                    * (*img).stride[AOM_PLANE_Y as usize] as ptrdiff_t) as isize,
            );
    }
    (*img).stride[AOM_PLANE_Y as usize] = -(*img).stride[AOM_PLANE_Y as usize];
    if !((*img).planes[AOM_PLANE_U as usize]).is_null() {
        (*img).planes[AOM_PLANE_U as usize] = ((*img).planes[AOM_PLANE_U as usize])
            .offset(
                (chroma_height.wrapping_sub(1 as core::ffi::c_uint) as ptrdiff_t
                    * (*img).stride[AOM_PLANE_U as usize] as ptrdiff_t) as isize,
            );
    }
    (*img).stride[AOM_PLANE_U as usize] = -(*img).stride[AOM_PLANE_U as usize];
    if !((*img).planes[AOM_PLANE_V as usize]).is_null() {
        (*img).planes[AOM_PLANE_V as usize] = ((*img).planes[AOM_PLANE_V as usize])
            .offset(
                (chroma_height.wrapping_sub(1 as core::ffi::c_uint) as ptrdiff_t
                    * (*img).stride[AOM_PLANE_V as usize] as ptrdiff_t) as isize,
            );
    }
    (*img).stride[AOM_PLANE_V as usize] = -(*img).stride[AOM_PLANE_V as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_free(mut img: *mut aom_image_t) {
    if !img.is_null() {
        aom_img_remove_metadata(img);
        if !((*img).img_data).is_null() && (*img).img_data_owner != 0 {
            aom_free((*img).img_data as *mut core::ffi::c_void);
        }
        if (*img).self_allocd != 0 {
            free(img as *mut core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_plane_width(
    mut img: *const aom_image_t,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if plane > 0 as core::ffi::c_int {
        return (((*img).d_w).wrapping_add((*img).x_chroma_shift)
            >> (*img).x_chroma_shift) as core::ffi::c_int
    } else {
        return (*img).d_w as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_plane_height(
    mut img: *const aom_image_t,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if plane > 0 as core::ffi::c_int {
        return (((*img).d_h).wrapping_add((*img).y_chroma_shift)
            >> (*img).y_chroma_shift) as core::ffi::c_int
    } else {
        return (*img).d_h as core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_metadata_alloc(
    mut type_0: uint32_t,
    mut data: *const uint8_t,
    mut sz: size_t,
    mut insert_flag: aom_metadata_insert_flags_t,
) -> *mut aom_metadata_t {
    if data.is_null() || sz == 0 as size_t {
        return 0 as *mut aom_metadata_t;
    }
    let mut metadata: *mut aom_metadata_t = malloc(
        ::core::mem::size_of::<aom_metadata_t>() as size_t,
    ) as *mut aom_metadata_t;
    if metadata.is_null() {
        return 0 as *mut aom_metadata_t;
    }
    (*metadata).type_0 = type_0;
    (*metadata).payload = malloc(sz) as *mut uint8_t;
    if ((*metadata).payload).is_null() {
        free(metadata as *mut core::ffi::c_void);
        return 0 as *mut aom_metadata_t;
    }
    memcpy(
        (*metadata).payload as *mut core::ffi::c_void,
        data as *const core::ffi::c_void,
        sz,
    );
    (*metadata).sz = sz;
    (*metadata).insert_flag = insert_flag;
    return metadata;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_metadata_free(mut metadata: *mut aom_metadata_t) {
    if !metadata.is_null() {
        if !((*metadata).payload).is_null() {
            free((*metadata).payload as *mut core::ffi::c_void);
        }
        free(metadata as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_metadata_array_alloc(
    mut sz: size_t,
) -> *mut aom_metadata_array_t {
    let mut arr: *mut aom_metadata_array_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<aom_metadata_array_t>() as size_t,
    ) as *mut aom_metadata_array_t;
    if arr.is_null() {
        return 0 as *mut aom_metadata_array_t;
    }
    if sz > 0 as size_t {
        (*arr).metadata_array = calloc(
            sz,
            ::core::mem::size_of::<*mut aom_metadata_t>() as size_t,
        ) as *mut *mut aom_metadata_t;
        if ((*arr).metadata_array).is_null() {
            aom_img_metadata_array_free(arr);
            return 0 as *mut aom_metadata_array_t;
        }
        (*arr).sz = sz;
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_metadata_array_free(
    mut arr: *mut aom_metadata_array_t,
) {
    if !arr.is_null() {
        if !((*arr).metadata_array).is_null() {
            let mut i: size_t = 0 as size_t;
            while i < (*arr).sz {
                aom_img_metadata_free(*((*arr).metadata_array).offset(i as isize));
                i = i.wrapping_add(1);
            }
            free((*arr).metadata_array as *mut core::ffi::c_void);
        }
        free(arr as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_add_metadata(
    mut img: *mut aom_image_t,
    mut type_0: uint32_t,
    mut data: *const uint8_t,
    mut sz: size_t,
    mut insert_flag: aom_metadata_insert_flags_t,
) -> core::ffi::c_int {
    if img.is_null() {
        return -(1 as core::ffi::c_int);
    }
    if ((*img).metadata).is_null() {
        (*img).metadata = aom_img_metadata_array_alloc(0 as size_t);
        if ((*img).metadata).is_null() {
            return -(1 as core::ffi::c_int);
        }
    }
    if insert_flag as core::ffi::c_uint & AOM_MIF_LAYER_SPECIFIC as core::ffi::c_uint
        != 0
        && (type_0 == OBU_METADATA_TYPE_SCALABILITY as core::ffi::c_int as uint32_t
            || type_0 == OBU_METADATA_TYPE_TIMECODE as core::ffi::c_int as uint32_t)
    {
        return -(1 as core::ffi::c_int);
    }
    let mut metadata: *mut aom_metadata_t = aom_img_metadata_alloc(
        type_0,
        data,
        sz,
        insert_flag,
    );
    if metadata.is_null() {
        return -(1 as core::ffi::c_int);
    }
    let mut metadata_array: *mut *mut aom_metadata_t = realloc(
        (*(*img).metadata).metadata_array as *mut core::ffi::c_void,
        ((*(*img).metadata).sz)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut aom_metadata_t>() as size_t),
    ) as *mut *mut aom_metadata_t;
    if metadata_array.is_null() {
        aom_img_metadata_free(metadata);
        return -(1 as core::ffi::c_int);
    }
    (*(*img).metadata).metadata_array = metadata_array;
    let fresh0 = &raw mut *((*(*img).metadata).metadata_array)
        .offset((*(*img).metadata).sz as isize);
    *fresh0 = metadata;
    (*(*img).metadata).sz = ((*(*img).metadata).sz).wrapping_add(1);
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_remove_metadata(mut img: *mut aom_image_t) {
    if !img.is_null() && !((*img).metadata).is_null() {
        aom_img_metadata_array_free((*img).metadata);
        (*img).metadata = 0 as *mut aom_metadata_array_t;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_get_metadata(
    mut img: *const aom_image_t,
    mut index: size_t,
) -> *const aom_metadata_t {
    if img.is_null() {
        return 0 as *const aom_metadata_t;
    }
    let mut array: *const aom_metadata_array_t = (*img).metadata;
    if !array.is_null() && index < (*array).sz {
        return *((*array).metadata_array).offset(index as isize);
    }
    return 0 as *const aom_metadata_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_img_num_metadata(mut img: *const aom_image_t) -> size_t {
    if img.is_null() || ((*img).metadata).is_null() {
        return 0 as size_t;
    }
    return (*(*img).metadata).sz;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
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
