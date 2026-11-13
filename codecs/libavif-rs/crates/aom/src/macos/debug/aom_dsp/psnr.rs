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

// Mechanically translated from upstream/aom/aom_dsp/psnr.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static mut aom_highbd_sse: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    static mut aom_sse: Option<unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int, *const uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> int64_t>;
    static mut aom_var_2d_u16: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
    static mut aom_var_2d_u8: Option<unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int) -> uint64_t>;
}
pub type size_t = usize;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSNR_STATS {
    pub psnr: [core::ffi::c_double; 4],
    pub sse: [uint64_t; 4],
    pub samples: [uint32_t; 4],
    pub psnr_hbd: [core::ffi::c_double; 4],
    pub sse_hbd: [uint64_t; 4],
    pub samples_hbd: [uint32_t; 4],
}
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_PSNR: core::ffi::c_double = 100.0f64;
unsafe extern "C-unwind" fn aom_sse_to_psnr(
    mut samples: core::ffi::c_double,
    mut peak: core::ffi::c_double,
    mut sse: core::ffi::c_double,
) -> core::ffi::c_double {
    if sse > 0.0f64 {
        let psnr: core::ffi::c_double = 10.0f64
            * log10(samples * peak * peak / sse) as core::ffi::c_double;
        return if psnr > MAX_PSNR { MAX_PSNR } else { psnr };
    } else {
        return MAX_PSNR
    };
}
unsafe extern "C-unwind" fn encoder_sse(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> int64_t {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut sse: int64_t = 0 as int64_t;
    i = 0 as core::ffi::c_int;
    while i < h {
        j = 0 as core::ffi::c_int;
        while j < w {
            let diff: core::ffi::c_int = *a.offset(j as isize) as core::ffi::c_int
                - *b.offset(j as isize) as core::ffi::c_int;
            sse += (diff * diff) as int64_t;
            j += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        i += 1;
    }
    return sse;
}
unsafe extern "C-unwind" fn encoder_highbd_sse(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> int64_t {
    let mut a: *const uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *const uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut sse: int64_t = 0 as int64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < h {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < w {
            let diff: core::ffi::c_int = *a.offset(j as isize) as core::ffi::c_int
                - *b.offset(j as isize) as core::ffi::c_int;
            sse += (diff * diff) as int64_t;
            j += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        i += 1;
    }
    return sse;
}
unsafe extern "C-unwind" fn get_sse(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    let dw: core::ffi::c_int = width % 16 as core::ffi::c_int;
    let dh: core::ffi::c_int = height % 16 as core::ffi::c_int;
    let mut total_sse: int64_t = 0 as int64_t;
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    if dw > 0 as core::ffi::c_int {
        total_sse
            += encoder_sse(
                &*a.offset((width - dw) as isize),
                a_stride,
                &*b.offset((width - dw) as isize),
                b_stride,
                dw,
                height,
            );
    }
    if dh > 0 as core::ffi::c_int {
        total_sse
            += encoder_sse(
                &*a.offset(((height - dh) * a_stride) as isize),
                a_stride,
                &*b.offset(((height - dh) * b_stride) as isize),
                b_stride,
                width - dw,
                dh,
            );
    }
    y = 0 as core::ffi::c_int;
    while y < height / 16 as core::ffi::c_int {
        let mut pa: *const uint8_t = a;
        let mut pb: *const uint8_t = b;
        x = 0 as core::ffi::c_int;
        while x < width / 16 as core::ffi::c_int {
            total_sse
                += aom_sse.expect("non-null function pointer")(
                    pa,
                    a_stride,
                    pb,
                    b_stride,
                    16 as core::ffi::c_int,
                    16 as core::ffi::c_int,
                );
            pa = pa.offset(16 as core::ffi::c_int as isize);
            pb = pb.offset(16 as core::ffi::c_int as isize);
            x += 1;
        }
        a = a.offset((16 as core::ffi::c_int * a_stride) as isize);
        b = b.offset((16 as core::ffi::c_int * b_stride) as isize);
        y += 1;
    }
    return total_sse;
}
unsafe extern "C-unwind" fn highbd_get_sse_shift(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut input_shift: core::ffi::c_uint,
) -> int64_t {
    let mut a: *const uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *const uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut total_sse: int64_t = 0 as int64_t;
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            let mut diff: int64_t = 0;
            diff = ((*a.offset(x as isize) as core::ffi::c_int >> input_shift)
                - (*b.offset(x as isize) as core::ffi::c_int >> input_shift)) as int64_t;
            total_sse += diff * diff;
            x += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        y += 1;
    }
    return total_sse;
}
unsafe extern "C-unwind" fn highbd_get_sse(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    let mut total_sse: int64_t = 0 as int64_t;
    let mut x: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let dw: core::ffi::c_int = width % 16 as core::ffi::c_int;
    let dh: core::ffi::c_int = height % 16 as core::ffi::c_int;
    if dw > 0 as core::ffi::c_int {
        total_sse
            += encoder_highbd_sse(
                &*a.offset((width - dw) as isize),
                a_stride,
                &*b.offset((width - dw) as isize),
                b_stride,
                dw,
                height,
            );
    }
    if dh > 0 as core::ffi::c_int {
        total_sse
            += encoder_highbd_sse(
                &*a.offset(((height - dh) * a_stride) as isize),
                a_stride,
                &*b.offset(((height - dh) * b_stride) as isize),
                b_stride,
                width - dw,
                dh,
            );
    }
    y = 0 as core::ffi::c_int;
    while y < height / 16 as core::ffi::c_int {
        let mut pa: *const uint8_t = a;
        let mut pb: *const uint8_t = b;
        x = 0 as core::ffi::c_int;
        while x < width / 16 as core::ffi::c_int {
            total_sse
                += aom_highbd_sse.expect("non-null function pointer")(
                    pa,
                    a_stride,
                    pb,
                    b_stride,
                    16 as core::ffi::c_int,
                    16 as core::ffi::c_int,
                );
            pa = pa.offset(16 as core::ffi::c_int as isize);
            pb = pb.offset(16 as core::ffi::c_int as isize);
            x += 1;
        }
        a = a.offset((16 as core::ffi::c_int * a_stride) as isize);
        b = b.offset((16 as core::ffi::c_int * b_stride) as isize);
        y += 1;
    }
    return total_sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_y_var(
    mut a: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    return (aom_var_2d_u8.expect("non-null function pointer")(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        width,
        height,
    ))
        .wrapping_div((width * height) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_u_var(
    mut a: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    return (aom_var_2d_u8.expect("non-null function pointer")(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    ))
        .wrapping_div((width * height) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_v_var(
    mut a: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    return (aom_var_2d_u8.expect("non-null function pointer")(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.v_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    ))
        .wrapping_div((width * height) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_y_sse_part(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    return get_sse(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        ((*b).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
            .offset((vstart * (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
            .offset(hstart as isize),
        (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_y_sse(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> int64_t {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.y_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_get_y_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            193 as core::ffi::c_int,
            b"a->y_crop_width == b->y_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.y_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_get_y_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            194 as core::ffi::c_int,
            b"a->y_crop_height == b->y_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return get_sse(
        (*a).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*b).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
        (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
        (*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_u_sse_part(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    return get_sse(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        ((*b).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
            .offset((vstart * (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_u_sse(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> int64_t {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_get_u_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            210 as core::ffi::c_int,
            b"a->uv_crop_width == b->uv_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_get_u_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            211 as core::ffi::c_int,
            b"a->uv_crop_height == b->uv_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return get_sse(
        (*a).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*b).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_v_sse_part(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    return get_sse(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.v_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        ((*b).c2rust_unnamed_4.c2rust_unnamed.v_buffer)
            .offset((vstart * (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_v_sse(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> int64_t {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_get_v_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            227 as core::ffi::c_int,
            b"a->uv_crop_width == b->uv_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_get_v_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            228 as core::ffi::c_int,
            b"a->uv_crop_height == b->uv_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return get_sse(
        (*a).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*b).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_y_var(
    mut a: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    return (aom_var_2d_u16.expect("non-null function pointer")(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        width,
        height,
    ))
        .wrapping_div((width * height) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_u_var(
    mut a: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    return (aom_var_2d_u16.expect("non-null function pointer")(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    ))
        .wrapping_div((width * height) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_v_var(
    mut a: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> uint64_t {
    return (aom_var_2d_u16.expect("non-null function pointer")(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.v_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    ))
        .wrapping_div((width * height) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_y_sse_part(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    return highbd_get_sse(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        ((*b).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
            .offset((vstart * (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride) as isize)
            .offset(hstart as isize),
        (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_y_sse(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> int64_t {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.y_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_y_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            266 as core::ffi::c_int,
            b"a->y_crop_width == b->y_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.y_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_y_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            267 as core::ffi::c_int,
            b"a->y_crop_height == b->y_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).flags & 8 as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_y_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            268 as core::ffi::c_int,
            b"(a->flags & YV12_FLAG_HIGHBITDEPTH) != 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*b).flags & 8 as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_y_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            269 as core::ffi::c_int,
            b"(b->flags & YV12_FLAG_HIGHBITDEPTH) != 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return highbd_get_sse(
        (*a).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*b).c2rust_unnamed_4.c2rust_unnamed.y_buffer,
        (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
        (*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_u_sse_part(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    return highbd_get_sse(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        ((*b).c2rust_unnamed_4.c2rust_unnamed.u_buffer)
            .offset((vstart * (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_u_sse(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> int64_t {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_u_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            286 as core::ffi::c_int,
            b"a->uv_crop_width == b->uv_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_u_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            287 as core::ffi::c_int,
            b"a->uv_crop_height == b->uv_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).flags & 8 as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_u_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            288 as core::ffi::c_int,
            b"(a->flags & YV12_FLAG_HIGHBITDEPTH) != 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*b).flags & 8 as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_u_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            289 as core::ffi::c_int,
            b"(b->flags & YV12_FLAG_HIGHBITDEPTH) != 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return highbd_get_sse(
        (*a).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*b).c2rust_unnamed_4.c2rust_unnamed.u_buffer,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_v_sse_part(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut hstart: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut vstart: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> int64_t {
    return highbd_get_sse(
        ((*a).c2rust_unnamed_4.c2rust_unnamed.v_buffer)
            .offset((vstart * (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        ((*b).c2rust_unnamed_4.c2rust_unnamed.v_buffer)
            .offset((vstart * (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride) as isize)
            .offset(hstart as isize),
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_get_v_sse(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
) -> int64_t {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_v_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            306 as core::ffi::c_int,
            b"a->uv_crop_width == b->uv_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_v_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            307 as core::ffi::c_int,
            b"a->uv_crop_height == b->uv_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).flags & 8 as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_v_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            308 as core::ffi::c_int,
            b"(a->flags & YV12_FLAG_HIGHBITDEPTH) != 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*b).flags & 8 as core::ffi::c_int != 0 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_highbd_get_v_sse\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            309 as core::ffi::c_int,
            b"(b->flags & YV12_FLAG_HIGHBITDEPTH) != 0\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return highbd_get_sse(
        (*a).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*b).c2rust_unnamed_4.c2rust_unnamed.v_buffer,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_sse_plane(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut plane: core::ffi::c_int,
    mut highbd: core::ffi::c_int,
) -> int64_t {
    if highbd != 0 {
        match plane {
            0 => return aom_highbd_get_y_sse(a, b),
            1 => return aom_highbd_get_u_sse(a, b),
            2 => return aom_highbd_get_v_sse(a, b),
            _ => {
                if !(plane >= 0 as core::ffi::c_int && plane <= 2 as core::ffi::c_int)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 18],
                            [core::ffi::c_char; 18],
                        >(*b"aom_get_sse_plane\0"))
                            .as_ptr(),
                        b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
                        324 as core::ffi::c_int,
                        b"plane >= 0 && plane <= 2\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                return 0 as int64_t;
            }
        }
    } else {
        match plane {
            0 => return aom_get_y_sse(a, b),
            1 => return aom_get_u_sse(a, b),
            2 => return aom_get_v_sse(a, b),
            _ => {
                if !(plane >= 0 as core::ffi::c_int && plane <= 2 as core::ffi::c_int)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 18],
                            [core::ffi::c_char; 18],
                        >(*b"aom_get_sse_plane\0"))
                            .as_ptr(),
                        b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
                        331 as core::ffi::c_int,
                        b"plane >= 0 && plane <= 2\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                return 0 as int64_t;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_calc_highbd_psnr(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut psnr: *mut PSNR_STATS,
    mut bit_depth: uint32_t,
    mut in_bit_depth: uint32_t,
) {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.y_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_calc_highbd_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            349 as core::ffi::c_int,
            b"a->y_crop_width == b->y_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.y_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_calc_highbd_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            350 as core::ffi::c_int,
            b"a->y_crop_height == b->y_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_calc_highbd_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            351 as core::ffi::c_int,
            b"a->uv_crop_width == b->uv_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"aom_calc_highbd_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            352 as core::ffi::c_int,
            b"a->uv_crop_height == b->uv_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let widths: [core::ffi::c_int; 3] = [
        (*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
    ];
    let heights: [core::ffi::c_int; 3] = [
        (*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
    ];
    let a_strides: [core::ffi::c_int; 3] = [
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
    ];
    let b_strides: [core::ffi::c_int; 3] = [
        (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
    ];
    let mut i: core::ffi::c_int = 0;
    let mut total_sse: uint64_t = 0 as uint64_t;
    let mut total_samples: uint32_t = 0 as uint32_t;
    let mut peak: core::ffi::c_double = ((255 as core::ffi::c_int)
        << in_bit_depth.wrapping_sub(8 as uint32_t)) as core::ffi::c_double;
    let input_shift: core::ffi::c_uint = (bit_depth as core::ffi::c_uint)
        .wrapping_sub(in_bit_depth as core::ffi::c_uint);
    i = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        let w: core::ffi::c_int = widths[i as usize];
        let h: core::ffi::c_int = heights[i as usize];
        let samples: uint32_t = (w * h) as uint32_t;
        let mut sse: uint64_t = 0;
        if (*a).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
            if input_shift != 0 {
                sse = highbd_get_sse_shift(
                    (*a).c2rust_unnamed_4.buffers[i as usize],
                    a_strides[i as usize],
                    (*b).c2rust_unnamed_4.buffers[i as usize],
                    b_strides[i as usize],
                    w,
                    h,
                    input_shift,
                ) as uint64_t;
            } else {
                sse = highbd_get_sse(
                    (*a).c2rust_unnamed_4.buffers[i as usize],
                    a_strides[i as usize],
                    (*b).c2rust_unnamed_4.buffers[i as usize],
                    b_strides[i as usize],
                    w,
                    h,
                ) as uint64_t;
            }
        } else {
            sse = get_sse(
                (*a).c2rust_unnamed_4.buffers[i as usize],
                a_strides[i as usize],
                (*b).c2rust_unnamed_4.buffers[i as usize],
                b_strides[i as usize],
                w,
                h,
            ) as uint64_t;
        }
        (*psnr).sse[(1 as core::ffi::c_int + i) as usize] = sse;
        (*psnr).samples[(1 as core::ffi::c_int + i) as usize] = samples;
        (*psnr).psnr[(1 as core::ffi::c_int + i) as usize] = aom_sse_to_psnr(
            samples as core::ffi::c_double,
            peak,
            sse as core::ffi::c_double,
        );
        total_sse = total_sse.wrapping_add(sse);
        total_samples = total_samples.wrapping_add(samples);
        i += 1;
    }
    (*psnr).sse[0 as core::ffi::c_int as usize] = total_sse;
    (*psnr).samples[0 as core::ffi::c_int as usize] = total_samples;
    (*psnr).psnr[0 as core::ffi::c_int as usize] = aom_sse_to_psnr(
        total_samples as core::ffi::c_double,
        peak,
        total_sse as core::ffi::c_double,
    );
    if (*a).flags & YV12_FLAG_HIGHBITDEPTH != 0 && in_bit_depth < bit_depth {
        peak = ((255 as core::ffi::c_int) << bit_depth.wrapping_sub(8 as uint32_t))
            as core::ffi::c_double;
        total_sse = 0 as uint64_t;
        total_samples = 0 as uint32_t;
        i = 0 as core::ffi::c_int;
        while i < 3 as core::ffi::c_int {
            let w_0: core::ffi::c_int = widths[i as usize];
            let h_0: core::ffi::c_int = heights[i as usize];
            let samples_0: uint32_t = (w_0 * h_0) as uint32_t;
            let mut sse_0: uint64_t = 0;
            sse_0 = highbd_get_sse(
                (*a).c2rust_unnamed_4.buffers[i as usize],
                a_strides[i as usize],
                (*b).c2rust_unnamed_4.buffers[i as usize],
                b_strides[i as usize],
                w_0,
                h_0,
            ) as uint64_t;
            (*psnr).sse_hbd[(1 as core::ffi::c_int + i) as usize] = sse_0;
            (*psnr).samples_hbd[(1 as core::ffi::c_int + i) as usize] = samples_0;
            (*psnr).psnr_hbd[(1 as core::ffi::c_int + i) as usize] = aom_sse_to_psnr(
                samples_0 as core::ffi::c_double,
                peak,
                sse_0 as core::ffi::c_double,
            );
            total_sse = total_sse.wrapping_add(sse_0);
            total_samples = total_samples.wrapping_add(samples_0);
            i += 1;
        }
        (*psnr).sse_hbd[0 as core::ffi::c_int as usize] = total_sse;
        (*psnr).samples_hbd[0 as core::ffi::c_int as usize] = total_samples;
        (*psnr).psnr_hbd[0 as core::ffi::c_int as usize] = aom_sse_to_psnr(
            total_samples as core::ffi::c_double,
            peak,
            total_sse as core::ffi::c_double,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_calc_psnr(
    mut a: *const YV12_BUFFER_CONFIG,
    mut b: *const YV12_BUFFER_CONFIG,
    mut psnr: *mut PSNR_STATS,
) {
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.y_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_calc_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            431 as core::ffi::c_int,
            b"a->y_crop_width == b->y_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.y_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_calc_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            432 as core::ffi::c_int,
            b"a->y_crop_height == b->y_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        == (*b).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_calc_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            433 as core::ffi::c_int,
            b"a->uv_crop_width == b->uv_crop_width\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !((*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height
        == (*b).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"aom_calc_psnr\0"))
                .as_ptr(),
            b"psnr.c\0" as *const u8 as *const core::ffi::c_char,
            434 as core::ffi::c_int,
            b"a->uv_crop_height == b->uv_crop_height\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    static mut peak: core::ffi::c_double = 255.0f64;
    let widths: [core::ffi::c_int; 3] = [
        (*a).c2rust_unnamed_1.c2rust_unnamed.y_crop_width,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
        (*a).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width,
    ];
    let heights: [core::ffi::c_int; 3] = [
        (*a).c2rust_unnamed_2.c2rust_unnamed.y_crop_height,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
        (*a).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height,
    ];
    let a_strides: [core::ffi::c_int; 3] = [
        (*a).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*a).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
    ];
    let b_strides: [core::ffi::c_int; 3] = [
        (*b).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*b).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
    ];
    let mut i: core::ffi::c_int = 0;
    let mut total_sse: uint64_t = 0 as uint64_t;
    let mut total_samples: uint32_t = 0 as uint32_t;
    i = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        let w: core::ffi::c_int = widths[i as usize];
        let h: core::ffi::c_int = heights[i as usize];
        let samples: uint32_t = (w * h) as uint32_t;
        let sse: uint64_t = get_sse(
            (*a).c2rust_unnamed_4.buffers[i as usize],
            a_strides[i as usize],
            (*b).c2rust_unnamed_4.buffers[i as usize],
            b_strides[i as usize],
            w,
            h,
        ) as uint64_t;
        (*psnr).sse[(1 as core::ffi::c_int + i) as usize] = sse;
        (*psnr).samples[(1 as core::ffi::c_int + i) as usize] = samples;
        (*psnr).psnr[(1 as core::ffi::c_int + i) as usize] = aom_sse_to_psnr(
            samples as core::ffi::c_double,
            peak,
            sse as core::ffi::c_double,
        );
        total_sse = total_sse.wrapping_add(sse);
        total_samples = total_samples.wrapping_add(samples);
        i += 1;
    }
    (*psnr).sse[0 as core::ffi::c_int as usize] = total_sse;
    (*psnr).samples[0 as core::ffi::c_int as usize] = total_samples;
    (*psnr).psnr[0 as core::ffi::c_int as usize] = aom_sse_to_psnr(
        total_samples as core::ffi::c_double,
        peak,
        total_sse as core::ffi::c_double,
    );
}

extern "C" {
    fn log10(_: core::ffi::c_double) -> core::ffi::c_double;
}
