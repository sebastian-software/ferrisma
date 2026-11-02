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

/*!\file
 * \brief Describes film grain parameters and film grain synthesis
 *
 */

// Mechanically translated from upstream/aom/av1/decoder/grain_synthesis.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type __sFILEX;
    pub type aom_metadata_array;
    static mut __stderrp: *mut FILE;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type int64_t = i64;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut core::ffi::c_uchar,
    pub _size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut core::ffi::c_uchar,
    pub _r: core::ffi::c_int,
    pub _w: core::ffi::c_int,
    pub _flags: core::ffi::c_short,
    pub _file: core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: core::ffi::c_int,
    pub _cookie: *mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C-unwind" fn(*mut core::ffi::c_void, fpos_t, core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: core::ffi::c_int,
    pub _ubuf: [core::ffi::c_uchar; 3],
    pub _nbuf: [core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = libc::FILE;
pub type uint8_t = u8;
pub type uint16_t = u16;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_grain_rng_t {
    pub random_register: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_grain_scaling_lut_t {
    pub y: [core::ffi::c_int; 256],
    pub cb: [core::ffi::c_int; 256],
    pub cr: [core::ffi::c_int; 256],
}
static mut gaussian_sequence: [core::ffi::c_int; 2048] = [
    56 as core::ffi::c_int,
    568 as core::ffi::c_int,
    -(180 as core::ffi::c_int),
    172 as core::ffi::c_int,
    124 as core::ffi::c_int,
    -(84 as core::ffi::c_int),
    172 as core::ffi::c_int,
    -(64 as core::ffi::c_int),
    -(900 as core::ffi::c_int),
    24 as core::ffi::c_int,
    820 as core::ffi::c_int,
    224 as core::ffi::c_int,
    1248 as core::ffi::c_int,
    996 as core::ffi::c_int,
    272 as core::ffi::c_int,
    -(8 as core::ffi::c_int),
    -(916 as core::ffi::c_int),
    -(388 as core::ffi::c_int),
    -(732 as core::ffi::c_int),
    -(104 as core::ffi::c_int),
    -(188 as core::ffi::c_int),
    800 as core::ffi::c_int,
    112 as core::ffi::c_int,
    -(652 as core::ffi::c_int),
    -(320 as core::ffi::c_int),
    -(376 as core::ffi::c_int),
    140 as core::ffi::c_int,
    -(252 as core::ffi::c_int),
    492 as core::ffi::c_int,
    -(168 as core::ffi::c_int),
    44 as core::ffi::c_int,
    -(788 as core::ffi::c_int),
    588 as core::ffi::c_int,
    -(584 as core::ffi::c_int),
    500 as core::ffi::c_int,
    -(228 as core::ffi::c_int),
    12 as core::ffi::c_int,
    680 as core::ffi::c_int,
    272 as core::ffi::c_int,
    -(476 as core::ffi::c_int),
    972 as core::ffi::c_int,
    -(100 as core::ffi::c_int),
    652 as core::ffi::c_int,
    368 as core::ffi::c_int,
    432 as core::ffi::c_int,
    -(196 as core::ffi::c_int),
    -(720 as core::ffi::c_int),
    -(192 as core::ffi::c_int),
    1000 as core::ffi::c_int,
    -(332 as core::ffi::c_int),
    652 as core::ffi::c_int,
    -(136 as core::ffi::c_int),
    -(552 as core::ffi::c_int),
    -(604 as core::ffi::c_int),
    -(4 as core::ffi::c_int),
    192 as core::ffi::c_int,
    -(220 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    1000 as core::ffi::c_int,
    -(52 as core::ffi::c_int),
    372 as core::ffi::c_int,
    -(96 as core::ffi::c_int),
    -(624 as core::ffi::c_int),
    124 as core::ffi::c_int,
    -(24 as core::ffi::c_int),
    396 as core::ffi::c_int,
    540 as core::ffi::c_int,
    -(12 as core::ffi::c_int),
    -(104 as core::ffi::c_int),
    640 as core::ffi::c_int,
    464 as core::ffi::c_int,
    244 as core::ffi::c_int,
    -(208 as core::ffi::c_int),
    -(84 as core::ffi::c_int),
    368 as core::ffi::c_int,
    -(528 as core::ffi::c_int),
    -(740 as core::ffi::c_int),
    248 as core::ffi::c_int,
    -(968 as core::ffi::c_int),
    -(848 as core::ffi::c_int),
    608 as core::ffi::c_int,
    376 as core::ffi::c_int,
    -(60 as core::ffi::c_int),
    -(292 as core::ffi::c_int),
    -(40 as core::ffi::c_int),
    -(156 as core::ffi::c_int),
    252 as core::ffi::c_int,
    -(292 as core::ffi::c_int),
    248 as core::ffi::c_int,
    224 as core::ffi::c_int,
    -(280 as core::ffi::c_int),
    400 as core::ffi::c_int,
    -(244 as core::ffi::c_int),
    244 as core::ffi::c_int,
    -(60 as core::ffi::c_int),
    76 as core::ffi::c_int,
    -(80 as core::ffi::c_int),
    212 as core::ffi::c_int,
    532 as core::ffi::c_int,
    340 as core::ffi::c_int,
    128 as core::ffi::c_int,
    -(36 as core::ffi::c_int),
    824 as core::ffi::c_int,
    -(352 as core::ffi::c_int),
    -(60 as core::ffi::c_int),
    -(264 as core::ffi::c_int),
    -(96 as core::ffi::c_int),
    -(612 as core::ffi::c_int),
    416 as core::ffi::c_int,
    -(704 as core::ffi::c_int),
    220 as core::ffi::c_int,
    -(204 as core::ffi::c_int),
    640 as core::ffi::c_int,
    -(160 as core::ffi::c_int),
    1220 as core::ffi::c_int,
    -(408 as core::ffi::c_int),
    900 as core::ffi::c_int,
    336 as core::ffi::c_int,
    20 as core::ffi::c_int,
    -(336 as core::ffi::c_int),
    -(96 as core::ffi::c_int),
    -(792 as core::ffi::c_int),
    304 as core::ffi::c_int,
    48 as core::ffi::c_int,
    -(28 as core::ffi::c_int),
    -(1232 as core::ffi::c_int),
    -(1172 as core::ffi::c_int),
    -(448 as core::ffi::c_int),
    104 as core::ffi::c_int,
    -(292 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    244 as core::ffi::c_int,
    60 as core::ffi::c_int,
    -(948 as core::ffi::c_int),
    0 as core::ffi::c_int,
    -(708 as core::ffi::c_int),
    268 as core::ffi::c_int,
    108 as core::ffi::c_int,
    356 as core::ffi::c_int,
    -(548 as core::ffi::c_int),
    488 as core::ffi::c_int,
    -(344 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    488 as core::ffi::c_int,
    -(196 as core::ffi::c_int),
    -(224 as core::ffi::c_int),
    656 as core::ffi::c_int,
    -(236 as core::ffi::c_int),
    -(1128 as core::ffi::c_int),
    60 as core::ffi::c_int,
    4 as core::ffi::c_int,
    140 as core::ffi::c_int,
    276 as core::ffi::c_int,
    -(676 as core::ffi::c_int),
    -(376 as core::ffi::c_int),
    168 as core::ffi::c_int,
    -(108 as core::ffi::c_int),
    464 as core::ffi::c_int,
    8 as core::ffi::c_int,
    564 as core::ffi::c_int,
    64 as core::ffi::c_int,
    240 as core::ffi::c_int,
    308 as core::ffi::c_int,
    -(300 as core::ffi::c_int),
    -(400 as core::ffi::c_int),
    -(456 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    56 as core::ffi::c_int,
    120 as core::ffi::c_int,
    -(408 as core::ffi::c_int),
    -(116 as core::ffi::c_int),
    436 as core::ffi::c_int,
    504 as core::ffi::c_int,
    -(232 as core::ffi::c_int),
    328 as core::ffi::c_int,
    844 as core::ffi::c_int,
    -(164 as core::ffi::c_int),
    -(84 as core::ffi::c_int),
    784 as core::ffi::c_int,
    -(168 as core::ffi::c_int),
    232 as core::ffi::c_int,
    -(224 as core::ffi::c_int),
    348 as core::ffi::c_int,
    -(376 as core::ffi::c_int),
    128 as core::ffi::c_int,
    568 as core::ffi::c_int,
    96 as core::ffi::c_int,
    -(1244 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    276 as core::ffi::c_int,
    848 as core::ffi::c_int,
    832 as core::ffi::c_int,
    -(360 as core::ffi::c_int),
    656 as core::ffi::c_int,
    464 as core::ffi::c_int,
    -(384 as core::ffi::c_int),
    -(332 as core::ffi::c_int),
    -(356 as core::ffi::c_int),
    728 as core::ffi::c_int,
    -(388 as core::ffi::c_int),
    160 as core::ffi::c_int,
    -(192 as core::ffi::c_int),
    468 as core::ffi::c_int,
    296 as core::ffi::c_int,
    224 as core::ffi::c_int,
    140 as core::ffi::c_int,
    -(776 as core::ffi::c_int),
    -(100 as core::ffi::c_int),
    280 as core::ffi::c_int,
    4 as core::ffi::c_int,
    196 as core::ffi::c_int,
    44 as core::ffi::c_int,
    -(36 as core::ffi::c_int),
    -(648 as core::ffi::c_int),
    932 as core::ffi::c_int,
    16 as core::ffi::c_int,
    1428 as core::ffi::c_int,
    28 as core::ffi::c_int,
    528 as core::ffi::c_int,
    808 as core::ffi::c_int,
    772 as core::ffi::c_int,
    20 as core::ffi::c_int,
    268 as core::ffi::c_int,
    88 as core::ffi::c_int,
    -(332 as core::ffi::c_int),
    -(284 as core::ffi::c_int),
    124 as core::ffi::c_int,
    -(384 as core::ffi::c_int),
    -(448 as core::ffi::c_int),
    208 as core::ffi::c_int,
    -(228 as core::ffi::c_int),
    -(1044 as core::ffi::c_int),
    -(328 as core::ffi::c_int),
    660 as core::ffi::c_int,
    380 as core::ffi::c_int,
    -(148 as core::ffi::c_int),
    -(300 as core::ffi::c_int),
    588 as core::ffi::c_int,
    240 as core::ffi::c_int,
    540 as core::ffi::c_int,
    28 as core::ffi::c_int,
    136 as core::ffi::c_int,
    -(88 as core::ffi::c_int),
    -(436 as core::ffi::c_int),
    256 as core::ffi::c_int,
    296 as core::ffi::c_int,
    -(1000 as core::ffi::c_int),
    1400 as core::ffi::c_int,
    0 as core::ffi::c_int,
    -(48 as core::ffi::c_int),
    1056 as core::ffi::c_int,
    -(136 as core::ffi::c_int),
    264 as core::ffi::c_int,
    -(528 as core::ffi::c_int),
    -(1108 as core::ffi::c_int),
    632 as core::ffi::c_int,
    -(484 as core::ffi::c_int),
    -(592 as core::ffi::c_int),
    -(344 as core::ffi::c_int),
    796 as core::ffi::c_int,
    124 as core::ffi::c_int,
    -(668 as core::ffi::c_int),
    -(768 as core::ffi::c_int),
    388 as core::ffi::c_int,
    1296 as core::ffi::c_int,
    -(232 as core::ffi::c_int),
    -(188 as core::ffi::c_int),
    -(200 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    -(4 as core::ffi::c_int),
    308 as core::ffi::c_int,
    100 as core::ffi::c_int,
    -(168 as core::ffi::c_int),
    256 as core::ffi::c_int,
    -(500 as core::ffi::c_int),
    204 as core::ffi::c_int,
    -(508 as core::ffi::c_int),
    648 as core::ffi::c_int,
    -(136 as core::ffi::c_int),
    372 as core::ffi::c_int,
    -(272 as core::ffi::c_int),
    -(120 as core::ffi::c_int),
    -(1004 as core::ffi::c_int),
    -(552 as core::ffi::c_int),
    -(548 as core::ffi::c_int),
    -(384 as core::ffi::c_int),
    548 as core::ffi::c_int,
    -(296 as core::ffi::c_int),
    428 as core::ffi::c_int,
    -(108 as core::ffi::c_int),
    -(8 as core::ffi::c_int),
    -(912 as core::ffi::c_int),
    -(324 as core::ffi::c_int),
    -(224 as core::ffi::c_int),
    -(88 as core::ffi::c_int),
    -(112 as core::ffi::c_int),
    -(220 as core::ffi::c_int),
    -(100 as core::ffi::c_int),
    996 as core::ffi::c_int,
    -(796 as core::ffi::c_int),
    548 as core::ffi::c_int,
    360 as core::ffi::c_int,
    -(216 as core::ffi::c_int),
    180 as core::ffi::c_int,
    428 as core::ffi::c_int,
    -(200 as core::ffi::c_int),
    -(212 as core::ffi::c_int),
    148 as core::ffi::c_int,
    96 as core::ffi::c_int,
    148 as core::ffi::c_int,
    284 as core::ffi::c_int,
    216 as core::ffi::c_int,
    -(412 as core::ffi::c_int),
    -(320 as core::ffi::c_int),
    120 as core::ffi::c_int,
    -(300 as core::ffi::c_int),
    -(384 as core::ffi::c_int),
    -(604 as core::ffi::c_int),
    -(572 as core::ffi::c_int),
    -(332 as core::ffi::c_int),
    -(8 as core::ffi::c_int),
    -(180 as core::ffi::c_int),
    -(176 as core::ffi::c_int),
    696 as core::ffi::c_int,
    116 as core::ffi::c_int,
    -(88 as core::ffi::c_int),
    628 as core::ffi::c_int,
    76 as core::ffi::c_int,
    44 as core::ffi::c_int,
    -(516 as core::ffi::c_int),
    240 as core::ffi::c_int,
    -(208 as core::ffi::c_int),
    -(40 as core::ffi::c_int),
    100 as core::ffi::c_int,
    -(592 as core::ffi::c_int),
    344 as core::ffi::c_int,
    -(308 as core::ffi::c_int),
    -(452 as core::ffi::c_int),
    -(228 as core::ffi::c_int),
    20 as core::ffi::c_int,
    916 as core::ffi::c_int,
    -(1752 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    -(340 as core::ffi::c_int),
    -(804 as core::ffi::c_int),
    140 as core::ffi::c_int,
    40 as core::ffi::c_int,
    512 as core::ffi::c_int,
    340 as core::ffi::c_int,
    248 as core::ffi::c_int,
    184 as core::ffi::c_int,
    -(492 as core::ffi::c_int),
    896 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    932 as core::ffi::c_int,
    -(628 as core::ffi::c_int),
    328 as core::ffi::c_int,
    -(688 as core::ffi::c_int),
    -(448 as core::ffi::c_int),
    -(616 as core::ffi::c_int),
    -(752 as core::ffi::c_int),
    -(100 as core::ffi::c_int),
    560 as core::ffi::c_int,
    -(1020 as core::ffi::c_int),
    180 as core::ffi::c_int,
    -(800 as core::ffi::c_int),
    -(64 as core::ffi::c_int),
    76 as core::ffi::c_int,
    576 as core::ffi::c_int,
    1068 as core::ffi::c_int,
    396 as core::ffi::c_int,
    660 as core::ffi::c_int,
    552 as core::ffi::c_int,
    -(108 as core::ffi::c_int),
    -(28 as core::ffi::c_int),
    320 as core::ffi::c_int,
    -(628 as core::ffi::c_int),
    312 as core::ffi::c_int,
    -(92 as core::ffi::c_int),
    -(92 as core::ffi::c_int),
    -(472 as core::ffi::c_int),
    268 as core::ffi::c_int,
    16 as core::ffi::c_int,
    560 as core::ffi::c_int,
    516 as core::ffi::c_int,
    -(672 as core::ffi::c_int),
    -(52 as core::ffi::c_int),
    492 as core::ffi::c_int,
    -(100 as core::ffi::c_int),
    260 as core::ffi::c_int,
    384 as core::ffi::c_int,
    284 as core::ffi::c_int,
    292 as core::ffi::c_int,
    304 as core::ffi::c_int,
    -(148 as core::ffi::c_int),
    88 as core::ffi::c_int,
    -(152 as core::ffi::c_int),
    1012 as core::ffi::c_int,
    1064 as core::ffi::c_int,
    -(228 as core::ffi::c_int),
    164 as core::ffi::c_int,
    -(376 as core::ffi::c_int),
    -(684 as core::ffi::c_int),
    592 as core::ffi::c_int,
    -(392 as core::ffi::c_int),
    156 as core::ffi::c_int,
    196 as core::ffi::c_int,
    -(524 as core::ffi::c_int),
    -(64 as core::ffi::c_int),
    -(884 as core::ffi::c_int),
    160 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    636 as core::ffi::c_int,
    648 as core::ffi::c_int,
    404 as core::ffi::c_int,
    -(396 as core::ffi::c_int),
    -(436 as core::ffi::c_int),
    864 as core::ffi::c_int,
    424 as core::ffi::c_int,
    -(728 as core::ffi::c_int),
    988 as core::ffi::c_int,
    -(604 as core::ffi::c_int),
    904 as core::ffi::c_int,
    -(592 as core::ffi::c_int),
    296 as core::ffi::c_int,
    -(224 as core::ffi::c_int),
    536 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    -(920 as core::ffi::c_int),
    436 as core::ffi::c_int,
    -(48 as core::ffi::c_int),
    1176 as core::ffi::c_int,
    -(884 as core::ffi::c_int),
    416 as core::ffi::c_int,
    -(776 as core::ffi::c_int),
    -(824 as core::ffi::c_int),
    -(884 as core::ffi::c_int),
    524 as core::ffi::c_int,
    -(548 as core::ffi::c_int),
    -(564 as core::ffi::c_int),
    -(68 as core::ffi::c_int),
    -(164 as core::ffi::c_int),
    -(96 as core::ffi::c_int),
    692 as core::ffi::c_int,
    364 as core::ffi::c_int,
    -(692 as core::ffi::c_int),
    -(1012 as core::ffi::c_int),
    -(68 as core::ffi::c_int),
    260 as core::ffi::c_int,
    -(480 as core::ffi::c_int),
    876 as core::ffi::c_int,
    -(1116 as core::ffi::c_int),
    452 as core::ffi::c_int,
    -(332 as core::ffi::c_int),
    -(352 as core::ffi::c_int),
    892 as core::ffi::c_int,
    -(1088 as core::ffi::c_int),
    1220 as core::ffi::c_int,
    -(676 as core::ffi::c_int),
    12 as core::ffi::c_int,
    -(292 as core::ffi::c_int),
    244 as core::ffi::c_int,
    496 as core::ffi::c_int,
    372 as core::ffi::c_int,
    -(32 as core::ffi::c_int),
    280 as core::ffi::c_int,
    200 as core::ffi::c_int,
    112 as core::ffi::c_int,
    -(440 as core::ffi::c_int),
    -(96 as core::ffi::c_int),
    24 as core::ffi::c_int,
    -(644 as core::ffi::c_int),
    -(184 as core::ffi::c_int),
    56 as core::ffi::c_int,
    -(432 as core::ffi::c_int),
    224 as core::ffi::c_int,
    -(980 as core::ffi::c_int),
    272 as core::ffi::c_int,
    -(260 as core::ffi::c_int),
    144 as core::ffi::c_int,
    -(436 as core::ffi::c_int),
    420 as core::ffi::c_int,
    356 as core::ffi::c_int,
    364 as core::ffi::c_int,
    -(528 as core::ffi::c_int),
    76 as core::ffi::c_int,
    172 as core::ffi::c_int,
    -(744 as core::ffi::c_int),
    -(368 as core::ffi::c_int),
    404 as core::ffi::c_int,
    -(752 as core::ffi::c_int),
    -(416 as core::ffi::c_int),
    684 as core::ffi::c_int,
    -(688 as core::ffi::c_int),
    72 as core::ffi::c_int,
    540 as core::ffi::c_int,
    416 as core::ffi::c_int,
    92 as core::ffi::c_int,
    444 as core::ffi::c_int,
    480 as core::ffi::c_int,
    -(72 as core::ffi::c_int),
    -(1416 as core::ffi::c_int),
    164 as core::ffi::c_int,
    -(1172 as core::ffi::c_int),
    -(68 as core::ffi::c_int),
    24 as core::ffi::c_int,
    424 as core::ffi::c_int,
    264 as core::ffi::c_int,
    1040 as core::ffi::c_int,
    128 as core::ffi::c_int,
    -(912 as core::ffi::c_int),
    -(524 as core::ffi::c_int),
    -(356 as core::ffi::c_int),
    64 as core::ffi::c_int,
    876 as core::ffi::c_int,
    -(12 as core::ffi::c_int),
    4 as core::ffi::c_int,
    -(88 as core::ffi::c_int),
    532 as core::ffi::c_int,
    272 as core::ffi::c_int,
    -(524 as core::ffi::c_int),
    320 as core::ffi::c_int,
    276 as core::ffi::c_int,
    -(508 as core::ffi::c_int),
    940 as core::ffi::c_int,
    24 as core::ffi::c_int,
    -(400 as core::ffi::c_int),
    -(120 as core::ffi::c_int),
    756 as core::ffi::c_int,
    60 as core::ffi::c_int,
    236 as core::ffi::c_int,
    -(412 as core::ffi::c_int),
    100 as core::ffi::c_int,
    376 as core::ffi::c_int,
    -(484 as core::ffi::c_int),
    400 as core::ffi::c_int,
    -(100 as core::ffi::c_int),
    -(740 as core::ffi::c_int),
    -(108 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    328 as core::ffi::c_int,
    -(268 as core::ffi::c_int),
    224 as core::ffi::c_int,
    -(200 as core::ffi::c_int),
    -(416 as core::ffi::c_int),
    184 as core::ffi::c_int,
    -(604 as core::ffi::c_int),
    -(564 as core::ffi::c_int),
    -(20 as core::ffi::c_int),
    296 as core::ffi::c_int,
    60 as core::ffi::c_int,
    892 as core::ffi::c_int,
    -(888 as core::ffi::c_int),
    60 as core::ffi::c_int,
    164 as core::ffi::c_int,
    68 as core::ffi::c_int,
    -(760 as core::ffi::c_int),
    216 as core::ffi::c_int,
    -(296 as core::ffi::c_int),
    904 as core::ffi::c_int,
    -(336 as core::ffi::c_int),
    -(28 as core::ffi::c_int),
    404 as core::ffi::c_int,
    -(356 as core::ffi::c_int),
    -(568 as core::ffi::c_int),
    -(208 as core::ffi::c_int),
    -(1480 as core::ffi::c_int),
    -(512 as core::ffi::c_int),
    296 as core::ffi::c_int,
    328 as core::ffi::c_int,
    -(360 as core::ffi::c_int),
    -(164 as core::ffi::c_int),
    -(1560 as core::ffi::c_int),
    -(776 as core::ffi::c_int),
    1156 as core::ffi::c_int,
    -(428 as core::ffi::c_int),
    164 as core::ffi::c_int,
    -(504 as core::ffi::c_int),
    -(112 as core::ffi::c_int),
    120 as core::ffi::c_int,
    -(216 as core::ffi::c_int),
    -(148 as core::ffi::c_int),
    -(264 as core::ffi::c_int),
    308 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    -(72 as core::ffi::c_int),
    72 as core::ffi::c_int,
    116 as core::ffi::c_int,
    176 as core::ffi::c_int,
    -(64 as core::ffi::c_int),
    -(272 as core::ffi::c_int),
    460 as core::ffi::c_int,
    -(536 as core::ffi::c_int),
    -(784 as core::ffi::c_int),
    -(280 as core::ffi::c_int),
    348 as core::ffi::c_int,
    108 as core::ffi::c_int,
    -(752 as core::ffi::c_int),
    -(132 as core::ffi::c_int),
    524 as core::ffi::c_int,
    -(540 as core::ffi::c_int),
    -(776 as core::ffi::c_int),
    116 as core::ffi::c_int,
    -(296 as core::ffi::c_int),
    -(1196 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    -(560 as core::ffi::c_int),
    1040 as core::ffi::c_int,
    -(472 as core::ffi::c_int),
    116 as core::ffi::c_int,
    -(848 as core::ffi::c_int),
    -(1116 as core::ffi::c_int),
    116 as core::ffi::c_int,
    636 as core::ffi::c_int,
    696 as core::ffi::c_int,
    284 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    1016 as core::ffi::c_int,
    204 as core::ffi::c_int,
    -(864 as core::ffi::c_int),
    -(648 as core::ffi::c_int),
    -(248 as core::ffi::c_int),
    356 as core::ffi::c_int,
    972 as core::ffi::c_int,
    -(584 as core::ffi::c_int),
    -(204 as core::ffi::c_int),
    264 as core::ffi::c_int,
    880 as core::ffi::c_int,
    528 as core::ffi::c_int,
    -(24 as core::ffi::c_int),
    -(184 as core::ffi::c_int),
    116 as core::ffi::c_int,
    448 as core::ffi::c_int,
    -(144 as core::ffi::c_int),
    828 as core::ffi::c_int,
    524 as core::ffi::c_int,
    212 as core::ffi::c_int,
    -(212 as core::ffi::c_int),
    52 as core::ffi::c_int,
    12 as core::ffi::c_int,
    200 as core::ffi::c_int,
    268 as core::ffi::c_int,
    -(488 as core::ffi::c_int),
    -(404 as core::ffi::c_int),
    -(880 as core::ffi::c_int),
    824 as core::ffi::c_int,
    -(672 as core::ffi::c_int),
    -(40 as core::ffi::c_int),
    908 as core::ffi::c_int,
    -(248 as core::ffi::c_int),
    500 as core::ffi::c_int,
    716 as core::ffi::c_int,
    -(576 as core::ffi::c_int),
    492 as core::ffi::c_int,
    -(576 as core::ffi::c_int),
    16 as core::ffi::c_int,
    720 as core::ffi::c_int,
    -(108 as core::ffi::c_int),
    384 as core::ffi::c_int,
    124 as core::ffi::c_int,
    344 as core::ffi::c_int,
    280 as core::ffi::c_int,
    576 as core::ffi::c_int,
    -(500 as core::ffi::c_int),
    252 as core::ffi::c_int,
    104 as core::ffi::c_int,
    -(308 as core::ffi::c_int),
    196 as core::ffi::c_int,
    -(188 as core::ffi::c_int),
    -(8 as core::ffi::c_int),
    1268 as core::ffi::c_int,
    296 as core::ffi::c_int,
    1032 as core::ffi::c_int,
    -(1196 as core::ffi::c_int),
    436 as core::ffi::c_int,
    316 as core::ffi::c_int,
    372 as core::ffi::c_int,
    -(432 as core::ffi::c_int),
    -(200 as core::ffi::c_int),
    -(660 as core::ffi::c_int),
    704 as core::ffi::c_int,
    -(224 as core::ffi::c_int),
    596 as core::ffi::c_int,
    -(132 as core::ffi::c_int),
    268 as core::ffi::c_int,
    32 as core::ffi::c_int,
    -(452 as core::ffi::c_int),
    884 as core::ffi::c_int,
    104 as core::ffi::c_int,
    -(1008 as core::ffi::c_int),
    424 as core::ffi::c_int,
    -(1348 as core::ffi::c_int),
    -(280 as core::ffi::c_int),
    4 as core::ffi::c_int,
    -(1168 as core::ffi::c_int),
    368 as core::ffi::c_int,
    476 as core::ffi::c_int,
    696 as core::ffi::c_int,
    300 as core::ffi::c_int,
    -(8 as core::ffi::c_int),
    24 as core::ffi::c_int,
    180 as core::ffi::c_int,
    -(592 as core::ffi::c_int),
    -(196 as core::ffi::c_int),
    388 as core::ffi::c_int,
    304 as core::ffi::c_int,
    500 as core::ffi::c_int,
    724 as core::ffi::c_int,
    -(160 as core::ffi::c_int),
    244 as core::ffi::c_int,
    -(84 as core::ffi::c_int),
    272 as core::ffi::c_int,
    -(256 as core::ffi::c_int),
    -(420 as core::ffi::c_int),
    320 as core::ffi::c_int,
    208 as core::ffi::c_int,
    -(144 as core::ffi::c_int),
    -(156 as core::ffi::c_int),
    156 as core::ffi::c_int,
    364 as core::ffi::c_int,
    452 as core::ffi::c_int,
    28 as core::ffi::c_int,
    540 as core::ffi::c_int,
    316 as core::ffi::c_int,
    220 as core::ffi::c_int,
    -(644 as core::ffi::c_int),
    -(248 as core::ffi::c_int),
    464 as core::ffi::c_int,
    72 as core::ffi::c_int,
    360 as core::ffi::c_int,
    32 as core::ffi::c_int,
    -(388 as core::ffi::c_int),
    496 as core::ffi::c_int,
    -(680 as core::ffi::c_int),
    -(48 as core::ffi::c_int),
    208 as core::ffi::c_int,
    -(116 as core::ffi::c_int),
    -(408 as core::ffi::c_int),
    60 as core::ffi::c_int,
    -(604 as core::ffi::c_int),
    -(392 as core::ffi::c_int),
    548 as core::ffi::c_int,
    -(840 as core::ffi::c_int),
    784 as core::ffi::c_int,
    -(460 as core::ffi::c_int),
    656 as core::ffi::c_int,
    -(544 as core::ffi::c_int),
    -(388 as core::ffi::c_int),
    -(264 as core::ffi::c_int),
    908 as core::ffi::c_int,
    -(800 as core::ffi::c_int),
    -(628 as core::ffi::c_int),
    -(612 as core::ffi::c_int),
    -(568 as core::ffi::c_int),
    572 as core::ffi::c_int,
    -(220 as core::ffi::c_int),
    164 as core::ffi::c_int,
    288 as core::ffi::c_int,
    -(16 as core::ffi::c_int),
    -(308 as core::ffi::c_int),
    308 as core::ffi::c_int,
    -(112 as core::ffi::c_int),
    -(636 as core::ffi::c_int),
    -(760 as core::ffi::c_int),
    280 as core::ffi::c_int,
    -(668 as core::ffi::c_int),
    432 as core::ffi::c_int,
    364 as core::ffi::c_int,
    240 as core::ffi::c_int,
    -(196 as core::ffi::c_int),
    604 as core::ffi::c_int,
    340 as core::ffi::c_int,
    384 as core::ffi::c_int,
    196 as core::ffi::c_int,
    592 as core::ffi::c_int,
    -(44 as core::ffi::c_int),
    -(500 as core::ffi::c_int),
    432 as core::ffi::c_int,
    -(580 as core::ffi::c_int),
    -(132 as core::ffi::c_int),
    636 as core::ffi::c_int,
    -(76 as core::ffi::c_int),
    392 as core::ffi::c_int,
    4 as core::ffi::c_int,
    -(412 as core::ffi::c_int),
    540 as core::ffi::c_int,
    508 as core::ffi::c_int,
    328 as core::ffi::c_int,
    -(356 as core::ffi::c_int),
    -(36 as core::ffi::c_int),
    16 as core::ffi::c_int,
    -(220 as core::ffi::c_int),
    -(64 as core::ffi::c_int),
    -(248 as core::ffi::c_int),
    -(60 as core::ffi::c_int),
    24 as core::ffi::c_int,
    -(192 as core::ffi::c_int),
    368 as core::ffi::c_int,
    1040 as core::ffi::c_int,
    92 as core::ffi::c_int,
    -(24 as core::ffi::c_int),
    -(1044 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    40 as core::ffi::c_int,
    104 as core::ffi::c_int,
    148 as core::ffi::c_int,
    192 as core::ffi::c_int,
    -(136 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    56 as core::ffi::c_int,
    -(816 as core::ffi::c_int),
    -(224 as core::ffi::c_int),
    732 as core::ffi::c_int,
    392 as core::ffi::c_int,
    356 as core::ffi::c_int,
    212 as core::ffi::c_int,
    -(80 as core::ffi::c_int),
    -(424 as core::ffi::c_int),
    -(1008 as core::ffi::c_int),
    -(324 as core::ffi::c_int),
    588 as core::ffi::c_int,
    -(1496 as core::ffi::c_int),
    576 as core::ffi::c_int,
    460 as core::ffi::c_int,
    -(816 as core::ffi::c_int),
    -(848 as core::ffi::c_int),
    56 as core::ffi::c_int,
    -(580 as core::ffi::c_int),
    -(92 as core::ffi::c_int),
    -(1372 as core::ffi::c_int),
    -(112 as core::ffi::c_int),
    -(496 as core::ffi::c_int),
    200 as core::ffi::c_int,
    364 as core::ffi::c_int,
    52 as core::ffi::c_int,
    -(140 as core::ffi::c_int),
    48 as core::ffi::c_int,
    -(48 as core::ffi::c_int),
    -(60 as core::ffi::c_int),
    84 as core::ffi::c_int,
    72 as core::ffi::c_int,
    40 as core::ffi::c_int,
    132 as core::ffi::c_int,
    -(356 as core::ffi::c_int),
    -(268 as core::ffi::c_int),
    -(104 as core::ffi::c_int),
    -(284 as core::ffi::c_int),
    -(404 as core::ffi::c_int),
    732 as core::ffi::c_int,
    -(520 as core::ffi::c_int),
    164 as core::ffi::c_int,
    -(304 as core::ffi::c_int),
    -(540 as core::ffi::c_int),
    120 as core::ffi::c_int,
    328 as core::ffi::c_int,
    -(76 as core::ffi::c_int),
    -(460 as core::ffi::c_int),
    756 as core::ffi::c_int,
    388 as core::ffi::c_int,
    588 as core::ffi::c_int,
    236 as core::ffi::c_int,
    -(436 as core::ffi::c_int),
    -(72 as core::ffi::c_int),
    -(176 as core::ffi::c_int),
    -(404 as core::ffi::c_int),
    -(316 as core::ffi::c_int),
    -(148 as core::ffi::c_int),
    716 as core::ffi::c_int,
    -(604 as core::ffi::c_int),
    404 as core::ffi::c_int,
    -(72 as core::ffi::c_int),
    -(88 as core::ffi::c_int),
    -(888 as core::ffi::c_int),
    -(68 as core::ffi::c_int),
    944 as core::ffi::c_int,
    88 as core::ffi::c_int,
    -(220 as core::ffi::c_int),
    -(344 as core::ffi::c_int),
    960 as core::ffi::c_int,
    472 as core::ffi::c_int,
    460 as core::ffi::c_int,
    -(232 as core::ffi::c_int),
    704 as core::ffi::c_int,
    120 as core::ffi::c_int,
    832 as core::ffi::c_int,
    -(228 as core::ffi::c_int),
    692 as core::ffi::c_int,
    -(508 as core::ffi::c_int),
    132 as core::ffi::c_int,
    -(476 as core::ffi::c_int),
    844 as core::ffi::c_int,
    -(748 as core::ffi::c_int),
    -(364 as core::ffi::c_int),
    -(44 as core::ffi::c_int),
    1116 as core::ffi::c_int,
    -(1104 as core::ffi::c_int),
    -(1056 as core::ffi::c_int),
    76 as core::ffi::c_int,
    428 as core::ffi::c_int,
    552 as core::ffi::c_int,
    -(692 as core::ffi::c_int),
    60 as core::ffi::c_int,
    356 as core::ffi::c_int,
    96 as core::ffi::c_int,
    -(384 as core::ffi::c_int),
    -(188 as core::ffi::c_int),
    -(612 as core::ffi::c_int),
    -(576 as core::ffi::c_int),
    736 as core::ffi::c_int,
    508 as core::ffi::c_int,
    892 as core::ffi::c_int,
    352 as core::ffi::c_int,
    -(1132 as core::ffi::c_int),
    504 as core::ffi::c_int,
    -(24 as core::ffi::c_int),
    -(352 as core::ffi::c_int),
    324 as core::ffi::c_int,
    332 as core::ffi::c_int,
    -(600 as core::ffi::c_int),
    -(312 as core::ffi::c_int),
    292 as core::ffi::c_int,
    508 as core::ffi::c_int,
    -(144 as core::ffi::c_int),
    -(8 as core::ffi::c_int),
    484 as core::ffi::c_int,
    48 as core::ffi::c_int,
    284 as core::ffi::c_int,
    -(260 as core::ffi::c_int),
    -(240 as core::ffi::c_int),
    256 as core::ffi::c_int,
    -(100 as core::ffi::c_int),
    -(292 as core::ffi::c_int),
    -(204 as core::ffi::c_int),
    -(44 as core::ffi::c_int),
    472 as core::ffi::c_int,
    -(204 as core::ffi::c_int),
    908 as core::ffi::c_int,
    -(188 as core::ffi::c_int),
    -(1000 as core::ffi::c_int),
    -(256 as core::ffi::c_int),
    92 as core::ffi::c_int,
    1164 as core::ffi::c_int,
    -(392 as core::ffi::c_int),
    564 as core::ffi::c_int,
    356 as core::ffi::c_int,
    652 as core::ffi::c_int,
    -(28 as core::ffi::c_int),
    -(884 as core::ffi::c_int),
    256 as core::ffi::c_int,
    484 as core::ffi::c_int,
    -(192 as core::ffi::c_int),
    760 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    376 as core::ffi::c_int,
    -(524 as core::ffi::c_int),
    -(452 as core::ffi::c_int),
    -(436 as core::ffi::c_int),
    860 as core::ffi::c_int,
    -(736 as core::ffi::c_int),
    212 as core::ffi::c_int,
    124 as core::ffi::c_int,
    504 as core::ffi::c_int,
    -(476 as core::ffi::c_int),
    468 as core::ffi::c_int,
    76 as core::ffi::c_int,
    -(472 as core::ffi::c_int),
    552 as core::ffi::c_int,
    -(692 as core::ffi::c_int),
    -(944 as core::ffi::c_int),
    -(620 as core::ffi::c_int),
    740 as core::ffi::c_int,
    -(240 as core::ffi::c_int),
    400 as core::ffi::c_int,
    132 as core::ffi::c_int,
    20 as core::ffi::c_int,
    192 as core::ffi::c_int,
    -(196 as core::ffi::c_int),
    264 as core::ffi::c_int,
    -(668 as core::ffi::c_int),
    -(1012 as core::ffi::c_int),
    -(60 as core::ffi::c_int),
    296 as core::ffi::c_int,
    -(316 as core::ffi::c_int),
    -(828 as core::ffi::c_int),
    76 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    284 as core::ffi::c_int,
    -(768 as core::ffi::c_int),
    -(448 as core::ffi::c_int),
    -(832 as core::ffi::c_int),
    148 as core::ffi::c_int,
    248 as core::ffi::c_int,
    652 as core::ffi::c_int,
    616 as core::ffi::c_int,
    1236 as core::ffi::c_int,
    288 as core::ffi::c_int,
    -(328 as core::ffi::c_int),
    -(400 as core::ffi::c_int),
    -(124 as core::ffi::c_int),
    588 as core::ffi::c_int,
    220 as core::ffi::c_int,
    520 as core::ffi::c_int,
    -(696 as core::ffi::c_int),
    1032 as core::ffi::c_int,
    768 as core::ffi::c_int,
    -(740 as core::ffi::c_int),
    -(92 as core::ffi::c_int),
    -(272 as core::ffi::c_int),
    296 as core::ffi::c_int,
    448 as core::ffi::c_int,
    -(464 as core::ffi::c_int),
    412 as core::ffi::c_int,
    -(200 as core::ffi::c_int),
    392 as core::ffi::c_int,
    440 as core::ffi::c_int,
    -(200 as core::ffi::c_int),
    264 as core::ffi::c_int,
    -(152 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    320 as core::ffi::c_int,
    1032 as core::ffi::c_int,
    216 as core::ffi::c_int,
    320 as core::ffi::c_int,
    -(8 as core::ffi::c_int),
    -(64 as core::ffi::c_int),
    156 as core::ffi::c_int,
    -(1016 as core::ffi::c_int),
    1084 as core::ffi::c_int,
    1172 as core::ffi::c_int,
    536 as core::ffi::c_int,
    484 as core::ffi::c_int,
    -(432 as core::ffi::c_int),
    132 as core::ffi::c_int,
    372 as core::ffi::c_int,
    -(52 as core::ffi::c_int),
    -(256 as core::ffi::c_int),
    84 as core::ffi::c_int,
    116 as core::ffi::c_int,
    -(352 as core::ffi::c_int),
    48 as core::ffi::c_int,
    116 as core::ffi::c_int,
    304 as core::ffi::c_int,
    -(384 as core::ffi::c_int),
    412 as core::ffi::c_int,
    924 as core::ffi::c_int,
    -(300 as core::ffi::c_int),
    528 as core::ffi::c_int,
    628 as core::ffi::c_int,
    180 as core::ffi::c_int,
    648 as core::ffi::c_int,
    44 as core::ffi::c_int,
    -(980 as core::ffi::c_int),
    -(220 as core::ffi::c_int),
    1320 as core::ffi::c_int,
    48 as core::ffi::c_int,
    332 as core::ffi::c_int,
    748 as core::ffi::c_int,
    524 as core::ffi::c_int,
    -(268 as core::ffi::c_int),
    -(720 as core::ffi::c_int),
    540 as core::ffi::c_int,
    -(276 as core::ffi::c_int),
    564 as core::ffi::c_int,
    -(344 as core::ffi::c_int),
    -(208 as core::ffi::c_int),
    -(196 as core::ffi::c_int),
    436 as core::ffi::c_int,
    896 as core::ffi::c_int,
    88 as core::ffi::c_int,
    -(392 as core::ffi::c_int),
    132 as core::ffi::c_int,
    80 as core::ffi::c_int,
    -(964 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    568 as core::ffi::c_int,
    56 as core::ffi::c_int,
    -(48 as core::ffi::c_int),
    -(456 as core::ffi::c_int),
    888 as core::ffi::c_int,
    8 as core::ffi::c_int,
    552 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    -(292 as core::ffi::c_int),
    948 as core::ffi::c_int,
    288 as core::ffi::c_int,
    128 as core::ffi::c_int,
    -(716 as core::ffi::c_int),
    -(292 as core::ffi::c_int),
    1192 as core::ffi::c_int,
    -(152 as core::ffi::c_int),
    876 as core::ffi::c_int,
    352 as core::ffi::c_int,
    -(600 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    -(812 as core::ffi::c_int),
    -(468 as core::ffi::c_int),
    -(28 as core::ffi::c_int),
    -(120 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    -(44 as core::ffi::c_int),
    1284 as core::ffi::c_int,
    496 as core::ffi::c_int,
    192 as core::ffi::c_int,
    464 as core::ffi::c_int,
    312 as core::ffi::c_int,
    -(76 as core::ffi::c_int),
    -(516 as core::ffi::c_int),
    -(380 as core::ffi::c_int),
    -(456 as core::ffi::c_int),
    -(1012 as core::ffi::c_int),
    -(48 as core::ffi::c_int),
    308 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    36 as core::ffi::c_int,
    492 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    -(808 as core::ffi::c_int),
    188 as core::ffi::c_int,
    1652 as core::ffi::c_int,
    68 as core::ffi::c_int,
    -(120 as core::ffi::c_int),
    -(116 as core::ffi::c_int),
    316 as core::ffi::c_int,
    160 as core::ffi::c_int,
    -(140 as core::ffi::c_int),
    352 as core::ffi::c_int,
    808 as core::ffi::c_int,
    -(416 as core::ffi::c_int),
    592 as core::ffi::c_int,
    316 as core::ffi::c_int,
    -(480 as core::ffi::c_int),
    56 as core::ffi::c_int,
    528 as core::ffi::c_int,
    -(204 as core::ffi::c_int),
    -(568 as core::ffi::c_int),
    372 as core::ffi::c_int,
    -(232 as core::ffi::c_int),
    752 as core::ffi::c_int,
    -(344 as core::ffi::c_int),
    744 as core::ffi::c_int,
    -(4 as core::ffi::c_int),
    324 as core::ffi::c_int,
    -(416 as core::ffi::c_int),
    -(600 as core::ffi::c_int),
    768 as core::ffi::c_int,
    268 as core::ffi::c_int,
    -(248 as core::ffi::c_int),
    -(88 as core::ffi::c_int),
    -(132 as core::ffi::c_int),
    -(420 as core::ffi::c_int),
    -(432 as core::ffi::c_int),
    80 as core::ffi::c_int,
    -(288 as core::ffi::c_int),
    404 as core::ffi::c_int,
    -(316 as core::ffi::c_int),
    -(1216 as core::ffi::c_int),
    -(588 as core::ffi::c_int),
    520 as core::ffi::c_int,
    -(108 as core::ffi::c_int),
    92 as core::ffi::c_int,
    -(320 as core::ffi::c_int),
    368 as core::ffi::c_int,
    -(480 as core::ffi::c_int),
    -(216 as core::ffi::c_int),
    -(92 as core::ffi::c_int),
    1688 as core::ffi::c_int,
    -(300 as core::ffi::c_int),
    180 as core::ffi::c_int,
    1020 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    820 as core::ffi::c_int,
    -(68 as core::ffi::c_int),
    -(228 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    436 as core::ffi::c_int,
    -(904 as core::ffi::c_int),
    20 as core::ffi::c_int,
    40 as core::ffi::c_int,
    -(508 as core::ffi::c_int),
    440 as core::ffi::c_int,
    -(736 as core::ffi::c_int),
    312 as core::ffi::c_int,
    332 as core::ffi::c_int,
    204 as core::ffi::c_int,
    760 as core::ffi::c_int,
    -(372 as core::ffi::c_int),
    728 as core::ffi::c_int,
    96 as core::ffi::c_int,
    -(20 as core::ffi::c_int),
    -(632 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    -(560 as core::ffi::c_int),
    336 as core::ffi::c_int,
    1076 as core::ffi::c_int,
    -(64 as core::ffi::c_int),
    -(532 as core::ffi::c_int),
    776 as core::ffi::c_int,
    584 as core::ffi::c_int,
    192 as core::ffi::c_int,
    396 as core::ffi::c_int,
    -(728 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    276 as core::ffi::c_int,
    -(188 as core::ffi::c_int),
    80 as core::ffi::c_int,
    -(52 as core::ffi::c_int),
    -(612 as core::ffi::c_int),
    -(252 as core::ffi::c_int),
    -(48 as core::ffi::c_int),
    648 as core::ffi::c_int,
    212 as core::ffi::c_int,
    -(688 as core::ffi::c_int),
    228 as core::ffi::c_int,
    -(52 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    428 as core::ffi::c_int,
    -(412 as core::ffi::c_int),
    -(272 as core::ffi::c_int),
    -(404 as core::ffi::c_int),
    180 as core::ffi::c_int,
    816 as core::ffi::c_int,
    -(796 as core::ffi::c_int),
    48 as core::ffi::c_int,
    152 as core::ffi::c_int,
    484 as core::ffi::c_int,
    -(88 as core::ffi::c_int),
    -(216 as core::ffi::c_int),
    988 as core::ffi::c_int,
    696 as core::ffi::c_int,
    188 as core::ffi::c_int,
    -(528 as core::ffi::c_int),
    648 as core::ffi::c_int,
    -(116 as core::ffi::c_int),
    -(180 as core::ffi::c_int),
    316 as core::ffi::c_int,
    476 as core::ffi::c_int,
    12 as core::ffi::c_int,
    -(564 as core::ffi::c_int),
    96 as core::ffi::c_int,
    476 as core::ffi::c_int,
    -(252 as core::ffi::c_int),
    -(364 as core::ffi::c_int),
    -(376 as core::ffi::c_int),
    -(392 as core::ffi::c_int),
    556 as core::ffi::c_int,
    -(256 as core::ffi::c_int),
    -(576 as core::ffi::c_int),
    260 as core::ffi::c_int,
    -(352 as core::ffi::c_int),
    120 as core::ffi::c_int,
    -(16 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    -(492 as core::ffi::c_int),
    72 as core::ffi::c_int,
    556 as core::ffi::c_int,
    660 as core::ffi::c_int,
    580 as core::ffi::c_int,
    616 as core::ffi::c_int,
    772 as core::ffi::c_int,
    436 as core::ffi::c_int,
    424 as core::ffi::c_int,
    -(32 as core::ffi::c_int),
    -(324 as core::ffi::c_int),
    -(1268 as core::ffi::c_int),
    416 as core::ffi::c_int,
    -(324 as core::ffi::c_int),
    -(80 as core::ffi::c_int),
    920 as core::ffi::c_int,
    160 as core::ffi::c_int,
    228 as core::ffi::c_int,
    724 as core::ffi::c_int,
    32 as core::ffi::c_int,
    -(516 as core::ffi::c_int),
    64 as core::ffi::c_int,
    384 as core::ffi::c_int,
    68 as core::ffi::c_int,
    -(128 as core::ffi::c_int),
    136 as core::ffi::c_int,
    240 as core::ffi::c_int,
    248 as core::ffi::c_int,
    -(204 as core::ffi::c_int),
    -(68 as core::ffi::c_int),
    252 as core::ffi::c_int,
    -(932 as core::ffi::c_int),
    -(120 as core::ffi::c_int),
    -(480 as core::ffi::c_int),
    -(628 as core::ffi::c_int),
    -(84 as core::ffi::c_int),
    192 as core::ffi::c_int,
    852 as core::ffi::c_int,
    -(404 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    -(132 as core::ffi::c_int),
    204 as core::ffi::c_int,
    100 as core::ffi::c_int,
    168 as core::ffi::c_int,
    -(68 as core::ffi::c_int),
    -(196 as core::ffi::c_int),
    -(868 as core::ffi::c_int),
    460 as core::ffi::c_int,
    1080 as core::ffi::c_int,
    380 as core::ffi::c_int,
    -(80 as core::ffi::c_int),
    244 as core::ffi::c_int,
    0 as core::ffi::c_int,
    484 as core::ffi::c_int,
    -(888 as core::ffi::c_int),
    64 as core::ffi::c_int,
    184 as core::ffi::c_int,
    352 as core::ffi::c_int,
    600 as core::ffi::c_int,
    460 as core::ffi::c_int,
    164 as core::ffi::c_int,
    604 as core::ffi::c_int,
    -(196 as core::ffi::c_int),
    320 as core::ffi::c_int,
    -(64 as core::ffi::c_int),
    588 as core::ffi::c_int,
    -(184 as core::ffi::c_int),
    228 as core::ffi::c_int,
    12 as core::ffi::c_int,
    372 as core::ffi::c_int,
    48 as core::ffi::c_int,
    -(848 as core::ffi::c_int),
    -(344 as core::ffi::c_int),
    224 as core::ffi::c_int,
    208 as core::ffi::c_int,
    -(200 as core::ffi::c_int),
    484 as core::ffi::c_int,
    128 as core::ffi::c_int,
    -(20 as core::ffi::c_int),
    272 as core::ffi::c_int,
    -(468 as core::ffi::c_int),
    -(840 as core::ffi::c_int),
    384 as core::ffi::c_int,
    256 as core::ffi::c_int,
    -(720 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    -(464 as core::ffi::c_int),
    -(580 as core::ffi::c_int),
    112 as core::ffi::c_int,
    -(120 as core::ffi::c_int),
    644 as core::ffi::c_int,
    -(356 as core::ffi::c_int),
    -(208 as core::ffi::c_int),
    -(608 as core::ffi::c_int),
    -(528 as core::ffi::c_int),
    704 as core::ffi::c_int,
    560 as core::ffi::c_int,
    -(424 as core::ffi::c_int),
    392 as core::ffi::c_int,
    828 as core::ffi::c_int,
    40 as core::ffi::c_int,
    84 as core::ffi::c_int,
    200 as core::ffi::c_int,
    -(152 as core::ffi::c_int),
    0 as core::ffi::c_int,
    -(144 as core::ffi::c_int),
    584 as core::ffi::c_int,
    280 as core::ffi::c_int,
    -(120 as core::ffi::c_int),
    80 as core::ffi::c_int,
    -(556 as core::ffi::c_int),
    -(972 as core::ffi::c_int),
    -(196 as core::ffi::c_int),
    -(472 as core::ffi::c_int),
    724 as core::ffi::c_int,
    80 as core::ffi::c_int,
    168 as core::ffi::c_int,
    -(32 as core::ffi::c_int),
    88 as core::ffi::c_int,
    160 as core::ffi::c_int,
    -(688 as core::ffi::c_int),
    0 as core::ffi::c_int,
    160 as core::ffi::c_int,
    356 as core::ffi::c_int,
    372 as core::ffi::c_int,
    -(776 as core::ffi::c_int),
    740 as core::ffi::c_int,
    -(128 as core::ffi::c_int),
    676 as core::ffi::c_int,
    -(248 as core::ffi::c_int),
    -(480 as core::ffi::c_int),
    4 as core::ffi::c_int,
    -(364 as core::ffi::c_int),
    96 as core::ffi::c_int,
    544 as core::ffi::c_int,
    232 as core::ffi::c_int,
    -(1032 as core::ffi::c_int),
    956 as core::ffi::c_int,
    236 as core::ffi::c_int,
    356 as core::ffi::c_int,
    20 as core::ffi::c_int,
    -(40 as core::ffi::c_int),
    300 as core::ffi::c_int,
    24 as core::ffi::c_int,
    -(676 as core::ffi::c_int),
    -(596 as core::ffi::c_int),
    132 as core::ffi::c_int,
    1120 as core::ffi::c_int,
    -(104 as core::ffi::c_int),
    532 as core::ffi::c_int,
    -(1096 as core::ffi::c_int),
    568 as core::ffi::c_int,
    648 as core::ffi::c_int,
    444 as core::ffi::c_int,
    508 as core::ffi::c_int,
    380 as core::ffi::c_int,
    188 as core::ffi::c_int,
    -(376 as core::ffi::c_int),
    -(604 as core::ffi::c_int),
    1488 as core::ffi::c_int,
    424 as core::ffi::c_int,
    24 as core::ffi::c_int,
    756 as core::ffi::c_int,
    -(220 as core::ffi::c_int),
    -(192 as core::ffi::c_int),
    716 as core::ffi::c_int,
    120 as core::ffi::c_int,
    920 as core::ffi::c_int,
    688 as core::ffi::c_int,
    168 as core::ffi::c_int,
    44 as core::ffi::c_int,
    -(460 as core::ffi::c_int),
    568 as core::ffi::c_int,
    284 as core::ffi::c_int,
    1144 as core::ffi::c_int,
    1160 as core::ffi::c_int,
    600 as core::ffi::c_int,
    424 as core::ffi::c_int,
    888 as core::ffi::c_int,
    656 as core::ffi::c_int,
    -(356 as core::ffi::c_int),
    -(320 as core::ffi::c_int),
    220 as core::ffi::c_int,
    316 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    -(724 as core::ffi::c_int),
    -(188 as core::ffi::c_int),
    -(816 as core::ffi::c_int),
    -(628 as core::ffi::c_int),
    -(348 as core::ffi::c_int),
    -(228 as core::ffi::c_int),
    -(380 as core::ffi::c_int),
    1012 as core::ffi::c_int,
    -(452 as core::ffi::c_int),
    -(660 as core::ffi::c_int),
    736 as core::ffi::c_int,
    928 as core::ffi::c_int,
    404 as core::ffi::c_int,
    -(696 as core::ffi::c_int),
    -(72 as core::ffi::c_int),
    -(268 as core::ffi::c_int),
    -(892 as core::ffi::c_int),
    128 as core::ffi::c_int,
    184 as core::ffi::c_int,
    -(344 as core::ffi::c_int),
    -(780 as core::ffi::c_int),
    360 as core::ffi::c_int,
    336 as core::ffi::c_int,
    400 as core::ffi::c_int,
    344 as core::ffi::c_int,
    428 as core::ffi::c_int,
    548 as core::ffi::c_int,
    -(112 as core::ffi::c_int),
    136 as core::ffi::c_int,
    -(228 as core::ffi::c_int),
    -(216 as core::ffi::c_int),
    -(820 as core::ffi::c_int),
    -(516 as core::ffi::c_int),
    340 as core::ffi::c_int,
    92 as core::ffi::c_int,
    -(136 as core::ffi::c_int),
    116 as core::ffi::c_int,
    -(300 as core::ffi::c_int),
    376 as core::ffi::c_int,
    -(244 as core::ffi::c_int),
    100 as core::ffi::c_int,
    -(316 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    -(284 as core::ffi::c_int),
    -(12 as core::ffi::c_int),
    824 as core::ffi::c_int,
    164 as core::ffi::c_int,
    -(548 as core::ffi::c_int),
    -(180 as core::ffi::c_int),
    -(128 as core::ffi::c_int),
    116 as core::ffi::c_int,
    -(924 as core::ffi::c_int),
    -(828 as core::ffi::c_int),
    268 as core::ffi::c_int,
    -(368 as core::ffi::c_int),
    -(580 as core::ffi::c_int),
    620 as core::ffi::c_int,
    192 as core::ffi::c_int,
    160 as core::ffi::c_int,
    0 as core::ffi::c_int,
    -(1676 as core::ffi::c_int),
    1068 as core::ffi::c_int,
    424 as core::ffi::c_int,
    -(56 as core::ffi::c_int),
    -(360 as core::ffi::c_int),
    468 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    720 as core::ffi::c_int,
    288 as core::ffi::c_int,
    -(528 as core::ffi::c_int),
    556 as core::ffi::c_int,
    -(364 as core::ffi::c_int),
    548 as core::ffi::c_int,
    -(148 as core::ffi::c_int),
    504 as core::ffi::c_int,
    316 as core::ffi::c_int,
    152 as core::ffi::c_int,
    -(648 as core::ffi::c_int),
    -(620 as core::ffi::c_int),
    -(684 as core::ffi::c_int),
    -(24 as core::ffi::c_int),
    -(376 as core::ffi::c_int),
    -(384 as core::ffi::c_int),
    -(108 as core::ffi::c_int),
    -(920 as core::ffi::c_int),
    -(1032 as core::ffi::c_int),
    768 as core::ffi::c_int,
    180 as core::ffi::c_int,
    -(264 as core::ffi::c_int),
    -(508 as core::ffi::c_int),
    -(1268 as core::ffi::c_int),
    -(260 as core::ffi::c_int),
    -(60 as core::ffi::c_int),
    300 as core::ffi::c_int,
    -(240 as core::ffi::c_int),
    988 as core::ffi::c_int,
    724 as core::ffi::c_int,
    -(376 as core::ffi::c_int),
    -(576 as core::ffi::c_int),
    -(212 as core::ffi::c_int),
    -(736 as core::ffi::c_int),
    556 as core::ffi::c_int,
    192 as core::ffi::c_int,
    1092 as core::ffi::c_int,
    -(620 as core::ffi::c_int),
    -(880 as core::ffi::c_int),
    376 as core::ffi::c_int,
    -(56 as core::ffi::c_int),
    -(4 as core::ffi::c_int),
    -(216 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    836 as core::ffi::c_int,
    268 as core::ffi::c_int,
    396 as core::ffi::c_int,
    1332 as core::ffi::c_int,
    864 as core::ffi::c_int,
    -(600 as core::ffi::c_int),
    100 as core::ffi::c_int,
    56 as core::ffi::c_int,
    -(412 as core::ffi::c_int),
    -(92 as core::ffi::c_int),
    356 as core::ffi::c_int,
    180 as core::ffi::c_int,
    884 as core::ffi::c_int,
    -(468 as core::ffi::c_int),
    -(436 as core::ffi::c_int),
    292 as core::ffi::c_int,
    -(388 as core::ffi::c_int),
    -(804 as core::ffi::c_int),
    -(704 as core::ffi::c_int),
    -(840 as core::ffi::c_int),
    368 as core::ffi::c_int,
    -(348 as core::ffi::c_int),
    140 as core::ffi::c_int,
    -(724 as core::ffi::c_int),
    1536 as core::ffi::c_int,
    940 as core::ffi::c_int,
    372 as core::ffi::c_int,
    112 as core::ffi::c_int,
    -(372 as core::ffi::c_int),
    436 as core::ffi::c_int,
    -(480 as core::ffi::c_int),
    1136 as core::ffi::c_int,
    296 as core::ffi::c_int,
    -(32 as core::ffi::c_int),
    -(228 as core::ffi::c_int),
    132 as core::ffi::c_int,
    -(48 as core::ffi::c_int),
    -(220 as core::ffi::c_int),
    868 as core::ffi::c_int,
    -(1016 as core::ffi::c_int),
    -(60 as core::ffi::c_int),
    -(1044 as core::ffi::c_int),
    -(464 as core::ffi::c_int),
    328 as core::ffi::c_int,
    916 as core::ffi::c_int,
    244 as core::ffi::c_int,
    12 as core::ffi::c_int,
    -(736 as core::ffi::c_int),
    -(296 as core::ffi::c_int),
    360 as core::ffi::c_int,
    468 as core::ffi::c_int,
    -(376 as core::ffi::c_int),
    -(108 as core::ffi::c_int),
    -(92 as core::ffi::c_int),
    788 as core::ffi::c_int,
    368 as core::ffi::c_int,
    -(56 as core::ffi::c_int),
    544 as core::ffi::c_int,
    400 as core::ffi::c_int,
    -(672 as core::ffi::c_int),
    -(420 as core::ffi::c_int),
    728 as core::ffi::c_int,
    16 as core::ffi::c_int,
    320 as core::ffi::c_int,
    44 as core::ffi::c_int,
    -(284 as core::ffi::c_int),
    -(380 as core::ffi::c_int),
    -(796 as core::ffi::c_int),
    488 as core::ffi::c_int,
    132 as core::ffi::c_int,
    204 as core::ffi::c_int,
    -(596 as core::ffi::c_int),
    -(372 as core::ffi::c_int),
    88 as core::ffi::c_int,
    -(152 as core::ffi::c_int),
    -(908 as core::ffi::c_int),
    -(636 as core::ffi::c_int),
    -(572 as core::ffi::c_int),
    -(624 as core::ffi::c_int),
    -(116 as core::ffi::c_int),
    -(692 as core::ffi::c_int),
    -(200 as core::ffi::c_int),
    -(56 as core::ffi::c_int),
    276 as core::ffi::c_int,
    -(88 as core::ffi::c_int),
    484 as core::ffi::c_int,
    -(324 as core::ffi::c_int),
    948 as core::ffi::c_int,
    864 as core::ffi::c_int,
    1000 as core::ffi::c_int,
    -(456 as core::ffi::c_int),
    -(184 as core::ffi::c_int),
    -(276 as core::ffi::c_int),
    292 as core::ffi::c_int,
    -(296 as core::ffi::c_int),
    156 as core::ffi::c_int,
    676 as core::ffi::c_int,
    320 as core::ffi::c_int,
    160 as core::ffi::c_int,
    908 as core::ffi::c_int,
    -(84 as core::ffi::c_int),
    -(1236 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    -(116 as core::ffi::c_int),
    260 as core::ffi::c_int,
    -(372 as core::ffi::c_int),
    -(644 as core::ffi::c_int),
    732 as core::ffi::c_int,
    -(756 as core::ffi::c_int),
    -(96 as core::ffi::c_int),
    84 as core::ffi::c_int,
    344 as core::ffi::c_int,
    -(520 as core::ffi::c_int),
    348 as core::ffi::c_int,
    -(688 as core::ffi::c_int),
    240 as core::ffi::c_int,
    -(84 as core::ffi::c_int),
    216 as core::ffi::c_int,
    -(1044 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    -(676 as core::ffi::c_int),
    -(396 as core::ffi::c_int),
    -(1500 as core::ffi::c_int),
    960 as core::ffi::c_int,
    -(40 as core::ffi::c_int),
    176 as core::ffi::c_int,
    168 as core::ffi::c_int,
    1516 as core::ffi::c_int,
    420 as core::ffi::c_int,
    -(504 as core::ffi::c_int),
    -(344 as core::ffi::c_int),
    -(364 as core::ffi::c_int),
    -(360 as core::ffi::c_int),
    1216 as core::ffi::c_int,
    -(940 as core::ffi::c_int),
    -(380 as core::ffi::c_int),
    -(212 as core::ffi::c_int),
    252 as core::ffi::c_int,
    -(660 as core::ffi::c_int),
    -(708 as core::ffi::c_int),
    484 as core::ffi::c_int,
    -(444 as core::ffi::c_int),
    -(152 as core::ffi::c_int),
    928 as core::ffi::c_int,
    -(120 as core::ffi::c_int),
    1112 as core::ffi::c_int,
    476 as core::ffi::c_int,
    -(260 as core::ffi::c_int),
    560 as core::ffi::c_int,
    -(148 as core::ffi::c_int),
    -(344 as core::ffi::c_int),
    108 as core::ffi::c_int,
    -(196 as core::ffi::c_int),
    228 as core::ffi::c_int,
    -(288 as core::ffi::c_int),
    504 as core::ffi::c_int,
    560 as core::ffi::c_int,
    -(328 as core::ffi::c_int),
    -(88 as core::ffi::c_int),
    288 as core::ffi::c_int,
    -(1008 as core::ffi::c_int),
    460 as core::ffi::c_int,
    -(228 as core::ffi::c_int),
    468 as core::ffi::c_int,
    -(836 as core::ffi::c_int),
    -(196 as core::ffi::c_int),
    76 as core::ffi::c_int,
    388 as core::ffi::c_int,
    232 as core::ffi::c_int,
    412 as core::ffi::c_int,
    -(1168 as core::ffi::c_int),
    -(716 as core::ffi::c_int),
    -(644 as core::ffi::c_int),
    756 as core::ffi::c_int,
    -(172 as core::ffi::c_int),
    -(356 as core::ffi::c_int),
    -(504 as core::ffi::c_int),
    116 as core::ffi::c_int,
    432 as core::ffi::c_int,
    528 as core::ffi::c_int,
    48 as core::ffi::c_int,
    476 as core::ffi::c_int,
    -(168 as core::ffi::c_int),
    -(608 as core::ffi::c_int),
    448 as core::ffi::c_int,
    160 as core::ffi::c_int,
    -(532 as core::ffi::c_int),
    -(272 as core::ffi::c_int),
    28 as core::ffi::c_int,
    -(676 as core::ffi::c_int),
    -(12 as core::ffi::c_int),
    828 as core::ffi::c_int,
    980 as core::ffi::c_int,
    456 as core::ffi::c_int,
    520 as core::ffi::c_int,
    104 as core::ffi::c_int,
    -(104 as core::ffi::c_int),
    256 as core::ffi::c_int,
    -(344 as core::ffi::c_int),
    -(4 as core::ffi::c_int),
    -(28 as core::ffi::c_int),
    -(368 as core::ffi::c_int),
    -(52 as core::ffi::c_int),
    -(524 as core::ffi::c_int),
    -(572 as core::ffi::c_int),
    -(556 as core::ffi::c_int),
    -(200 as core::ffi::c_int),
    768 as core::ffi::c_int,
    1124 as core::ffi::c_int,
    -(208 as core::ffi::c_int),
    -(512 as core::ffi::c_int),
    176 as core::ffi::c_int,
    232 as core::ffi::c_int,
    248 as core::ffi::c_int,
    -(148 as core::ffi::c_int),
    -(888 as core::ffi::c_int),
    604 as core::ffi::c_int,
    -(600 as core::ffi::c_int),
    -(304 as core::ffi::c_int),
    804 as core::ffi::c_int,
    -(156 as core::ffi::c_int),
    -(212 as core::ffi::c_int),
    488 as core::ffi::c_int,
    -(192 as core::ffi::c_int),
    -(804 as core::ffi::c_int),
    -(256 as core::ffi::c_int),
    368 as core::ffi::c_int,
    -(360 as core::ffi::c_int),
    -(916 as core::ffi::c_int),
    -(328 as core::ffi::c_int),
    228 as core::ffi::c_int,
    -(240 as core::ffi::c_int),
    -(448 as core::ffi::c_int),
    -(472 as core::ffi::c_int),
    856 as core::ffi::c_int,
    -(556 as core::ffi::c_int),
    -(364 as core::ffi::c_int),
    572 as core::ffi::c_int,
    -(12 as core::ffi::c_int),
    -(156 as core::ffi::c_int),
    -(368 as core::ffi::c_int),
    -(340 as core::ffi::c_int),
    432 as core::ffi::c_int,
    252 as core::ffi::c_int,
    -(752 as core::ffi::c_int),
    -(152 as core::ffi::c_int),
    288 as core::ffi::c_int,
    268 as core::ffi::c_int,
    -(580 as core::ffi::c_int),
    -(848 as core::ffi::c_int),
    -(592 as core::ffi::c_int),
    108 as core::ffi::c_int,
    -(76 as core::ffi::c_int),
    244 as core::ffi::c_int,
    312 as core::ffi::c_int,
    -(716 as core::ffi::c_int),
    592 as core::ffi::c_int,
    -(80 as core::ffi::c_int),
    436 as core::ffi::c_int,
    360 as core::ffi::c_int,
    4 as core::ffi::c_int,
    -(248 as core::ffi::c_int),
    160 as core::ffi::c_int,
    516 as core::ffi::c_int,
    584 as core::ffi::c_int,
    732 as core::ffi::c_int,
    44 as core::ffi::c_int,
    -(468 as core::ffi::c_int),
    -(280 as core::ffi::c_int),
    -(292 as core::ffi::c_int),
    -(156 as core::ffi::c_int),
    -(588 as core::ffi::c_int),
    28 as core::ffi::c_int,
    308 as core::ffi::c_int,
    912 as core::ffi::c_int,
    24 as core::ffi::c_int,
    124 as core::ffi::c_int,
    156 as core::ffi::c_int,
    180 as core::ffi::c_int,
    -(252 as core::ffi::c_int),
    944 as core::ffi::c_int,
    -(924 as core::ffi::c_int),
    -(772 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    -(428 as core::ffi::c_int),
    -(624 as core::ffi::c_int),
    300 as core::ffi::c_int,
    -(212 as core::ffi::c_int),
    -(1144 as core::ffi::c_int),
    32 as core::ffi::c_int,
    -(724 as core::ffi::c_int),
    800 as core::ffi::c_int,
    -(1128 as core::ffi::c_int),
    -(212 as core::ffi::c_int),
    -(1288 as core::ffi::c_int),
    -(848 as core::ffi::c_int),
    180 as core::ffi::c_int,
    -(416 as core::ffi::c_int),
    440 as core::ffi::c_int,
    192 as core::ffi::c_int,
    -(576 as core::ffi::c_int),
    -(792 as core::ffi::c_int),
    -(76 as core::ffi::c_int),
    -(1080 as core::ffi::c_int),
    80 as core::ffi::c_int,
    -(532 as core::ffi::c_int),
    -(352 as core::ffi::c_int),
    -(132 as core::ffi::c_int),
    380 as core::ffi::c_int,
    -(820 as core::ffi::c_int),
    148 as core::ffi::c_int,
    1112 as core::ffi::c_int,
    128 as core::ffi::c_int,
    164 as core::ffi::c_int,
    456 as core::ffi::c_int,
    700 as core::ffi::c_int,
    -(924 as core::ffi::c_int),
    144 as core::ffi::c_int,
    -(668 as core::ffi::c_int),
    -(384 as core::ffi::c_int),
    648 as core::ffi::c_int,
    -(832 as core::ffi::c_int),
    508 as core::ffi::c_int,
    552 as core::ffi::c_int,
    -(52 as core::ffi::c_int),
    -(100 as core::ffi::c_int),
    -(656 as core::ffi::c_int),
    208 as core::ffi::c_int,
    -(568 as core::ffi::c_int),
    748 as core::ffi::c_int,
    -(88 as core::ffi::c_int),
    680 as core::ffi::c_int,
    232 as core::ffi::c_int,
    300 as core::ffi::c_int,
    192 as core::ffi::c_int,
    -(408 as core::ffi::c_int),
    -(1012 as core::ffi::c_int),
    -(152 as core::ffi::c_int),
    -(252 as core::ffi::c_int),
    -(268 as core::ffi::c_int),
    272 as core::ffi::c_int,
    -(876 as core::ffi::c_int),
    -(664 as core::ffi::c_int),
    -(648 as core::ffi::c_int),
    -(332 as core::ffi::c_int),
    -(136 as core::ffi::c_int),
    16 as core::ffi::c_int,
    12 as core::ffi::c_int,
    1152 as core::ffi::c_int,
    -(28 as core::ffi::c_int),
    332 as core::ffi::c_int,
    -(536 as core::ffi::c_int),
    320 as core::ffi::c_int,
    -(672 as core::ffi::c_int),
    -(460 as core::ffi::c_int),
    -(316 as core::ffi::c_int),
    532 as core::ffi::c_int,
    -(260 as core::ffi::c_int),
    228 as core::ffi::c_int,
    -(40 as core::ffi::c_int),
    1052 as core::ffi::c_int,
    -(816 as core::ffi::c_int),
    180 as core::ffi::c_int,
    88 as core::ffi::c_int,
    -(496 as core::ffi::c_int),
    -(556 as core::ffi::c_int),
    -(672 as core::ffi::c_int),
    -(368 as core::ffi::c_int),
    428 as core::ffi::c_int,
    92 as core::ffi::c_int,
    356 as core::ffi::c_int,
    404 as core::ffi::c_int,
    -(408 as core::ffi::c_int),
    252 as core::ffi::c_int,
    196 as core::ffi::c_int,
    -(176 as core::ffi::c_int),
    -(556 as core::ffi::c_int),
    792 as core::ffi::c_int,
    268 as core::ffi::c_int,
    32 as core::ffi::c_int,
    372 as core::ffi::c_int,
    40 as core::ffi::c_int,
    96 as core::ffi::c_int,
    -(332 as core::ffi::c_int),
    328 as core::ffi::c_int,
    120 as core::ffi::c_int,
    372 as core::ffi::c_int,
    -(900 as core::ffi::c_int),
    -(40 as core::ffi::c_int),
    472 as core::ffi::c_int,
    -(264 as core::ffi::c_int),
    -(592 as core::ffi::c_int),
    952 as core::ffi::c_int,
    128 as core::ffi::c_int,
    656 as core::ffi::c_int,
    112 as core::ffi::c_int,
    664 as core::ffi::c_int,
    -(232 as core::ffi::c_int),
    420 as core::ffi::c_int,
    4 as core::ffi::c_int,
    -(344 as core::ffi::c_int),
    -(464 as core::ffi::c_int),
    556 as core::ffi::c_int,
    244 as core::ffi::c_int,
    -(416 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    252 as core::ffi::c_int,
    0 as core::ffi::c_int,
    -(412 as core::ffi::c_int),
    188 as core::ffi::c_int,
    -(696 as core::ffi::c_int),
    508 as core::ffi::c_int,
    -(476 as core::ffi::c_int),
    324 as core::ffi::c_int,
    -(1096 as core::ffi::c_int),
    656 as core::ffi::c_int,
    -(312 as core::ffi::c_int),
    560 as core::ffi::c_int,
    264 as core::ffi::c_int,
    -(136 as core::ffi::c_int),
    304 as core::ffi::c_int,
    160 as core::ffi::c_int,
    -(64 as core::ffi::c_int),
    -(580 as core::ffi::c_int),
    248 as core::ffi::c_int,
    336 as core::ffi::c_int,
    -(720 as core::ffi::c_int),
    560 as core::ffi::c_int,
    -(348 as core::ffi::c_int),
    -(288 as core::ffi::c_int),
    -(276 as core::ffi::c_int),
    -(196 as core::ffi::c_int),
    -(500 as core::ffi::c_int),
    852 as core::ffi::c_int,
    -(544 as core::ffi::c_int),
    -(236 as core::ffi::c_int),
    -(1128 as core::ffi::c_int),
    -(992 as core::ffi::c_int),
    -(776 as core::ffi::c_int),
    116 as core::ffi::c_int,
    56 as core::ffi::c_int,
    52 as core::ffi::c_int,
    860 as core::ffi::c_int,
    884 as core::ffi::c_int,
    212 as core::ffi::c_int,
    -(12 as core::ffi::c_int),
    168 as core::ffi::c_int,
    1020 as core::ffi::c_int,
    512 as core::ffi::c_int,
    -(552 as core::ffi::c_int),
    924 as core::ffi::c_int,
    -(148 as core::ffi::c_int),
    716 as core::ffi::c_int,
    188 as core::ffi::c_int,
    164 as core::ffi::c_int,
    -(340 as core::ffi::c_int),
    -(520 as core::ffi::c_int),
    -(184 as core::ffi::c_int),
    880 as core::ffi::c_int,
    -(152 as core::ffi::c_int),
    -(680 as core::ffi::c_int),
    -(208 as core::ffi::c_int),
    -(1156 as core::ffi::c_int),
    -(300 as core::ffi::c_int),
    -(528 as core::ffi::c_int),
    -(472 as core::ffi::c_int),
    364 as core::ffi::c_int,
    100 as core::ffi::c_int,
    -(744 as core::ffi::c_int),
    -(1056 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    540 as core::ffi::c_int,
    280 as core::ffi::c_int,
    144 as core::ffi::c_int,
    -(676 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    -(232 as core::ffi::c_int),
    -(280 as core::ffi::c_int),
    -(224 as core::ffi::c_int),
    96 as core::ffi::c_int,
    568 as core::ffi::c_int,
    -(76 as core::ffi::c_int),
    172 as core::ffi::c_int,
    148 as core::ffi::c_int,
    148 as core::ffi::c_int,
    104 as core::ffi::c_int,
    32 as core::ffi::c_int,
    -(296 as core::ffi::c_int),
    -(32 as core::ffi::c_int),
    788 as core::ffi::c_int,
    -(80 as core::ffi::c_int),
    32 as core::ffi::c_int,
    -(16 as core::ffi::c_int),
    280 as core::ffi::c_int,
    288 as core::ffi::c_int,
    944 as core::ffi::c_int,
    428 as core::ffi::c_int,
    -(484 as core::ffi::c_int),
];
static mut gauss_bits: core::ffi::c_int = 11 as core::ffi::c_int;
static mut luma_subblock_size_y: core::ffi::c_int = 32 as core::ffi::c_int;
static mut luma_subblock_size_x: core::ffi::c_int = 32 as core::ffi::c_int;
static mut min_luma_legal_range: core::ffi::c_int = 16 as core::ffi::c_int;
static mut max_luma_legal_range: core::ffi::c_int = 235 as core::ffi::c_int;
static mut min_chroma_legal_range: core::ffi::c_int = 16 as core::ffi::c_int;
static mut max_chroma_legal_range: core::ffi::c_int = 240 as core::ffi::c_int;
unsafe extern "C-unwind" fn dealloc_arrays(
    mut params: *const aom_film_grain_t,
    mut pred_pos_luma: *mut *mut *mut core::ffi::c_int,
    mut pred_pos_chroma: *mut *mut *mut core::ffi::c_int,
    mut luma_grain_block: *mut *mut core::ffi::c_int,
    mut cb_grain_block: *mut *mut core::ffi::c_int,
    mut cr_grain_block: *mut *mut core::ffi::c_int,
    mut y_line_buf: *mut *mut core::ffi::c_int,
    mut cb_line_buf: *mut *mut core::ffi::c_int,
    mut cr_line_buf: *mut *mut core::ffi::c_int,
    mut y_col_buf: *mut *mut core::ffi::c_int,
    mut cb_col_buf: *mut *mut core::ffi::c_int,
    mut cr_col_buf: *mut *mut core::ffi::c_int,
) {
    let mut num_pos_luma: core::ffi::c_int = 2 as core::ffi::c_int
        * (*params).ar_coeff_lag * ((*params).ar_coeff_lag + 1 as core::ffi::c_int);
    let mut num_pos_chroma: core::ffi::c_int = num_pos_luma;
    if (*params).num_y_points > 0 as core::ffi::c_int {
        num_pos_chroma += 1;
    }
    if !(*pred_pos_luma).is_null() {
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < num_pos_luma {
            aom_free(*(*pred_pos_luma).offset(row as isize) as *mut core::ffi::c_void);
            row += 1;
        }
        aom_free(*pred_pos_luma as *mut core::ffi::c_void);
        *pred_pos_luma = 0 as *mut *mut core::ffi::c_int;
    }
    if !(*pred_pos_chroma).is_null() {
        let mut row_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while row_0 < num_pos_chroma {
            aom_free(
                *(*pred_pos_chroma).offset(row_0 as isize) as *mut core::ffi::c_void,
            );
            row_0 += 1;
        }
        aom_free(*pred_pos_chroma as *mut core::ffi::c_void);
        *pred_pos_chroma = 0 as *mut *mut core::ffi::c_int;
    }
    aom_free(*y_line_buf as *mut core::ffi::c_void);
    *y_line_buf = 0 as *mut core::ffi::c_int;
    aom_free(*cb_line_buf as *mut core::ffi::c_void);
    *cb_line_buf = 0 as *mut core::ffi::c_int;
    aom_free(*cr_line_buf as *mut core::ffi::c_void);
    *cr_line_buf = 0 as *mut core::ffi::c_int;
    aom_free(*y_col_buf as *mut core::ffi::c_void);
    *y_col_buf = 0 as *mut core::ffi::c_int;
    aom_free(*cb_col_buf as *mut core::ffi::c_void);
    *cb_col_buf = 0 as *mut core::ffi::c_int;
    aom_free(*cr_col_buf as *mut core::ffi::c_void);
    *cr_col_buf = 0 as *mut core::ffi::c_int;
    aom_free(*luma_grain_block as *mut core::ffi::c_void);
    *luma_grain_block = 0 as *mut core::ffi::c_int;
    aom_free(*cb_grain_block as *mut core::ffi::c_void);
    *cb_grain_block = 0 as *mut core::ffi::c_int;
    aom_free(*cr_grain_block as *mut core::ffi::c_void);
    *cr_grain_block = 0 as *mut core::ffi::c_int;
}
unsafe extern "C-unwind" fn init_arrays(
    mut params: *const aom_film_grain_t,
    mut luma_stride: core::ffi::c_int,
    mut chroma_stride: core::ffi::c_int,
    mut pred_pos_luma_p: *mut *mut *mut core::ffi::c_int,
    mut pred_pos_chroma_p: *mut *mut *mut core::ffi::c_int,
    mut luma_grain_block: *mut *mut core::ffi::c_int,
    mut cb_grain_block: *mut *mut core::ffi::c_int,
    mut cr_grain_block: *mut *mut core::ffi::c_int,
    mut y_line_buf: *mut *mut core::ffi::c_int,
    mut cb_line_buf: *mut *mut core::ffi::c_int,
    mut cr_line_buf: *mut *mut core::ffi::c_int,
    mut y_col_buf: *mut *mut core::ffi::c_int,
    mut cb_col_buf: *mut *mut core::ffi::c_int,
    mut cr_col_buf: *mut *mut core::ffi::c_int,
    mut luma_grain_samples: core::ffi::c_int,
    mut chroma_grain_samples: core::ffi::c_int,
    mut chroma_subsamp_y: core::ffi::c_int,
    mut chroma_subsamp_x: core::ffi::c_int,
) -> bool {
    *pred_pos_luma_p = 0 as *mut *mut core::ffi::c_int;
    *pred_pos_chroma_p = 0 as *mut *mut core::ffi::c_int;
    *luma_grain_block = 0 as *mut core::ffi::c_int;
    *cb_grain_block = 0 as *mut core::ffi::c_int;
    *cr_grain_block = 0 as *mut core::ffi::c_int;
    *y_line_buf = 0 as *mut core::ffi::c_int;
    *cb_line_buf = 0 as *mut core::ffi::c_int;
    *cr_line_buf = 0 as *mut core::ffi::c_int;
    *y_col_buf = 0 as *mut core::ffi::c_int;
    *cb_col_buf = 0 as *mut core::ffi::c_int;
    *cr_col_buf = 0 as *mut core::ffi::c_int;
    let chroma_subblock_size_y: core::ffi::c_int = luma_subblock_size_y
        >> chroma_subsamp_y;
    let mut num_pos_luma: core::ffi::c_int = 2 as core::ffi::c_int
        * (*params).ar_coeff_lag * ((*params).ar_coeff_lag + 1 as core::ffi::c_int);
    let mut num_pos_chroma: core::ffi::c_int = num_pos_luma;
    if (*params).num_y_points > 0 as core::ffi::c_int {
        num_pos_chroma += 1;
    }
    let mut pred_pos_luma: *mut *mut core::ffi::c_int = 0 as *mut *mut core::ffi::c_int;
    let mut pred_pos_chroma: *mut *mut core::ffi::c_int = 0
        as *mut *mut core::ffi::c_int;
    pred_pos_luma = aom_calloc(
        num_pos_luma as size_t,
        ::core::mem::size_of::<*mut core::ffi::c_int>() as size_t,
    ) as *mut *mut core::ffi::c_int;
    if pred_pos_luma.is_null() {
        return false_0 != 0;
    }
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < num_pos_luma {
        let fresh0 = &raw mut *pred_pos_luma.offset(row as isize);
        *fresh0 = aom_malloc(
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(3 as size_t),
        ) as *mut core::ffi::c_int;
        if (*pred_pos_luma.offset(row as isize)).is_null() {
            dealloc_arrays(
                params,
                pred_pos_luma_p,
                pred_pos_chroma_p,
                luma_grain_block,
                cb_grain_block,
                cr_grain_block,
                y_line_buf,
                cb_line_buf,
                cr_line_buf,
                y_col_buf,
                cb_col_buf,
                cr_col_buf,
            );
            return false_0 != 0;
        }
        row += 1;
    }
    pred_pos_chroma = aom_calloc(
        num_pos_chroma as size_t,
        ::core::mem::size_of::<*mut core::ffi::c_int>() as size_t,
    ) as *mut *mut core::ffi::c_int;
    if pred_pos_chroma.is_null() {
        dealloc_arrays(
            params,
            pred_pos_luma_p,
            pred_pos_chroma_p,
            luma_grain_block,
            cb_grain_block,
            cr_grain_block,
            y_line_buf,
            cb_line_buf,
            cr_line_buf,
            y_col_buf,
            cb_col_buf,
            cr_col_buf,
        );
        return false_0 != 0;
    }
    let mut row_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while row_0 < num_pos_chroma {
        let fresh1 = &raw mut *pred_pos_chroma.offset(row_0 as isize);
        *fresh1 = aom_malloc(
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(3 as size_t),
        ) as *mut core::ffi::c_int;
        if (*pred_pos_chroma.offset(row_0 as isize)).is_null() {
            dealloc_arrays(
                params,
                pred_pos_luma_p,
                pred_pos_chroma_p,
                luma_grain_block,
                cb_grain_block,
                cr_grain_block,
                y_line_buf,
                cb_line_buf,
                cr_line_buf,
                y_col_buf,
                cb_col_buf,
                cr_col_buf,
            );
            return false_0 != 0;
        }
        row_0 += 1;
    }
    let mut pos_ar_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row_1: core::ffi::c_int = -(*params).ar_coeff_lag;
    while row_1 < 0 as core::ffi::c_int {
        let mut col: core::ffi::c_int = -(*params).ar_coeff_lag;
        while col < (*params).ar_coeff_lag + 1 as core::ffi::c_int {
            *(*pred_pos_luma.offset(pos_ar_index as isize))
                .offset(0 as core::ffi::c_int as isize) = row_1;
            *(*pred_pos_luma.offset(pos_ar_index as isize))
                .offset(1 as core::ffi::c_int as isize) = col;
            *(*pred_pos_luma.offset(pos_ar_index as isize))
                .offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
            *(*pred_pos_chroma.offset(pos_ar_index as isize))
                .offset(0 as core::ffi::c_int as isize) = row_1;
            *(*pred_pos_chroma.offset(pos_ar_index as isize))
                .offset(1 as core::ffi::c_int as isize) = col;
            *(*pred_pos_chroma.offset(pos_ar_index as isize))
                .offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
            pos_ar_index += 1;
            col += 1;
        }
        row_1 += 1;
    }
    let mut col_0: core::ffi::c_int = -(*params).ar_coeff_lag;
    while col_0 < 0 as core::ffi::c_int {
        *(*pred_pos_luma.offset(pos_ar_index as isize))
            .offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        *(*pred_pos_luma.offset(pos_ar_index as isize))
            .offset(1 as core::ffi::c_int as isize) = col_0;
        *(*pred_pos_luma.offset(pos_ar_index as isize))
            .offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        *(*pred_pos_chroma.offset(pos_ar_index as isize))
            .offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        *(*pred_pos_chroma.offset(pos_ar_index as isize))
            .offset(1 as core::ffi::c_int as isize) = col_0;
        *(*pred_pos_chroma.offset(pos_ar_index as isize))
            .offset(2 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        pos_ar_index += 1;
        col_0 += 1;
    }
    if (*params).num_y_points > 0 as core::ffi::c_int {
        *(*pred_pos_chroma.offset(pos_ar_index as isize))
            .offset(0 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        *(*pred_pos_chroma.offset(pos_ar_index as isize))
            .offset(1 as core::ffi::c_int as isize) = 0 as core::ffi::c_int;
        *(*pred_pos_chroma.offset(pos_ar_index as isize))
            .offset(2 as core::ffi::c_int as isize) = 1 as core::ffi::c_int;
    }
    *pred_pos_luma_p = pred_pos_luma;
    *pred_pos_chroma_p = pred_pos_chroma;
    *y_line_buf = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(luma_stride as size_t)
            .wrapping_mul(2 as size_t),
    ) as *mut core::ffi::c_int;
    *cb_line_buf = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(chroma_stride as size_t)
            .wrapping_mul((2 as core::ffi::c_int >> chroma_subsamp_y) as size_t),
    ) as *mut core::ffi::c_int;
    *cr_line_buf = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(chroma_stride as size_t)
            .wrapping_mul((2 as core::ffi::c_int >> chroma_subsamp_y) as size_t),
    ) as *mut core::ffi::c_int;
    *y_col_buf = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul((luma_subblock_size_y + 2 as core::ffi::c_int) as size_t)
            .wrapping_mul(2 as size_t),
    ) as *mut core::ffi::c_int;
    *cb_col_buf = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(
                (chroma_subblock_size_y + (2 as core::ffi::c_int >> chroma_subsamp_y))
                    as size_t,
            )
            .wrapping_mul((2 as core::ffi::c_int >> chroma_subsamp_x) as size_t),
    ) as *mut core::ffi::c_int;
    *cr_col_buf = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(
                (chroma_subblock_size_y + (2 as core::ffi::c_int >> chroma_subsamp_y))
                    as size_t,
            )
            .wrapping_mul((2 as core::ffi::c_int >> chroma_subsamp_x) as size_t),
    ) as *mut core::ffi::c_int;
    *luma_grain_block = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(luma_grain_samples as size_t),
    ) as *mut core::ffi::c_int;
    *cb_grain_block = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(chroma_grain_samples as size_t),
    ) as *mut core::ffi::c_int;
    *cr_grain_block = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_int>() as size_t)
            .wrapping_mul(chroma_grain_samples as size_t),
    ) as *mut core::ffi::c_int;
    if !(!(*pred_pos_luma_p).is_null() && !(*pred_pos_chroma_p).is_null()
        && !(*y_line_buf).is_null() && !(*cb_line_buf).is_null()
        && !(*cr_line_buf).is_null() && !(*y_col_buf).is_null()
        && !(*cb_col_buf).is_null() && !(*cr_col_buf).is_null()
        && !(*luma_grain_block).is_null() && !(*cb_grain_block).is_null()
        && !(*cr_grain_block).is_null())
    {
        dealloc_arrays(
            params,
            pred_pos_luma_p,
            pred_pos_chroma_p,
            luma_grain_block,
            cb_grain_block,
            cr_grain_block,
            y_line_buf,
            cb_line_buf,
            cr_line_buf,
            y_col_buf,
            cb_col_buf,
            cr_col_buf,
        );
        return false_0 != 0;
    }
    return true_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn get_random_number(
    mut rng: *mut aom_grain_rng_t,
    mut bits: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut bit: uint16_t = 0;
    bit = (((*rng).random_register as core::ffi::c_int >> 0 as core::ffi::c_int
        ^ (*rng).random_register as core::ffi::c_int >> 1 as core::ffi::c_int
        ^ (*rng).random_register as core::ffi::c_int >> 3 as core::ffi::c_int
        ^ (*rng).random_register as core::ffi::c_int >> 12 as core::ffi::c_int)
        & 1 as core::ffi::c_int) as uint16_t;
    (*rng).random_register = ((*rng).random_register as core::ffi::c_int
        >> 1 as core::ffi::c_int | (bit as core::ffi::c_int) << 15 as core::ffi::c_int)
        as uint16_t;
    return (*rng).random_register as core::ffi::c_int >> 16 as core::ffi::c_int - bits
        & ((1 as core::ffi::c_int) << bits) - 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn init_random_generator(
    mut rng: *mut aom_grain_rng_t,
    mut luma_line: core::ffi::c_int,
    mut seed: uint16_t,
) {
    let mut msb: uint16_t = (seed as core::ffi::c_int >> 8 as core::ffi::c_int
        & 255 as core::ffi::c_int) as uint16_t;
    let mut lsb: uint16_t = (seed as core::ffi::c_int & 255 as core::ffi::c_int)
        as uint16_t;
    (*rng).random_register = (((msb as core::ffi::c_int) << 8 as core::ffi::c_int)
        + lsb as core::ffi::c_int) as uint16_t;
    let mut luma_num: core::ffi::c_int = luma_line >> 5 as core::ffi::c_int;
    (*rng).random_register = ((*rng).random_register as core::ffi::c_int
        ^ (luma_num * 37 as core::ffi::c_int + 178 as core::ffi::c_int
            & 255 as core::ffi::c_int) << 8 as core::ffi::c_int) as uint16_t;
    (*rng).random_register = ((*rng).random_register as core::ffi::c_int
        ^ luma_num * 173 as core::ffi::c_int + 105 as core::ffi::c_int
            & 255 as core::ffi::c_int) as uint16_t;
}
unsafe extern "C-unwind" fn generate_luma_grain_block(
    mut params: *const aom_film_grain_t,
    mut rng: *mut aom_grain_rng_t,
    mut pred_pos_luma: *mut *mut core::ffi::c_int,
    mut luma_grain_block: *mut core::ffi::c_int,
    mut luma_block_size_y: core::ffi::c_int,
    mut luma_block_size_x: core::ffi::c_int,
    mut luma_grain_stride: core::ffi::c_int,
    mut left_pad: core::ffi::c_int,
    mut top_pad: core::ffi::c_int,
    mut right_pad: core::ffi::c_int,
    mut bottom_pad: core::ffi::c_int,
) {
    if (*params).num_y_points == 0 as core::ffi::c_int {
        memset(
            luma_grain_block as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(luma_block_size_y as size_t)
                .wrapping_mul(luma_grain_stride as size_t),
        );
        return;
    }
    let mut bit_depth: core::ffi::c_int = (*params).bit_depth as core::ffi::c_int;
    let mut gauss_sec_shift: core::ffi::c_int = 12 as core::ffi::c_int - bit_depth
        + (*params).grain_scale_shift;
    let mut num_pos_luma: core::ffi::c_int = 2 as core::ffi::c_int
        * (*params).ar_coeff_lag * ((*params).ar_coeff_lag + 1 as core::ffi::c_int);
    let mut rounding_offset: core::ffi::c_int = (1 as core::ffi::c_int)
        << (*params).ar_coeff_shift - 1 as core::ffi::c_int;
    let grain_min: core::ffi::c_int = -((1 as core::ffi::c_int)
        << bit_depth - 1 as core::ffi::c_int);
    let grain_max: core::ffi::c_int = ((1 as core::ffi::c_int)
        << bit_depth - 1 as core::ffi::c_int) - 1 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < luma_block_size_y {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < luma_block_size_x {
            *luma_grain_block.offset((i * luma_grain_stride + j) as isize) = gaussian_sequence[get_random_number(
                rng,
                gauss_bits,
            ) as usize]
                + ((1 as core::ffi::c_int) << gauss_sec_shift >> 1 as core::ffi::c_int)
                >> gauss_sec_shift;
            j += 1;
        }
        i += 1;
    }
    let mut i_0: core::ffi::c_int = top_pad;
    while i_0 < luma_block_size_y - bottom_pad {
        let mut j_0: core::ffi::c_int = left_pad;
        while j_0 < luma_block_size_x - right_pad {
            let mut wsum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut pos: core::ffi::c_int = 0 as core::ffi::c_int;
            while pos < num_pos_luma {
                wsum = wsum
                    + (*params).ar_coeffs_y[pos as usize]
                        * *luma_grain_block
                            .offset(
                                ((i_0
                                    + *(*pred_pos_luma.offset(pos as isize))
                                        .offset(0 as core::ffi::c_int as isize)) * luma_grain_stride
                                    + j_0
                                    + *(*pred_pos_luma.offset(pos as isize))
                                        .offset(1 as core::ffi::c_int as isize)) as isize,
                            );
                pos += 1;
            }
            *luma_grain_block.offset((i_0 * luma_grain_stride + j_0) as isize) = clamp(
                *luma_grain_block.offset((i_0 * luma_grain_stride + j_0) as isize)
                    + (wsum + rounding_offset >> (*params).ar_coeff_shift),
                grain_min,
                grain_max,
            );
            j_0 += 1;
        }
        i_0 += 1;
    }
}
unsafe extern "C-unwind" fn generate_chroma_grain_blocks(
    mut params: *const aom_film_grain_t,
    mut rng: *mut aom_grain_rng_t,
    mut pred_pos_chroma: *mut *mut core::ffi::c_int,
    mut luma_grain_block: *mut core::ffi::c_int,
    mut cb_grain_block: *mut core::ffi::c_int,
    mut cr_grain_block: *mut core::ffi::c_int,
    mut luma_grain_stride: core::ffi::c_int,
    mut chroma_block_size_y: core::ffi::c_int,
    mut chroma_block_size_x: core::ffi::c_int,
    mut chroma_grain_stride: core::ffi::c_int,
    mut left_pad: core::ffi::c_int,
    mut top_pad: core::ffi::c_int,
    mut right_pad: core::ffi::c_int,
    mut bottom_pad: core::ffi::c_int,
    mut chroma_subsamp_y: core::ffi::c_int,
    mut chroma_subsamp_x: core::ffi::c_int,
) -> bool {
    let mut bit_depth: core::ffi::c_int = (*params).bit_depth as core::ffi::c_int;
    let mut gauss_sec_shift: core::ffi::c_int = 12 as core::ffi::c_int - bit_depth
        + (*params).grain_scale_shift;
    let mut num_pos_chroma: core::ffi::c_int = 2 as core::ffi::c_int
        * (*params).ar_coeff_lag * ((*params).ar_coeff_lag + 1 as core::ffi::c_int);
    if (*params).num_y_points > 0 as core::ffi::c_int {
        num_pos_chroma += 1;
    }
    let mut rounding_offset: core::ffi::c_int = (1 as core::ffi::c_int)
        << (*params).ar_coeff_shift - 1 as core::ffi::c_int;
    let mut chroma_grain_block_size: core::ffi::c_int = chroma_block_size_y
        * chroma_grain_stride;
    let grain_min: core::ffi::c_int = -((1 as core::ffi::c_int)
        << bit_depth - 1 as core::ffi::c_int);
    let grain_max: core::ffi::c_int = ((1 as core::ffi::c_int)
        << bit_depth - 1 as core::ffi::c_int) - 1 as core::ffi::c_int;
    if (*params).num_cb_points != 0 || (*params).chroma_scaling_from_luma != 0 {
        init_random_generator(
            rng,
            (7 as core::ffi::c_int) << 5 as core::ffi::c_int,
            (*params).random_seed,
        );
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < chroma_block_size_y {
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < chroma_block_size_x {
                *cb_grain_block.offset((i * chroma_grain_stride + j) as isize) = gaussian_sequence[get_random_number(
                    rng,
                    gauss_bits,
                ) as usize]
                    + ((1 as core::ffi::c_int) << gauss_sec_shift
                        >> 1 as core::ffi::c_int) >> gauss_sec_shift;
                j += 1;
            }
            i += 1;
        }
    } else {
        memset(
            cb_grain_block as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(chroma_grain_block_size as size_t),
        );
    }
    if (*params).num_cr_points != 0 || (*params).chroma_scaling_from_luma != 0 {
        init_random_generator(
            rng,
            (11 as core::ffi::c_int) << 5 as core::ffi::c_int,
            (*params).random_seed,
        );
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < chroma_block_size_y {
            let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_0 < chroma_block_size_x {
                *cr_grain_block.offset((i_0 * chroma_grain_stride + j_0) as isize) = gaussian_sequence[get_random_number(
                    rng,
                    gauss_bits,
                ) as usize]
                    + ((1 as core::ffi::c_int) << gauss_sec_shift
                        >> 1 as core::ffi::c_int) >> gauss_sec_shift;
                j_0 += 1;
            }
            i_0 += 1;
        }
    } else {
        memset(
            cr_grain_block as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(chroma_grain_block_size as size_t),
        );
    }
    let mut i_1: core::ffi::c_int = top_pad;
    while i_1 < chroma_block_size_y - bottom_pad {
        let mut j_1: core::ffi::c_int = left_pad;
        while j_1 < chroma_block_size_x - right_pad {
            let mut wsum_cb: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut wsum_cr: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut pos: core::ffi::c_int = 0 as core::ffi::c_int;
            while pos < num_pos_chroma {
                if *(*pred_pos_chroma.offset(pos as isize))
                    .offset(2 as core::ffi::c_int as isize) == 0 as core::ffi::c_int
                {
                    wsum_cb = wsum_cb
                        + (*params).ar_coeffs_cb[pos as usize]
                            * *cb_grain_block
                                .offset(
                                    ((i_1
                                        + *(*pred_pos_chroma.offset(pos as isize))
                                            .offset(0 as core::ffi::c_int as isize))
                                        * chroma_grain_stride + j_1
                                        + *(*pred_pos_chroma.offset(pos as isize))
                                            .offset(1 as core::ffi::c_int as isize)) as isize,
                                );
                    wsum_cr = wsum_cr
                        + (*params).ar_coeffs_cr[pos as usize]
                            * *cr_grain_block
                                .offset(
                                    ((i_1
                                        + *(*pred_pos_chroma.offset(pos as isize))
                                            .offset(0 as core::ffi::c_int as isize))
                                        * chroma_grain_stride + j_1
                                        + *(*pred_pos_chroma.offset(pos as isize))
                                            .offset(1 as core::ffi::c_int as isize)) as isize,
                                );
                } else if *(*pred_pos_chroma.offset(pos as isize))
                    .offset(2 as core::ffi::c_int as isize) == 1 as core::ffi::c_int
                {
                    let mut av_luma: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut luma_coord_y: core::ffi::c_int = (i_1 - top_pad
                        << chroma_subsamp_y) + top_pad;
                    let mut luma_coord_x: core::ffi::c_int = (j_1 - left_pad
                        << chroma_subsamp_x) + left_pad;
                    let mut k: core::ffi::c_int = luma_coord_y;
                    while k < luma_coord_y + chroma_subsamp_y + 1 as core::ffi::c_int {
                        let mut l: core::ffi::c_int = luma_coord_x;
                        while l < luma_coord_x + chroma_subsamp_x + 1 as core::ffi::c_int
                        {
                            av_luma
                                += *luma_grain_block
                                    .offset((k * luma_grain_stride + l) as isize);
                            l += 1;
                        }
                        k += 1;
                    }
                    av_luma = av_luma
                        + ((1 as core::ffi::c_int) << chroma_subsamp_y + chroma_subsamp_x
                            >> 1 as core::ffi::c_int)
                        >> chroma_subsamp_y + chroma_subsamp_x;
                    wsum_cb = wsum_cb + (*params).ar_coeffs_cb[pos as usize] * av_luma;
                    wsum_cr = wsum_cr + (*params).ar_coeffs_cr[pos as usize] * av_luma;
                } else {
                    fprintf(
                        __stderrp,
                        b"Grain synthesis: prediction between two chroma components is not supported!\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                    return false_0 != 0;
                }
                pos += 1;
            }
            if (*params).num_cb_points != 0 || (*params).chroma_scaling_from_luma != 0 {
                *cb_grain_block.offset((i_1 * chroma_grain_stride + j_1) as isize) = clamp(
                    *cb_grain_block.offset((i_1 * chroma_grain_stride + j_1) as isize)
                        + (wsum_cb + rounding_offset >> (*params).ar_coeff_shift),
                    grain_min,
                    grain_max,
                );
            }
            if (*params).num_cr_points != 0 || (*params).chroma_scaling_from_luma != 0 {
                *cr_grain_block.offset((i_1 * chroma_grain_stride + j_1) as isize) = clamp(
                    *cr_grain_block.offset((i_1 * chroma_grain_stride + j_1) as isize)
                        + (wsum_cr + rounding_offset >> (*params).ar_coeff_shift),
                    grain_min,
                    grain_max,
                );
            }
            j_1 += 1;
        }
        i_1 += 1;
    }
    return true_0 != 0;
}
unsafe extern "C-unwind" fn init_scaling_function(
    mut scaling_points: *const [core::ffi::c_int; 2],
    mut num_points: core::ffi::c_int,
    mut scaling_lut: *mut core::ffi::c_int,
) {
    if num_points == 0 as core::ffi::c_int {
        return;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i
        < (*scaling_points
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
    {
        *scaling_lut.offset(i as isize) = (*scaling_points
            .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize];
        i += 1;
    }
    let mut point: core::ffi::c_int = 0 as core::ffi::c_int;
    while point < num_points - 1 as core::ffi::c_int {
        let mut delta_y: core::ffi::c_int = (*scaling_points
            .offset(
                (point + 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize]
            - (*scaling_points.offset(point as isize))[1 as core::ffi::c_int as usize];
        let mut delta_x: core::ffi::c_int = (*scaling_points
            .offset(
                (point + 1 as core::ffi::c_int) as isize,
            ))[0 as core::ffi::c_int as usize]
            - (*scaling_points.offset(point as isize))[0 as core::ffi::c_int as usize];
        let mut delta: int64_t = (delta_y
            * ((65536 as core::ffi::c_int + (delta_x >> 1 as core::ffi::c_int))
                / delta_x)) as int64_t;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < delta_x {
            *scaling_lut
                .offset(
                    ((*scaling_points
                        .offset(point as isize))[0 as core::ffi::c_int as usize] + x)
                        as isize,
                ) = (*scaling_points
                .offset(point as isize))[1 as core::ffi::c_int as usize]
                + (x as int64_t * delta + 32768 as int64_t >> 16 as core::ffi::c_int)
                    as core::ffi::c_int;
            x += 1;
        }
        point += 1;
    }
    let mut i_0: core::ffi::c_int = (*scaling_points
        .offset(
            (num_points - 1 as core::ffi::c_int) as isize,
        ))[0 as core::ffi::c_int as usize];
    while i_0 < 256 as core::ffi::c_int {
        *scaling_lut.offset(i_0 as isize) = (*scaling_points
            .offset(
                (num_points - 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize];
        i_0 += 1;
    }
}
unsafe extern "C-unwind" fn scale_LUT(
    mut scaling_lut: *const core::ffi::c_int,
    mut index: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut x: core::ffi::c_int = index >> bit_depth - 8 as core::ffi::c_int;
    if bit_depth - 8 as core::ffi::c_int == 0 || x == 255 as core::ffi::c_int {
        return *scaling_lut.offset(x as isize)
    } else {
        return *scaling_lut.offset(x as isize)
            + ((*scaling_lut.offset((x + 1 as core::ffi::c_int) as isize)
                - *scaling_lut.offset(x as isize))
                * (index
                    & ((1 as core::ffi::c_int) << bit_depth - 8 as core::ffi::c_int)
                        - 1 as core::ffi::c_int)
                + ((1 as core::ffi::c_int) << bit_depth - 9 as core::ffi::c_int)
                >> bit_depth - 8 as core::ffi::c_int)
    };
}
unsafe extern "C-unwind" fn add_noise_to_block(
    mut params: *const aom_film_grain_t,
    mut scaling_lut: *const aom_grain_scaling_lut_t,
    mut luma: *mut uint8_t,
    mut cb: *mut uint8_t,
    mut cr: *mut uint8_t,
    mut luma_stride: core::ffi::c_int,
    mut chroma_stride: core::ffi::c_int,
    mut luma_grain: *mut core::ffi::c_int,
    mut cb_grain: *mut core::ffi::c_int,
    mut cr_grain: *mut core::ffi::c_int,
    mut luma_grain_stride: core::ffi::c_int,
    mut chroma_grain_stride: core::ffi::c_int,
    mut half_luma_height: core::ffi::c_int,
    mut half_luma_width: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut chroma_subsamp_y: core::ffi::c_int,
    mut chroma_subsamp_x: core::ffi::c_int,
    mut mc_identity: core::ffi::c_int,
) {
    let mut cb_mult: core::ffi::c_int = (*params).cb_mult - 128 as core::ffi::c_int;
    let mut cb_luma_mult: core::ffi::c_int = (*params).cb_luma_mult
        - 128 as core::ffi::c_int;
    let mut cb_offset: core::ffi::c_int = (*params).cb_offset - 256 as core::ffi::c_int;
    let mut cr_mult: core::ffi::c_int = (*params).cr_mult - 128 as core::ffi::c_int;
    let mut cr_luma_mult: core::ffi::c_int = (*params).cr_luma_mult
        - 128 as core::ffi::c_int;
    let mut cr_offset: core::ffi::c_int = (*params).cr_offset - 256 as core::ffi::c_int;
    let mut rounding_offset: core::ffi::c_int = (1 as core::ffi::c_int)
        << (*params).scaling_shift - 1 as core::ffi::c_int;
    let mut apply_y: core::ffi::c_int = if (*params).num_y_points > 0 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut apply_cb: core::ffi::c_int = if (*params).num_cb_points
        > 0 as core::ffi::c_int || (*params).chroma_scaling_from_luma != 0
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut apply_cr: core::ffi::c_int = if (*params).num_cr_points
        > 0 as core::ffi::c_int || (*params).chroma_scaling_from_luma != 0
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if (*params).chroma_scaling_from_luma != 0 {
        cb_mult = 0 as core::ffi::c_int;
        cb_luma_mult = 64 as core::ffi::c_int;
        cb_offset = 0 as core::ffi::c_int;
        cr_mult = 0 as core::ffi::c_int;
        cr_luma_mult = 64 as core::ffi::c_int;
        cr_offset = 0 as core::ffi::c_int;
    }
    let mut min_luma: core::ffi::c_int = 0;
    let mut max_luma: core::ffi::c_int = 0;
    let mut min_chroma: core::ffi::c_int = 0;
    let mut max_chroma: core::ffi::c_int = 0;
    if (*params).clip_to_restricted_range != 0 {
        min_luma = min_luma_legal_range;
        max_luma = max_luma_legal_range;
        if mc_identity != 0 {
            min_chroma = min_luma_legal_range;
            max_chroma = max_luma_legal_range;
        } else {
            min_chroma = min_chroma_legal_range;
            max_chroma = max_chroma_legal_range;
        }
    } else {
        min_chroma = 0 as core::ffi::c_int;
        min_luma = min_chroma;
        max_chroma = 255 as core::ffi::c_int;
        max_luma = max_chroma;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < half_luma_height << 1 as core::ffi::c_int - chroma_subsamp_y {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < half_luma_width << 1 as core::ffi::c_int - chroma_subsamp_x {
            let mut average_luma: core::ffi::c_int = 0 as core::ffi::c_int;
            if chroma_subsamp_x != 0 {
                average_luma = *luma
                    .offset(
                        ((i << chroma_subsamp_y) * luma_stride + (j << chroma_subsamp_x))
                            as isize,
                    ) as core::ffi::c_int
                    + *luma
                        .offset(
                            ((i << chroma_subsamp_y) * luma_stride
                                + (j << chroma_subsamp_x) + 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int + 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int;
            } else {
                average_luma = *luma
                    .offset(((i << chroma_subsamp_y) * luma_stride + j) as isize)
                    as core::ffi::c_int;
            }
            if apply_cb != 0 {
                *cb.offset((i * chroma_stride + j) as isize) = clamp(
                    *cb.offset((i * chroma_stride + j) as isize) as core::ffi::c_int
                        + (scale_LUT(
                            ((*scaling_lut).cb).as_ptr(),
                            clamp(
                                (average_luma * cb_luma_mult
                                    + cb_mult
                                        * *cb.offset((i * chroma_stride + j) as isize)
                                            as core::ffi::c_int >> 6 as core::ffi::c_int) + cb_offset,
                                0 as core::ffi::c_int,
                                ((256 as core::ffi::c_int)
                                    << bit_depth - 8 as core::ffi::c_int)
                                    - 1 as core::ffi::c_int,
                            ),
                            8 as core::ffi::c_int,
                        ) * *cb_grain.offset((i * chroma_grain_stride + j) as isize)
                            + rounding_offset >> (*params).scaling_shift),
                    min_chroma,
                    max_chroma,
                ) as uint8_t;
            }
            if apply_cr != 0 {
                *cr.offset((i * chroma_stride + j) as isize) = clamp(
                    *cr.offset((i * chroma_stride + j) as isize) as core::ffi::c_int
                        + (scale_LUT(
                            ((*scaling_lut).cr).as_ptr(),
                            clamp(
                                (average_luma * cr_luma_mult
                                    + cr_mult
                                        * *cr.offset((i * chroma_stride + j) as isize)
                                            as core::ffi::c_int >> 6 as core::ffi::c_int) + cr_offset,
                                0 as core::ffi::c_int,
                                ((256 as core::ffi::c_int)
                                    << bit_depth - 8 as core::ffi::c_int)
                                    - 1 as core::ffi::c_int,
                            ),
                            8 as core::ffi::c_int,
                        ) * *cr_grain.offset((i * chroma_grain_stride + j) as isize)
                            + rounding_offset >> (*params).scaling_shift),
                    min_chroma,
                    max_chroma,
                ) as uint8_t;
            }
            j += 1;
        }
        i += 1;
    }
    if apply_y != 0 {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < half_luma_height << 1 as core::ffi::c_int {
            let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_0 < half_luma_width << 1 as core::ffi::c_int {
                *luma.offset((i_0 * luma_stride + j_0) as isize) = clamp(
                    *luma.offset((i_0 * luma_stride + j_0) as isize) as core::ffi::c_int
                        + (scale_LUT(
                            ((*scaling_lut).y).as_ptr(),
                            *luma.offset((i_0 * luma_stride + j_0) as isize)
                                as core::ffi::c_int,
                            8 as core::ffi::c_int,
                        ) * *luma_grain.offset((i_0 * luma_grain_stride + j_0) as isize)
                            + rounding_offset >> (*params).scaling_shift),
                    min_luma,
                    max_luma,
                ) as uint8_t;
                j_0 += 1;
            }
            i_0 += 1;
        }
    }
}
unsafe extern "C-unwind" fn add_noise_to_block_hbd(
    mut params: *const aom_film_grain_t,
    mut scaling_lut: *const aom_grain_scaling_lut_t,
    mut luma: *mut uint16_t,
    mut cb: *mut uint16_t,
    mut cr: *mut uint16_t,
    mut luma_stride: core::ffi::c_int,
    mut chroma_stride: core::ffi::c_int,
    mut luma_grain: *mut core::ffi::c_int,
    mut cb_grain: *mut core::ffi::c_int,
    mut cr_grain: *mut core::ffi::c_int,
    mut luma_grain_stride: core::ffi::c_int,
    mut chroma_grain_stride: core::ffi::c_int,
    mut half_luma_height: core::ffi::c_int,
    mut half_luma_width: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut chroma_subsamp_y: core::ffi::c_int,
    mut chroma_subsamp_x: core::ffi::c_int,
    mut mc_identity: core::ffi::c_int,
) {
    let mut cb_mult: core::ffi::c_int = (*params).cb_mult - 128 as core::ffi::c_int;
    let mut cb_luma_mult: core::ffi::c_int = (*params).cb_luma_mult
        - 128 as core::ffi::c_int;
    let mut cb_offset: core::ffi::c_int = ((*params).cb_offset
        << bit_depth - 8 as core::ffi::c_int) - ((1 as core::ffi::c_int) << bit_depth);
    let mut cr_mult: core::ffi::c_int = (*params).cr_mult - 128 as core::ffi::c_int;
    let mut cr_luma_mult: core::ffi::c_int = (*params).cr_luma_mult
        - 128 as core::ffi::c_int;
    let mut cr_offset: core::ffi::c_int = ((*params).cr_offset
        << bit_depth - 8 as core::ffi::c_int) - ((1 as core::ffi::c_int) << bit_depth);
    let mut rounding_offset: core::ffi::c_int = (1 as core::ffi::c_int)
        << (*params).scaling_shift - 1 as core::ffi::c_int;
    let mut apply_y: core::ffi::c_int = if (*params).num_y_points > 0 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut apply_cb: core::ffi::c_int = if ((*params).num_cb_points
        > 0 as core::ffi::c_int || (*params).chroma_scaling_from_luma != 0)
        as core::ffi::c_int > 0 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let mut apply_cr: core::ffi::c_int = if ((*params).num_cr_points
        > 0 as core::ffi::c_int || (*params).chroma_scaling_from_luma != 0)
        as core::ffi::c_int > 0 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if (*params).chroma_scaling_from_luma != 0 {
        cb_mult = 0 as core::ffi::c_int;
        cb_luma_mult = 64 as core::ffi::c_int;
        cb_offset = 0 as core::ffi::c_int;
        cr_mult = 0 as core::ffi::c_int;
        cr_luma_mult = 64 as core::ffi::c_int;
        cr_offset = 0 as core::ffi::c_int;
    }
    let mut min_luma: core::ffi::c_int = 0;
    let mut max_luma: core::ffi::c_int = 0;
    let mut min_chroma: core::ffi::c_int = 0;
    let mut max_chroma: core::ffi::c_int = 0;
    if (*params).clip_to_restricted_range != 0 {
        min_luma = min_luma_legal_range << bit_depth - 8 as core::ffi::c_int;
        max_luma = max_luma_legal_range << bit_depth - 8 as core::ffi::c_int;
        if mc_identity != 0 {
            min_chroma = min_luma_legal_range << bit_depth - 8 as core::ffi::c_int;
            max_chroma = max_luma_legal_range << bit_depth - 8 as core::ffi::c_int;
        } else {
            min_chroma = min_chroma_legal_range << bit_depth - 8 as core::ffi::c_int;
            max_chroma = max_chroma_legal_range << bit_depth - 8 as core::ffi::c_int;
        }
    } else {
        min_chroma = 0 as core::ffi::c_int;
        min_luma = min_chroma;
        max_chroma = ((256 as core::ffi::c_int) << bit_depth - 8 as core::ffi::c_int)
            - 1 as core::ffi::c_int;
        max_luma = max_chroma;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < half_luma_height << 1 as core::ffi::c_int - chroma_subsamp_y {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < half_luma_width << 1 as core::ffi::c_int - chroma_subsamp_x {
            let mut average_luma: core::ffi::c_int = 0 as core::ffi::c_int;
            if chroma_subsamp_x != 0 {
                average_luma = *luma
                    .offset(
                        ((i << chroma_subsamp_y) * luma_stride + (j << chroma_subsamp_x))
                            as isize,
                    ) as core::ffi::c_int
                    + *luma
                        .offset(
                            ((i << chroma_subsamp_y) * luma_stride
                                + (j << chroma_subsamp_x) + 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int + 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int;
            } else {
                average_luma = *luma
                    .offset(((i << chroma_subsamp_y) * luma_stride + j) as isize)
                    as core::ffi::c_int;
            }
            if apply_cb != 0 {
                *cb.offset((i * chroma_stride + j) as isize) = clamp(
                    *cb.offset((i * chroma_stride + j) as isize) as core::ffi::c_int
                        + (scale_LUT(
                            ((*scaling_lut).cb).as_ptr(),
                            clamp(
                                (average_luma * cb_luma_mult
                                    + cb_mult
                                        * *cb.offset((i * chroma_stride + j) as isize)
                                            as core::ffi::c_int >> 6 as core::ffi::c_int) + cb_offset,
                                0 as core::ffi::c_int,
                                ((256 as core::ffi::c_int)
                                    << bit_depth - 8 as core::ffi::c_int)
                                    - 1 as core::ffi::c_int,
                            ),
                            bit_depth,
                        ) * *cb_grain.offset((i * chroma_grain_stride + j) as isize)
                            + rounding_offset >> (*params).scaling_shift),
                    min_chroma,
                    max_chroma,
                ) as uint16_t;
            }
            if apply_cr != 0 {
                *cr.offset((i * chroma_stride + j) as isize) = clamp(
                    *cr.offset((i * chroma_stride + j) as isize) as core::ffi::c_int
                        + (scale_LUT(
                            ((*scaling_lut).cr).as_ptr(),
                            clamp(
                                (average_luma * cr_luma_mult
                                    + cr_mult
                                        * *cr.offset((i * chroma_stride + j) as isize)
                                            as core::ffi::c_int >> 6 as core::ffi::c_int) + cr_offset,
                                0 as core::ffi::c_int,
                                ((256 as core::ffi::c_int)
                                    << bit_depth - 8 as core::ffi::c_int)
                                    - 1 as core::ffi::c_int,
                            ),
                            bit_depth,
                        ) * *cr_grain.offset((i * chroma_grain_stride + j) as isize)
                            + rounding_offset >> (*params).scaling_shift),
                    min_chroma,
                    max_chroma,
                ) as uint16_t;
            }
            j += 1;
        }
        i += 1;
    }
    if apply_y != 0 {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < half_luma_height << 1 as core::ffi::c_int {
            let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while j_0 < half_luma_width << 1 as core::ffi::c_int {
                *luma.offset((i_0 * luma_stride + j_0) as isize) = clamp(
                    *luma.offset((i_0 * luma_stride + j_0) as isize) as core::ffi::c_int
                        + (scale_LUT(
                            ((*scaling_lut).y).as_ptr(),
                            *luma.offset((i_0 * luma_stride + j_0) as isize)
                                as core::ffi::c_int,
                            bit_depth,
                        ) * *luma_grain.offset((i_0 * luma_grain_stride + j_0) as isize)
                            + rounding_offset >> (*params).scaling_shift),
                    min_luma,
                    max_luma,
                ) as uint16_t;
                j_0 += 1;
            }
            i_0 += 1;
        }
    }
}
unsafe extern "C-unwind" fn copy_rect(
    mut src: *mut uint8_t,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut use_high_bit_depth: core::ffi::c_int,
) {
    let mut hbd_coeff: core::ffi::c_int = if use_high_bit_depth != 0 {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    while height != 0 {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (width as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t)
                .wrapping_mul(hbd_coeff as size_t),
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        height -= 1;
    }
}
unsafe extern "C-unwind" fn copy_area(
    mut src: *mut core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst: *mut core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) {
    while height != 0 {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            (width as size_t)
                .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
        );
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
        height -= 1;
    }
}
unsafe extern "C-unwind" fn extend_even(
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut use_high_bit_depth: core::ffi::c_int,
) {
    if width & 1 as core::ffi::c_int == 0 as core::ffi::c_int
        && height & 1 as core::ffi::c_int == 0 as core::ffi::c_int
    {
        return;
    }
    if use_high_bit_depth != 0 {
        let mut dst16: *mut uint16_t = dst as *mut uint16_t;
        let mut dst16_stride: core::ffi::c_int = dst_stride / 2 as core::ffi::c_int;
        if width & 1 as core::ffi::c_int != 0 {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < height {
                *dst16.offset((i * dst16_stride + width) as isize) = *dst16
                    .offset((i * dst16_stride + width - 1 as core::ffi::c_int) as isize);
                i += 1;
            }
        }
        width = width + 1 as core::ffi::c_int & !(1 as core::ffi::c_int);
        if height & 1 as core::ffi::c_int != 0 {
            memcpy(
                &raw mut *dst16.offset((height * dst16_stride) as isize) as *mut uint16_t
                    as *mut core::ffi::c_void,
                &raw mut *dst16
                    .offset(((height - 1 as core::ffi::c_int) * dst16_stride) as isize)
                    as *mut uint16_t as *const core::ffi::c_void,
                (::core::mem::size_of::<uint16_t>() as size_t)
                    .wrapping_mul(width as size_t),
            );
        }
    } else {
        if width & 1 as core::ffi::c_int != 0 {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < height {
                *dst.offset((i_0 * dst_stride + width) as isize) = *dst
                    .offset((i_0 * dst_stride + width - 1 as core::ffi::c_int) as isize);
                i_0 += 1;
            }
        }
        width = width + 1 as core::ffi::c_int & !(1 as core::ffi::c_int);
        if height & 1 as core::ffi::c_int != 0 {
            memcpy(
                &raw mut *dst.offset((height * dst_stride) as isize) as *mut uint8_t
                    as *mut core::ffi::c_void,
                &raw mut *dst
                    .offset(((height - 1 as core::ffi::c_int) * dst_stride) as isize)
                    as *mut uint8_t as *const core::ffi::c_void,
                (::core::mem::size_of::<uint8_t>() as size_t)
                    .wrapping_mul(width as size_t),
            );
        }
    };
}
unsafe extern "C-unwind" fn ver_boundary_overlap(
    mut left_block: *mut core::ffi::c_int,
    mut left_stride: core::ffi::c_int,
    mut right_block: *mut core::ffi::c_int,
    mut right_stride: core::ffi::c_int,
    mut dst_block: *mut core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut grain_min: core::ffi::c_int,
    mut grain_max: core::ffi::c_int,
) {
    if width == 1 as core::ffi::c_int {
        while height != 0 {
            *dst_block = clamp(
                *left_block * 23 as core::ffi::c_int
                    + *right_block * 22 as core::ffi::c_int + 16 as core::ffi::c_int
                    >> 5 as core::ffi::c_int,
                grain_min,
                grain_max,
            );
            left_block = left_block.offset(left_stride as isize);
            right_block = right_block.offset(right_stride as isize);
            dst_block = dst_block.offset(dst_stride as isize);
            height -= 1;
        }
        return;
    } else if width == 2 as core::ffi::c_int {
        while height != 0 {
            *dst_block.offset(0 as core::ffi::c_int as isize) = clamp(
                27 as core::ffi::c_int
                    * *left_block.offset(0 as core::ffi::c_int as isize)
                    + 17 as core::ffi::c_int
                        * *right_block.offset(0 as core::ffi::c_int as isize)
                    + 16 as core::ffi::c_int >> 5 as core::ffi::c_int,
                grain_min,
                grain_max,
            );
            *dst_block.offset(1 as core::ffi::c_int as isize) = clamp(
                17 as core::ffi::c_int
                    * *left_block.offset(1 as core::ffi::c_int as isize)
                    + 27 as core::ffi::c_int
                        * *right_block.offset(1 as core::ffi::c_int as isize)
                    + 16 as core::ffi::c_int >> 5 as core::ffi::c_int,
                grain_min,
                grain_max,
            );
            left_block = left_block.offset(left_stride as isize);
            right_block = right_block.offset(right_stride as isize);
            dst_block = dst_block.offset(dst_stride as isize);
            height -= 1;
        }
        return;
    }
}
unsafe extern "C-unwind" fn hor_boundary_overlap(
    mut top_block: *mut core::ffi::c_int,
    mut top_stride: core::ffi::c_int,
    mut bottom_block: *mut core::ffi::c_int,
    mut bottom_stride: core::ffi::c_int,
    mut dst_block: *mut core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut grain_min: core::ffi::c_int,
    mut grain_max: core::ffi::c_int,
) {
    if height == 1 as core::ffi::c_int {
        while width != 0 {
            *dst_block = clamp(
                *top_block * 23 as core::ffi::c_int
                    + *bottom_block * 22 as core::ffi::c_int + 16 as core::ffi::c_int
                    >> 5 as core::ffi::c_int,
                grain_min,
                grain_max,
            );
            top_block = top_block.offset(1);
            bottom_block = bottom_block.offset(1);
            dst_block = dst_block.offset(1);
            width -= 1;
        }
        return;
    } else if height == 2 as core::ffi::c_int {
        while width != 0 {
            *dst_block.offset(0 as core::ffi::c_int as isize) = clamp(
                27 as core::ffi::c_int
                    * *top_block.offset(0 as core::ffi::c_int as isize)
                    + 17 as core::ffi::c_int
                        * *bottom_block.offset(0 as core::ffi::c_int as isize)
                    + 16 as core::ffi::c_int >> 5 as core::ffi::c_int,
                grain_min,
                grain_max,
            );
            *dst_block.offset(dst_stride as isize) = clamp(
                17 as core::ffi::c_int * *top_block.offset(top_stride as isize)
                    + 27 as core::ffi::c_int
                        * *bottom_block.offset(bottom_stride as isize)
                    + 16 as core::ffi::c_int >> 5 as core::ffi::c_int,
                grain_min,
                grain_max,
            );
            top_block = top_block.offset(1);
            bottom_block = bottom_block.offset(1);
            dst_block = dst_block.offset(1);
            width -= 1;
        }
        return;
    }
}
unsafe extern "C-unwind" fn add_film_grain_run(
    mut params: *const aom_film_grain_t,
    mut luma: *mut uint8_t,
    mut cb: *mut uint8_t,
    mut cr: *mut uint8_t,
    mut height: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut luma_stride: core::ffi::c_int,
    mut chroma_stride: core::ffi::c_int,
    mut use_high_bit_depth: core::ffi::c_int,
    mut chroma_subsamp_y: core::ffi::c_int,
    mut chroma_subsamp_x: core::ffi::c_int,
    mut mc_identity: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut pred_pos_luma: *mut *mut core::ffi::c_int = 0 as *mut *mut core::ffi::c_int;
    let mut pred_pos_chroma: *mut *mut core::ffi::c_int = 0
        as *mut *mut core::ffi::c_int;
    let mut luma_grain_block: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut cb_grain_block: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut cr_grain_block: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut y_line_buf: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut cb_line_buf: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut cr_line_buf: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut y_col_buf: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut cb_col_buf: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut cr_col_buf: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut scaling_lut: aom_grain_scaling_lut_t = aom_grain_scaling_lut_t {
        y: [0; 256],
        cb: [0; 256],
        cr: [0; 256],
    };
    memset(
        &raw mut scaling_lut as *mut aom_grain_scaling_lut_t as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_grain_scaling_lut_t>() as size_t,
    );
    let mut rng: aom_grain_rng_t = aom_grain_rng_t {
        random_register: 0,
    };
    rng.random_register = (*params).random_seed;
    let mut left_pad: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut right_pad: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut top_pad: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut bottom_pad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ar_padding: core::ffi::c_int = 3 as core::ffi::c_int;
    let chroma_subblock_size_y: core::ffi::c_int = luma_subblock_size_y
        >> chroma_subsamp_y;
    let chroma_subblock_size_x: core::ffi::c_int = luma_subblock_size_x
        >> chroma_subsamp_x;
    let mut luma_block_size_y: core::ffi::c_int = top_pad
        + 2 as core::ffi::c_int * ar_padding
        + luma_subblock_size_y * 2 as core::ffi::c_int + bottom_pad;
    let mut luma_block_size_x: core::ffi::c_int = left_pad
        + 2 as core::ffi::c_int * ar_padding
        + luma_subblock_size_x * 2 as core::ffi::c_int
        + 2 as core::ffi::c_int * ar_padding + right_pad;
    let mut chroma_block_size_y: core::ffi::c_int = top_pad
        + (2 as core::ffi::c_int >> chroma_subsamp_y) * ar_padding
        + chroma_subblock_size_y * 2 as core::ffi::c_int + bottom_pad;
    let mut chroma_block_size_x: core::ffi::c_int = left_pad
        + (2 as core::ffi::c_int >> chroma_subsamp_x) * ar_padding
        + chroma_subblock_size_x * 2 as core::ffi::c_int
        + (2 as core::ffi::c_int >> chroma_subsamp_x) * ar_padding + right_pad;
    let mut luma_grain_stride: core::ffi::c_int = luma_block_size_x;
    let mut chroma_grain_stride: core::ffi::c_int = chroma_block_size_x;
    let mut overlap: core::ffi::c_int = (*params).overlap_flag;
    let mut bit_depth: core::ffi::c_int = (*params).bit_depth as core::ffi::c_int;
    let mut grain_min: core::ffi::c_int = -((1 as core::ffi::c_int)
        << bit_depth - 1 as core::ffi::c_int);
    let mut grain_max: core::ffi::c_int = ((1 as core::ffi::c_int)
        << bit_depth - 1 as core::ffi::c_int) - 1 as core::ffi::c_int;
    if !init_arrays(
        params,
        luma_stride,
        chroma_stride,
        &raw mut pred_pos_luma,
        &raw mut pred_pos_chroma,
        &raw mut luma_grain_block,
        &raw mut cb_grain_block,
        &raw mut cr_grain_block,
        &raw mut y_line_buf,
        &raw mut cb_line_buf,
        &raw mut cr_line_buf,
        &raw mut y_col_buf,
        &raw mut cb_col_buf,
        &raw mut cr_col_buf,
        luma_block_size_y * luma_block_size_x,
        chroma_block_size_y * chroma_block_size_x,
        chroma_subsamp_y,
        chroma_subsamp_x,
    ) {
        return -(1 as core::ffi::c_int);
    }
    generate_luma_grain_block(
        params,
        &raw mut rng,
        pred_pos_luma,
        luma_grain_block,
        luma_block_size_y,
        luma_block_size_x,
        luma_grain_stride,
        left_pad,
        top_pad,
        right_pad,
        bottom_pad,
    );
    if !generate_chroma_grain_blocks(
        params,
        &raw mut rng,
        pred_pos_chroma,
        luma_grain_block,
        cb_grain_block,
        cr_grain_block,
        luma_grain_stride,
        chroma_block_size_y,
        chroma_block_size_x,
        chroma_grain_stride,
        left_pad,
        top_pad,
        right_pad,
        bottom_pad,
        chroma_subsamp_y,
        chroma_subsamp_x,
    ) {
        return -(1 as core::ffi::c_int);
    }
    init_scaling_function(
        ((*params).scaling_points_y).as_ptr() as *const [core::ffi::c_int; 2],
        (*params).num_y_points,
        (scaling_lut.y).as_mut_ptr(),
    );
    if (*params).chroma_scaling_from_luma != 0 {
        memcpy(
            (scaling_lut.cb).as_mut_ptr() as *mut core::ffi::c_void,
            (scaling_lut.y).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[core::ffi::c_int; 256]>() as size_t,
        );
        memcpy(
            (scaling_lut.cr).as_mut_ptr() as *mut core::ffi::c_void,
            (scaling_lut.y).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[core::ffi::c_int; 256]>() as size_t,
        );
    } else {
        init_scaling_function(
            ((*params).scaling_points_cb).as_ptr() as *const [core::ffi::c_int; 2],
            (*params).num_cb_points,
            (scaling_lut.cb).as_mut_ptr(),
        );
        init_scaling_function(
            ((*params).scaling_points_cr).as_ptr() as *const [core::ffi::c_int; 2],
            (*params).num_cr_points,
            (scaling_lut.cr).as_mut_ptr(),
        );
    }
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height / 2 as core::ffi::c_int {
        init_random_generator(
            &raw mut rng,
            y * 2 as core::ffi::c_int,
            (*params).random_seed,
        );
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < width / 2 as core::ffi::c_int {
            let mut offset_y: core::ffi::c_int = get_random_number(
                &raw mut rng,
                8 as core::ffi::c_int,
            );
            let mut offset_x: core::ffi::c_int = offset_y >> 4 as core::ffi::c_int
                & 15 as core::ffi::c_int;
            offset_y &= 15 as core::ffi::c_int;
            let mut luma_offset_y: core::ffi::c_int = left_pad
                + 2 as core::ffi::c_int * ar_padding
                + (offset_y << 1 as core::ffi::c_int);
            let mut luma_offset_x: core::ffi::c_int = top_pad
                + 2 as core::ffi::c_int * ar_padding
                + (offset_x << 1 as core::ffi::c_int);
            let mut chroma_offset_y: core::ffi::c_int = top_pad
                + (2 as core::ffi::c_int >> chroma_subsamp_y) * ar_padding
                + offset_y * (2 as core::ffi::c_int >> chroma_subsamp_y);
            let mut chroma_offset_x: core::ffi::c_int = left_pad
                + (2 as core::ffi::c_int >> chroma_subsamp_x) * ar_padding
                + offset_x * (2 as core::ffi::c_int >> chroma_subsamp_x);
            if overlap != 0 && x != 0 {
                ver_boundary_overlap(
                    y_col_buf,
                    2 as core::ffi::c_int,
                    luma_grain_block
                        .offset((luma_offset_y * luma_grain_stride) as isize)
                        .offset(luma_offset_x as isize),
                    luma_grain_stride,
                    y_col_buf,
                    2 as core::ffi::c_int,
                    2 as core::ffi::c_int,
                    if (luma_subblock_size_y + 2 as core::ffi::c_int)
                        < height - (y << 1 as core::ffi::c_int)
                    {
                        luma_subblock_size_y + 2 as core::ffi::c_int
                    } else {
                        height - (y << 1 as core::ffi::c_int)
                    },
                    grain_min,
                    grain_max,
                );
                ver_boundary_overlap(
                    cb_col_buf,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    cb_grain_block
                        .offset((chroma_offset_y * chroma_grain_stride) as isize)
                        .offset(chroma_offset_x as isize),
                    chroma_grain_stride,
                    cb_col_buf,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    if chroma_subblock_size_y
                        + (2 as core::ffi::c_int >> chroma_subsamp_y)
                        < height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    {
                        chroma_subblock_size_y
                            + (2 as core::ffi::c_int >> chroma_subsamp_y)
                    } else {
                        height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    },
                    grain_min,
                    grain_max,
                );
                ver_boundary_overlap(
                    cr_col_buf,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    cr_grain_block
                        .offset((chroma_offset_y * chroma_grain_stride) as isize)
                        .offset(chroma_offset_x as isize),
                    chroma_grain_stride,
                    cr_col_buf,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    if chroma_subblock_size_y
                        + (2 as core::ffi::c_int >> chroma_subsamp_y)
                        < height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    {
                        chroma_subblock_size_y
                            + (2 as core::ffi::c_int >> chroma_subsamp_y)
                    } else {
                        height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    },
                    grain_min,
                    grain_max,
                );
                let mut i: core::ffi::c_int = if y != 0 {
                    1 as core::ffi::c_int
                } else {
                    0 as core::ffi::c_int
                };
                if use_high_bit_depth != 0 {
                    add_noise_to_block_hbd(
                        params,
                        &raw mut scaling_lut,
                        (luma as *mut uint16_t)
                            .offset(
                                ((y + i << 1 as core::ffi::c_int) * luma_stride) as isize,
                            )
                            .offset((x << 1 as core::ffi::c_int) as isize),
                        (cb as *mut uint16_t)
                            .offset(
                                ((y + i << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        (cr as *mut uint16_t)
                            .offset(
                                ((y + i << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        luma_stride,
                        chroma_stride,
                        y_col_buf.offset((i * 4 as core::ffi::c_int) as isize),
                        cb_col_buf
                            .offset(
                                (i * (2 as core::ffi::c_int - chroma_subsamp_y)
                                    * (2 as core::ffi::c_int - chroma_subsamp_x)) as isize,
                            ),
                        cr_col_buf
                            .offset(
                                (i * (2 as core::ffi::c_int - chroma_subsamp_y)
                                    * (2 as core::ffi::c_int - chroma_subsamp_x)) as isize,
                            ),
                        2 as core::ffi::c_int,
                        2 as core::ffi::c_int - chroma_subsamp_x,
                        (if (luma_subblock_size_y >> 1 as core::ffi::c_int)
                            < height / 2 as core::ffi::c_int - y
                        {
                            luma_subblock_size_y >> 1 as core::ffi::c_int
                        } else {
                            height / 2 as core::ffi::c_int - y
                        }) - i,
                        1 as core::ffi::c_int,
                        bit_depth,
                        chroma_subsamp_y,
                        chroma_subsamp_x,
                        mc_identity,
                    );
                } else {
                    add_noise_to_block(
                        params,
                        &raw mut scaling_lut,
                        luma
                            .offset(
                                ((y + i << 1 as core::ffi::c_int) * luma_stride) as isize,
                            )
                            .offset((x << 1 as core::ffi::c_int) as isize),
                        cb
                            .offset(
                                ((y + i << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        cr
                            .offset(
                                ((y + i << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        luma_stride,
                        chroma_stride,
                        y_col_buf.offset((i * 4 as core::ffi::c_int) as isize),
                        cb_col_buf
                            .offset(
                                (i * (2 as core::ffi::c_int - chroma_subsamp_y)
                                    * (2 as core::ffi::c_int - chroma_subsamp_x)) as isize,
                            ),
                        cr_col_buf
                            .offset(
                                (i * (2 as core::ffi::c_int - chroma_subsamp_y)
                                    * (2 as core::ffi::c_int - chroma_subsamp_x)) as isize,
                            ),
                        2 as core::ffi::c_int,
                        2 as core::ffi::c_int - chroma_subsamp_x,
                        (if (luma_subblock_size_y >> 1 as core::ffi::c_int)
                            < height / 2 as core::ffi::c_int - y
                        {
                            luma_subblock_size_y >> 1 as core::ffi::c_int
                        } else {
                            height / 2 as core::ffi::c_int - y
                        }) - i,
                        1 as core::ffi::c_int,
                        bit_depth,
                        chroma_subsamp_y,
                        chroma_subsamp_x,
                        mc_identity,
                    );
                }
            }
            if overlap != 0 && y != 0 {
                if x != 0 {
                    hor_boundary_overlap(
                        y_line_buf.offset((x << 1 as core::ffi::c_int) as isize),
                        luma_stride,
                        y_col_buf,
                        2 as core::ffi::c_int,
                        y_line_buf.offset((x << 1 as core::ffi::c_int) as isize),
                        luma_stride,
                        2 as core::ffi::c_int,
                        2 as core::ffi::c_int,
                        grain_min,
                        grain_max,
                    );
                    hor_boundary_overlap(
                        cb_line_buf
                            .offset(
                                (x * (2 as core::ffi::c_int >> chroma_subsamp_x)) as isize,
                            ),
                        chroma_stride,
                        cb_col_buf,
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        cb_line_buf
                            .offset(
                                (x * (2 as core::ffi::c_int >> chroma_subsamp_x)) as isize,
                            ),
                        chroma_stride,
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        2 as core::ffi::c_int >> chroma_subsamp_y,
                        grain_min,
                        grain_max,
                    );
                    hor_boundary_overlap(
                        cr_line_buf
                            .offset(
                                (x * (2 as core::ffi::c_int >> chroma_subsamp_x)) as isize,
                            ),
                        chroma_stride,
                        cr_col_buf,
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        cr_line_buf
                            .offset(
                                (x * (2 as core::ffi::c_int >> chroma_subsamp_x)) as isize,
                            ),
                        chroma_stride,
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        2 as core::ffi::c_int >> chroma_subsamp_y,
                        grain_min,
                        grain_max,
                    );
                }
                hor_boundary_overlap(
                    y_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) as isize,
                        ),
                    luma_stride,
                    luma_grain_block
                        .offset((luma_offset_y * luma_grain_stride) as isize)
                        .offset(luma_offset_x as isize)
                        .offset(
                            (if x != 0 {
                                2 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ),
                    luma_grain_stride,
                    y_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) as isize,
                        ),
                    luma_stride,
                    if luma_subblock_size_x
                        - ((if x != 0 {
                            1 as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }) << 1 as core::ffi::c_int)
                        < width
                            - ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int)
                    {
                        luma_subblock_size_x
                            - ((if x != 0 {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int)
                    } else {
                        width
                            - ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int)
                    },
                    2 as core::ffi::c_int,
                    grain_min,
                    grain_max,
                );
                hor_boundary_overlap(
                    cb_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_stride,
                    cb_grain_block
                        .offset((chroma_offset_y * chroma_grain_stride) as isize)
                        .offset(chroma_offset_x as isize)
                        .offset(
                            ((if x != 0 {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_grain_stride,
                    cb_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_stride,
                    if chroma_subblock_size_x
                        - ((if x != 0 {
                            1 as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }) << 1 as core::ffi::c_int - chroma_subsamp_x)
                        < width
                            - ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    {
                        chroma_subblock_size_x
                            - ((if x != 0 {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x)
                    } else {
                        width
                            - ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    },
                    2 as core::ffi::c_int >> chroma_subsamp_y,
                    grain_min,
                    grain_max,
                );
                hor_boundary_overlap(
                    cr_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_stride,
                    cr_grain_block
                        .offset((chroma_offset_y * chroma_grain_stride) as isize)
                        .offset(chroma_offset_x as isize)
                        .offset(
                            ((if x != 0 {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_grain_stride,
                    cr_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_stride,
                    if chroma_subblock_size_x
                        - ((if x != 0 {
                            1 as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }) << 1 as core::ffi::c_int - chroma_subsamp_x)
                        < width
                            - ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    {
                        chroma_subblock_size_x
                            - ((if x != 0 {
                                1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x)
                    } else {
                        width
                            - ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    },
                    2 as core::ffi::c_int >> chroma_subsamp_y,
                    grain_min,
                    grain_max,
                );
                if use_high_bit_depth != 0 {
                    add_noise_to_block_hbd(
                        params,
                        &raw mut scaling_lut,
                        (luma as *mut uint16_t)
                            .offset(
                                ((y << 1 as core::ffi::c_int) * luma_stride) as isize,
                            )
                            .offset((x << 1 as core::ffi::c_int) as isize),
                        (cb as *mut uint16_t)
                            .offset(
                                ((y << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        (cr as *mut uint16_t)
                            .offset(
                                ((y << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        luma_stride,
                        chroma_stride,
                        y_line_buf.offset((x << 1 as core::ffi::c_int) as isize),
                        cb_line_buf
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        cr_line_buf
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        luma_stride,
                        chroma_stride,
                        1 as core::ffi::c_int,
                        if (luma_subblock_size_x >> 1 as core::ffi::c_int)
                            < width / 2 as core::ffi::c_int - x
                        {
                            luma_subblock_size_x >> 1 as core::ffi::c_int
                        } else {
                            width / 2 as core::ffi::c_int - x
                        },
                        bit_depth,
                        chroma_subsamp_y,
                        chroma_subsamp_x,
                        mc_identity,
                    );
                } else {
                    add_noise_to_block(
                        params,
                        &raw mut scaling_lut,
                        luma
                            .offset(
                                ((y << 1 as core::ffi::c_int) * luma_stride) as isize,
                            )
                            .offset((x << 1 as core::ffi::c_int) as isize),
                        cb
                            .offset(
                                ((y << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        cr
                            .offset(
                                ((y << 1 as core::ffi::c_int - chroma_subsamp_y)
                                    * chroma_stride) as isize,
                            )
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        luma_stride,
                        chroma_stride,
                        y_line_buf.offset((x << 1 as core::ffi::c_int) as isize),
                        cb_line_buf
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        cr_line_buf
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        luma_stride,
                        chroma_stride,
                        1 as core::ffi::c_int,
                        if (luma_subblock_size_x >> 1 as core::ffi::c_int)
                            < width / 2 as core::ffi::c_int - x
                        {
                            luma_subblock_size_x >> 1 as core::ffi::c_int
                        } else {
                            width / 2 as core::ffi::c_int - x
                        },
                        bit_depth,
                        chroma_subsamp_y,
                        chroma_subsamp_x,
                        mc_identity,
                    );
                }
            }
            let mut i_0: core::ffi::c_int = if overlap != 0 && y != 0 {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            };
            let mut j: core::ffi::c_int = if overlap != 0 && x != 0 {
                1 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            };
            if use_high_bit_depth != 0 {
                add_noise_to_block_hbd(
                    params,
                    &raw mut scaling_lut,
                    (luma as *mut uint16_t)
                        .offset(
                            ((y + i_0 << 1 as core::ffi::c_int) * luma_stride) as isize,
                        )
                        .offset((x + j << 1 as core::ffi::c_int) as isize),
                    (cb as *mut uint16_t)
                        .offset(
                            ((y + i_0 << 1 as core::ffi::c_int - chroma_subsamp_y)
                                * chroma_stride) as isize,
                        )
                        .offset(
                            (x + j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    (cr as *mut uint16_t)
                        .offset(
                            ((y + i_0 << 1 as core::ffi::c_int - chroma_subsamp_y)
                                * chroma_stride) as isize,
                        )
                        .offset(
                            (x + j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    luma_stride,
                    chroma_stride,
                    luma_grain_block
                        .offset(
                            ((luma_offset_y + (i_0 << 1 as core::ffi::c_int))
                                * luma_grain_stride) as isize,
                        )
                        .offset(luma_offset_x as isize)
                        .offset((j << 1 as core::ffi::c_int) as isize),
                    cb_grain_block
                        .offset(
                            ((chroma_offset_y
                                + (i_0 << 1 as core::ffi::c_int - chroma_subsamp_y))
                                * chroma_grain_stride) as isize,
                        )
                        .offset(chroma_offset_x as isize)
                        .offset(
                            (j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    cr_grain_block
                        .offset(
                            ((chroma_offset_y
                                + (i_0 << 1 as core::ffi::c_int - chroma_subsamp_y))
                                * chroma_grain_stride) as isize,
                        )
                        .offset(chroma_offset_x as isize)
                        .offset(
                            (j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    luma_grain_stride,
                    chroma_grain_stride,
                    (if (luma_subblock_size_y >> 1 as core::ffi::c_int)
                        < height / 2 as core::ffi::c_int - y
                    {
                        luma_subblock_size_y >> 1 as core::ffi::c_int
                    } else {
                        height / 2 as core::ffi::c_int - y
                    }) - i_0,
                    (if (luma_subblock_size_x >> 1 as core::ffi::c_int)
                        < width / 2 as core::ffi::c_int - x
                    {
                        luma_subblock_size_x >> 1 as core::ffi::c_int
                    } else {
                        width / 2 as core::ffi::c_int - x
                    }) - j,
                    bit_depth,
                    chroma_subsamp_y,
                    chroma_subsamp_x,
                    mc_identity,
                );
            } else {
                add_noise_to_block(
                    params,
                    &raw mut scaling_lut,
                    luma
                        .offset(
                            ((y + i_0 << 1 as core::ffi::c_int) * luma_stride) as isize,
                        )
                        .offset((x + j << 1 as core::ffi::c_int) as isize),
                    cb
                        .offset(
                            ((y + i_0 << 1 as core::ffi::c_int - chroma_subsamp_y)
                                * chroma_stride) as isize,
                        )
                        .offset(
                            (x + j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    cr
                        .offset(
                            ((y + i_0 << 1 as core::ffi::c_int - chroma_subsamp_y)
                                * chroma_stride) as isize,
                        )
                        .offset(
                            (x + j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    luma_stride,
                    chroma_stride,
                    luma_grain_block
                        .offset(
                            ((luma_offset_y + (i_0 << 1 as core::ffi::c_int))
                                * luma_grain_stride) as isize,
                        )
                        .offset(luma_offset_x as isize)
                        .offset((j << 1 as core::ffi::c_int) as isize),
                    cb_grain_block
                        .offset(
                            ((chroma_offset_y
                                + (i_0 << 1 as core::ffi::c_int - chroma_subsamp_y))
                                * chroma_grain_stride) as isize,
                        )
                        .offset(chroma_offset_x as isize)
                        .offset(
                            (j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    cr_grain_block
                        .offset(
                            ((chroma_offset_y
                                + (i_0 << 1 as core::ffi::c_int - chroma_subsamp_y))
                                * chroma_grain_stride) as isize,
                        )
                        .offset(chroma_offset_x as isize)
                        .offset(
                            (j << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    luma_grain_stride,
                    chroma_grain_stride,
                    (if (luma_subblock_size_y >> 1 as core::ffi::c_int)
                        < height / 2 as core::ffi::c_int - y
                    {
                        luma_subblock_size_y >> 1 as core::ffi::c_int
                    } else {
                        height / 2 as core::ffi::c_int - y
                    }) - i_0,
                    (if (luma_subblock_size_x >> 1 as core::ffi::c_int)
                        < width / 2 as core::ffi::c_int - x
                    {
                        luma_subblock_size_x >> 1 as core::ffi::c_int
                    } else {
                        width / 2 as core::ffi::c_int - x
                    }) - j,
                    bit_depth,
                    chroma_subsamp_y,
                    chroma_subsamp_x,
                    mc_identity,
                );
            }
            if overlap != 0 {
                if x != 0 {
                    copy_area(
                        y_col_buf
                            .offset(
                                (luma_subblock_size_y << 1 as core::ffi::c_int) as isize,
                            ),
                        2 as core::ffi::c_int,
                        y_line_buf.offset((x << 1 as core::ffi::c_int) as isize),
                        luma_stride,
                        2 as core::ffi::c_int,
                        2 as core::ffi::c_int,
                    );
                    copy_area(
                        cb_col_buf
                            .offset(
                                (chroma_subblock_size_y
                                    << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        cb_line_buf
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        chroma_stride,
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        2 as core::ffi::c_int >> chroma_subsamp_y,
                    );
                    copy_area(
                        cr_col_buf
                            .offset(
                                (chroma_subblock_size_y
                                    << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        cr_line_buf
                            .offset(
                                (x << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                            ),
                        chroma_stride,
                        2 as core::ffi::c_int >> chroma_subsamp_x,
                        2 as core::ffi::c_int >> chroma_subsamp_y,
                    );
                }
                copy_area(
                    luma_grain_block
                        .offset(
                            ((luma_offset_y + luma_subblock_size_y) * luma_grain_stride)
                                as isize,
                        )
                        .offset(luma_offset_x as isize)
                        .offset(
                            (if x != 0 {
                                2 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ),
                    luma_grain_stride,
                    y_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int) as isize,
                        ),
                    luma_stride,
                    (if luma_subblock_size_x < width - (x << 1 as core::ffi::c_int) {
                        luma_subblock_size_x
                    } else {
                        width - (x << 1 as core::ffi::c_int)
                    })
                        - (if x != 0 {
                            2 as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        }),
                    2 as core::ffi::c_int,
                );
                copy_area(
                    cb_grain_block
                        .offset(
                            ((chroma_offset_y + chroma_subblock_size_y)
                                * chroma_grain_stride) as isize,
                        )
                        .offset(chroma_offset_x as isize)
                        .offset(
                            (if x != 0 {
                                2 as core::ffi::c_int >> chroma_subsamp_x
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ),
                    chroma_grain_stride,
                    cb_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_stride,
                    (if chroma_subblock_size_x
                        < width - (x << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    {
                        chroma_subblock_size_x
                    } else {
                        width - (x << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    })
                        - (if x != 0 {
                            2 as core::ffi::c_int >> chroma_subsamp_x
                        } else {
                            0 as core::ffi::c_int
                        }),
                    2 as core::ffi::c_int >> chroma_subsamp_y,
                );
                copy_area(
                    cr_grain_block
                        .offset(
                            ((chroma_offset_y + chroma_subblock_size_y)
                                * chroma_grain_stride) as isize,
                        )
                        .offset(chroma_offset_x as isize)
                        .offset(
                            (if x != 0 {
                                2 as core::ffi::c_int >> chroma_subsamp_x
                            } else {
                                0 as core::ffi::c_int
                            }) as isize,
                        ),
                    chroma_grain_stride,
                    cr_line_buf
                        .offset(
                            ((if x != 0 {
                                x + 1 as core::ffi::c_int
                            } else {
                                0 as core::ffi::c_int
                            }) << 1 as core::ffi::c_int - chroma_subsamp_x) as isize,
                        ),
                    chroma_stride,
                    (if chroma_subblock_size_x
                        < width - (x << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    {
                        chroma_subblock_size_x
                    } else {
                        width - (x << 1 as core::ffi::c_int) >> chroma_subsamp_x
                    })
                        - (if x != 0 {
                            2 as core::ffi::c_int >> chroma_subsamp_x
                        } else {
                            0 as core::ffi::c_int
                        }),
                    2 as core::ffi::c_int >> chroma_subsamp_y,
                );
                copy_area(
                    luma_grain_block
                        .offset((luma_offset_y * luma_grain_stride) as isize)
                        .offset(luma_offset_x as isize)
                        .offset(luma_subblock_size_x as isize),
                    luma_grain_stride,
                    y_col_buf,
                    2 as core::ffi::c_int,
                    2 as core::ffi::c_int,
                    if (luma_subblock_size_y + 2 as core::ffi::c_int)
                        < height - (y << 1 as core::ffi::c_int)
                    {
                        luma_subblock_size_y + 2 as core::ffi::c_int
                    } else {
                        height - (y << 1 as core::ffi::c_int)
                    },
                );
                copy_area(
                    cb_grain_block
                        .offset((chroma_offset_y * chroma_grain_stride) as isize)
                        .offset(chroma_offset_x as isize)
                        .offset(chroma_subblock_size_x as isize),
                    chroma_grain_stride,
                    cb_col_buf,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    if chroma_subblock_size_y
                        + (2 as core::ffi::c_int >> chroma_subsamp_y)
                        < height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    {
                        chroma_subblock_size_y
                            + (2 as core::ffi::c_int >> chroma_subsamp_y)
                    } else {
                        height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    },
                );
                copy_area(
                    cr_grain_block
                        .offset((chroma_offset_y * chroma_grain_stride) as isize)
                        .offset(chroma_offset_x as isize)
                        .offset(chroma_subblock_size_x as isize),
                    chroma_grain_stride,
                    cr_col_buf,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    2 as core::ffi::c_int >> chroma_subsamp_x,
                    if chroma_subblock_size_y
                        + (2 as core::ffi::c_int >> chroma_subsamp_y)
                        < height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    {
                        chroma_subblock_size_y
                            + (2 as core::ffi::c_int >> chroma_subsamp_y)
                    } else {
                        height - (y << 1 as core::ffi::c_int) >> chroma_subsamp_y
                    },
                );
            }
            x += luma_subblock_size_x >> 1 as core::ffi::c_int;
        }
        y += luma_subblock_size_y >> 1 as core::ffi::c_int;
    }
    dealloc_arrays(
        params,
        &raw mut pred_pos_luma,
        &raw mut pred_pos_chroma,
        &raw mut luma_grain_block,
        &raw mut cb_grain_block,
        &raw mut cr_grain_block,
        &raw mut y_line_buf,
        &raw mut cb_line_buf,
        &raw mut cr_line_buf,
        &raw mut y_col_buf,
        &raw mut cb_col_buf,
        &raw mut cr_col_buf,
    );
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_add_film_grain(
    mut params: *const aom_film_grain_t,
    mut src: *const aom_image_t,
    mut dst: *mut aom_image_t,
) -> core::ffi::c_int {
    let mut luma: *mut uint8_t = 0 as *mut uint8_t;
    let mut cb: *mut uint8_t = 0 as *mut uint8_t;
    let mut cr: *mut uint8_t = 0 as *mut uint8_t;
    let mut height: core::ffi::c_int = 0;
    let mut width: core::ffi::c_int = 0;
    let mut luma_stride: core::ffi::c_int = 0;
    let mut chroma_stride: core::ffi::c_int = 0;
    let mut use_high_bit_depth: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut chroma_subsamp_x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut chroma_subsamp_y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mc_identity: core::ffi::c_int = if (*src).mc as core::ffi::c_uint
        == AOM_CICP_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    match (*src).fmt as core::ffi::c_uint {
        258 => {
            use_high_bit_depth = 0 as core::ffi::c_int;
            chroma_subsamp_x = 1 as core::ffi::c_int;
            chroma_subsamp_y = 1 as core::ffi::c_int;
        }
        2306 => {
            use_high_bit_depth = 1 as core::ffi::c_int;
            chroma_subsamp_x = 1 as core::ffi::c_int;
            chroma_subsamp_y = 1 as core::ffi::c_int;
        }
        262 => {
            use_high_bit_depth = 0 as core::ffi::c_int;
            chroma_subsamp_x = 0 as core::ffi::c_int;
            chroma_subsamp_y = 0 as core::ffi::c_int;
        }
        2310 => {
            use_high_bit_depth = 1 as core::ffi::c_int;
            chroma_subsamp_x = 0 as core::ffi::c_int;
            chroma_subsamp_y = 0 as core::ffi::c_int;
        }
        261 => {
            use_high_bit_depth = 0 as core::ffi::c_int;
            chroma_subsamp_x = 1 as core::ffi::c_int;
            chroma_subsamp_y = 0 as core::ffi::c_int;
        }
        2309 => {
            use_high_bit_depth = 1 as core::ffi::c_int;
            chroma_subsamp_x = 1 as core::ffi::c_int;
            chroma_subsamp_y = 0 as core::ffi::c_int;
        }
        _ => {
            fprintf(
                __stderrp,
                b"Film grain error: input format is not supported!\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return -(1 as core::ffi::c_int);
        }
    }
    (*dst).fmt = (*src).fmt;
    (*dst).bit_depth = (*src).bit_depth;
    (*dst).r_w = (*src).r_w;
    (*dst).r_h = (*src).r_h;
    (*dst).d_w = (*src).d_w;
    (*dst).d_h = (*src).d_h;
    (*dst).cp = (*src).cp;
    (*dst).tc = (*src).tc;
    (*dst).mc = (*src).mc;
    (*dst).monochrome = (*src).monochrome;
    (*dst).csp = (*src).csp;
    (*dst).range = (*src).range;
    (*dst).x_chroma_shift = (*src).x_chroma_shift;
    (*dst).y_chroma_shift = (*src).y_chroma_shift;
    (*dst).temporal_id = (*src).temporal_id;
    (*dst).spatial_id = (*src).spatial_id;
    width = (if ((*src).d_w).wrapping_rem(2 as core::ffi::c_uint) != 0 {
        ((*src).d_w).wrapping_add(1 as core::ffi::c_uint)
    } else {
        (*src).d_w
    }) as core::ffi::c_int;
    height = (if ((*src).d_h).wrapping_rem(2 as core::ffi::c_uint) != 0 {
        ((*src).d_h).wrapping_add(1 as core::ffi::c_uint)
    } else {
        (*src).d_h
    }) as core::ffi::c_int;
    copy_rect(
        (*src).planes[AOM_PLANE_Y as usize],
        (*src).stride[AOM_PLANE_Y as usize],
        (*dst).planes[AOM_PLANE_Y as usize],
        (*dst).stride[AOM_PLANE_Y as usize],
        (*src).d_w as core::ffi::c_int,
        (*src).d_h as core::ffi::c_int,
        use_high_bit_depth,
    );
    extend_even(
        (*dst).planes[AOM_PLANE_Y as usize],
        (*dst).stride[AOM_PLANE_Y as usize],
        (*src).d_w as core::ffi::c_int,
        (*src).d_h as core::ffi::c_int,
        use_high_bit_depth,
    );
    if (*src).monochrome == 0 {
        copy_rect(
            (*src).planes[AOM_PLANE_U as usize],
            (*src).stride[AOM_PLANE_U as usize],
            (*dst).planes[AOM_PLANE_U as usize],
            (*dst).stride[AOM_PLANE_U as usize],
            width >> chroma_subsamp_x,
            height >> chroma_subsamp_y,
            use_high_bit_depth,
        );
        copy_rect(
            (*src).planes[AOM_PLANE_V as usize],
            (*src).stride[AOM_PLANE_V as usize],
            (*dst).planes[AOM_PLANE_V as usize],
            (*dst).stride[AOM_PLANE_V as usize],
            width >> chroma_subsamp_x,
            height >> chroma_subsamp_y,
            use_high_bit_depth,
        );
    }
    luma = (*dst).planes[AOM_PLANE_Y as usize] as *mut uint8_t;
    cb = (*dst).planes[AOM_PLANE_U as usize] as *mut uint8_t;
    cr = (*dst).planes[AOM_PLANE_V as usize] as *mut uint8_t;
    luma_stride = (*dst).stride[AOM_PLANE_Y as usize] >> use_high_bit_depth;
    chroma_stride = (*dst).stride[AOM_PLANE_U as usize] >> use_high_bit_depth;
    return add_film_grain_run(
        params,
        luma,
        cb,
        cr,
        height,
        width,
        luma_stride,
        chroma_stride,
        use_high_bit_depth,
        chroma_subsamp_y,
        chroma_subsamp_x,
        mc_identity,
    );
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;

extern "C" {
    fn fprintf(_: *mut FILE, _: *const core::ffi::c_char, ...) -> core::ffi::c_int;
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
