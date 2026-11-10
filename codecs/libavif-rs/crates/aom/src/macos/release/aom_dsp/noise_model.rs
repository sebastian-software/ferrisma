/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/noise_model.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type __sFILEX;
    pub type image_pyramid;
    pub type corner_list;
    pub type aom_noise_tx_t;
    static mut __stderrp: *mut FILE;
    fn aom_noise_tx_malloc(block_size: core::ffi::c_int) -> *mut aom_noise_tx_t;
    fn aom_noise_tx_free(aom_noise_tx: *mut aom_noise_tx_t);
    fn aom_noise_tx_forward(
        aom_noise_tx: *mut aom_noise_tx_t,
        data: *const core::ffi::c_float,
    );
    fn aom_noise_tx_filter(
        aom_noise_tx: *mut aom_noise_tx_t,
        psd: *const core::ffi::c_float,
    );
    fn aom_noise_tx_inverse(
        aom_noise_tx: *mut aom_noise_tx_t,
        data: *mut core::ffi::c_float,
    );
    fn aom_noise_psd_get_default_value(
        block_size: core::ffi::c_int,
        factor: core::ffi::c_float,
    ) -> core::ffi::c_float;
    fn aom_normalized_cross_correlation(
        a: *const core::ffi::c_double,
        b: *const core::ffi::c_double,
        n: core::ffi::c_int,
    ) -> core::ffi::c_double;
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type uintptr_t = usize;
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
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_equation_system_t {
    pub A: *mut core::ffi::c_double,
    pub b: *mut core::ffi::c_double,
    pub x: *mut core::ffi::c_double,
    pub n: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_noise_strength_lut_t {
    pub points: *mut [core::ffi::c_double; 2],
    pub num_points: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_noise_strength_solver_t {
    pub eqns: aom_equation_system_t,
    pub min_intensity: core::ffi::c_double,
    pub max_intensity: core::ffi::c_double,
    pub num_bins: core::ffi::c_int,
    pub num_equations: core::ffi::c_int,
    pub total: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_flat_block_finder_t {
    pub AtA_inv: *mut core::ffi::c_double,
    pub A: *mut core::ffi::c_double,
    pub num_params: core::ffi::c_int,
    pub block_size: core::ffi::c_int,
    pub normalization: core::ffi::c_double,
    pub use_highbd: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct index_and_score_t {
    pub index: core::ffi::c_int,
    pub score: core::ffi::c_float,
}
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const AOM_NOISE_SHAPE_SQUARE: C2RustUnnamed_11 = 1;
pub const AOM_NOISE_SHAPE_DIAMOND: C2RustUnnamed_11 = 0;
pub type aom_noise_shape = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_noise_model_params_t {
    pub shape: aom_noise_shape,
    pub lag: core::ffi::c_int,
    pub bit_depth: core::ffi::c_int,
    pub use_highbd: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_noise_state_t {
    pub eqns: aom_equation_system_t,
    pub strength_solver: aom_noise_strength_solver_t,
    pub num_observations: core::ffi::c_int,
    pub ar_gain: core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_noise_model_t {
    pub params: aom_noise_model_params_t,
    pub combined_state: [aom_noise_state_t; 3],
    pub latest_state: [aom_noise_state_t; 3],
    pub coords: *mut [core::ffi::c_int; 2],
    pub n: core::ffi::c_int,
    pub bit_depth: core::ffi::c_int,
}
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const AOM_NOISE_STATUS_INTERNAL_ERROR: C2RustUnnamed_12 = 4;
pub const AOM_NOISE_STATUS_DIFFERENT_NOISE_TYPE: C2RustUnnamed_12 = 3;
pub const AOM_NOISE_STATUS_INSUFFICIENT_FLAT_BLOCKS: C2RustUnnamed_12 = 2;
pub const AOM_NOISE_STATUS_INVALID_ARGUMENT: C2RustUnnamed_12 = 1;
pub const AOM_NOISE_STATUS_OK: C2RustUnnamed_12 = 0;
pub type aom_noise_status_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_denoise_and_model_t {
    pub block_size: core::ffi::c_int,
    pub bit_depth: core::ffi::c_int,
    pub noise_level: core::ffi::c_float,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
    pub num_blocks_w: core::ffi::c_int,
    pub num_blocks_h: core::ffi::c_int,
    pub noise_psd: [*mut core::ffi::c_float; 3],
    pub denoised: [*mut uint8_t; 3],
    pub flat_blocks: *mut uint8_t,
    pub flat_block_finder: aom_flat_block_finder_t,
    pub noise_model: aom_noise_model_t,
}
#[inline(always)]
unsafe extern "C-unwind" fn __inline_isnanf(mut __x: core::ffi::c_float) -> core::ffi::c_int {
    return (__x != __x) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C-unwind" fn __inline_isnand(mut __x: core::ffi::c_double) -> core::ffi::c_int {
    return (__x != __x) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C-unwind" fn __inline_isnanl(mut __x: core::ffi::c_double) -> core::ffi::c_int {
    return (__x != __x) as core::ffi::c_int;
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const PI: core::ffi::c_double = 3.141592653589793238462643383279502884f64;
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
static mut TINY_NEAR_ZERO: core::ffi::c_double = 1.0E-16f64;
#[inline]
unsafe extern "C-unwind" fn linsolve(
    mut n: core::ffi::c_int,
    mut A: *mut core::ffi::c_double,
    mut stride: core::ffi::c_int,
    mut b: *mut core::ffi::c_double,
    mut x: *mut core::ffi::c_double,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut c: core::ffi::c_double = 0.;
    k = 0 as core::ffi::c_int;
    while k < n - 1 as core::ffi::c_int {
        i = n - 1 as core::ffi::c_int;
        while i > k {
            if fabs(*A.offset(((i - 1 as core::ffi::c_int) * stride + k) as isize))
                < fabs(*A.offset((i * stride + k) as isize))
            {
                j = 0 as core::ffi::c_int;
                while j < n {
                    c = *A.offset((i * stride + j) as isize);
                    *A.offset((i * stride + j) as isize) = *A
                        .offset(((i - 1 as core::ffi::c_int) * stride + j) as isize);
                    *A.offset(((i - 1 as core::ffi::c_int) * stride + j) as isize) = c;
                    j += 1;
                }
                c = *b.offset(i as isize);
                *b.offset(i as isize) = *b.offset((i - 1 as core::ffi::c_int) as isize);
                *b.offset((i - 1 as core::ffi::c_int) as isize) = c;
            }
            i -= 1;
        }
        i = k;
        while i < n - 1 as core::ffi::c_int {
            if fabs(*A.offset((k * stride + k) as isize)) < TINY_NEAR_ZERO {
                return 0 as core::ffi::c_int;
            }
            c = *A.offset(((i + 1 as core::ffi::c_int) * stride + k) as isize)
                / *A.offset((k * stride + k) as isize);
            j = 0 as core::ffi::c_int;
            while j < n {
                *A.offset(((i + 1 as core::ffi::c_int) * stride + j) as isize)
                    -= c * *A.offset((k * stride + j) as isize);
                j += 1;
            }
            *b.offset((i + 1 as core::ffi::c_int) as isize) -= c * *b.offset(k as isize);
            i += 1;
        }
        k += 1;
    }
    i = n - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        if fabs(*A.offset((i * stride + i) as isize)) < TINY_NEAR_ZERO {
            return 0 as core::ffi::c_int;
        }
        c = 0 as core::ffi::c_int as core::ffi::c_double;
        j = i + 1 as core::ffi::c_int;
        while j <= n - 1 as core::ffi::c_int {
            c += *A.offset((i * stride + j) as isize) * *x.offset(j as isize);
            j += 1;
        }
        *x.offset(i as isize) = (*b.offset(i as isize) - c)
            / *A.offset((i * stride + i) as isize);
        i -= 1;
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn multiply_mat(
    mut m1: *const core::ffi::c_double,
    mut m2: *const core::ffi::c_double,
    mut res: *mut core::ffi::c_double,
    m1_rows: core::ffi::c_int,
    inner_dim: core::ffi::c_int,
    m2_cols: core::ffi::c_int,
) {
    let mut sum: core::ffi::c_double = 0.;
    let mut row: core::ffi::c_int = 0;
    let mut col: core::ffi::c_int = 0;
    let mut inner: core::ffi::c_int = 0;
    row = 0 as core::ffi::c_int;
    while row < m1_rows {
        col = 0 as core::ffi::c_int;
        while col < m2_cols {
            sum = 0 as core::ffi::c_int as core::ffi::c_double;
            inner = 0 as core::ffi::c_int;
            while inner < inner_dim {
                sum
                    += *m1.offset((row * inner_dim + inner) as isize)
                        * *m2.offset((inner * m2_cols + col) as isize);
                inner += 1;
            }
            let fresh0 = res;
            res = res.offset(1);
            *fresh0 = sum;
            col += 1;
        }
        row += 1;
    }
}
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const kLowPolyNumParams: core::ffi::c_int = 3 as core::ffi::c_int;
static mut kMaxLag: core::ffi::c_int = 4 as core::ffi::c_int;
unsafe extern "C-unwind" fn get_block_mean_lowbd(
    mut data: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut x_o: core::ffi::c_int,
    mut y_o: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
) -> core::ffi::c_double {
    let max_h: core::ffi::c_int = if h - y_o < block_size {
        h - y_o
    } else {
        block_size
    };
    let max_w: core::ffi::c_int = if w - x_o < block_size {
        w - x_o
    } else {
        block_size
    };
    let mut block_mean: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < max_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < max_w {
            block_mean
                += *data.offset(((y_o + y) * stride + x_o + x) as isize)
                    as core::ffi::c_int as core::ffi::c_double;
            x += 1;
        }
        y += 1;
    }
    return block_mean / (max_w * max_h) as core::ffi::c_double;
}
unsafe extern "C-unwind" fn get_block_mean_highbd(
    mut data: *const uint16_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut x_o: core::ffi::c_int,
    mut y_o: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
) -> core::ffi::c_double {
    let max_h: core::ffi::c_int = if h - y_o < block_size {
        h - y_o
    } else {
        block_size
    };
    let max_w: core::ffi::c_int = if w - x_o < block_size {
        w - x_o
    } else {
        block_size
    };
    let mut block_mean: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < max_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < max_w {
            block_mean
                += *data.offset(((y_o + y) * stride + x_o + x) as isize)
                    as core::ffi::c_int as core::ffi::c_double;
            x += 1;
        }
        y += 1;
    }
    return block_mean / (max_w * max_h) as core::ffi::c_double;
}
#[inline]
unsafe extern "C-unwind" fn get_block_mean(
    mut data: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut x_o: core::ffi::c_int,
    mut y_o: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
) -> core::ffi::c_double {
    if use_highbd != 0 {
        return get_block_mean_highbd(
            data as *const uint16_t,
            w,
            h,
            stride,
            x_o,
            y_o,
            block_size,
        );
    }
    return get_block_mean_lowbd(data, w, h, stride, x_o, y_o, block_size);
}
unsafe extern "C-unwind" fn get_noise_var_lowbd(
    mut data: *const uint8_t,
    mut denoised: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut x_o: core::ffi::c_int,
    mut y_o: core::ffi::c_int,
    mut block_size_x: core::ffi::c_int,
    mut block_size_y: core::ffi::c_int,
) -> core::ffi::c_double {
    let max_h: core::ffi::c_int = if h - y_o < block_size_y {
        h - y_o
    } else {
        block_size_y
    };
    let max_w: core::ffi::c_int = if w - x_o < block_size_x {
        w - x_o
    } else {
        block_size_x
    };
    let mut noise_var: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut noise_mean: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < max_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < max_w {
            let mut noise: core::ffi::c_double = *data
                .offset(((y_o + y) * stride + x_o + x) as isize) as core::ffi::c_double
                - *denoised.offset(((y_o + y) * stride + x_o + x) as isize)
                    as core::ffi::c_int as core::ffi::c_double;
            noise_mean += noise;
            noise_var += noise * noise;
            x += 1;
        }
        y += 1;
    }
    noise_mean /= (max_w * max_h) as core::ffi::c_double;
    return noise_var / (max_w * max_h) as core::ffi::c_double - noise_mean * noise_mean;
}
unsafe extern "C-unwind" fn get_noise_var_highbd(
    mut data: *const uint16_t,
    mut denoised: *const uint16_t,
    mut stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut x_o: core::ffi::c_int,
    mut y_o: core::ffi::c_int,
    mut block_size_x: core::ffi::c_int,
    mut block_size_y: core::ffi::c_int,
) -> core::ffi::c_double {
    let max_h: core::ffi::c_int = if h - y_o < block_size_y {
        h - y_o
    } else {
        block_size_y
    };
    let max_w: core::ffi::c_int = if w - x_o < block_size_x {
        w - x_o
    } else {
        block_size_x
    };
    let mut noise_var: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut noise_mean: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < max_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < max_w {
            let mut noise: core::ffi::c_double = *data
                .offset(((y_o + y) * stride + x_o + x) as isize) as core::ffi::c_double
                - *denoised.offset(((y_o + y) * stride + x_o + x) as isize)
                    as core::ffi::c_int as core::ffi::c_double;
            noise_mean += noise;
            noise_var += noise * noise;
            x += 1;
        }
        y += 1;
    }
    noise_mean /= (max_w * max_h) as core::ffi::c_double;
    return noise_var / (max_w * max_h) as core::ffi::c_double - noise_mean * noise_mean;
}
#[inline]
unsafe extern "C-unwind" fn get_noise_var(
    mut data: *const uint8_t,
    mut denoised: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut x_o: core::ffi::c_int,
    mut y_o: core::ffi::c_int,
    mut block_size_x: core::ffi::c_int,
    mut block_size_y: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
) -> core::ffi::c_double {
    if use_highbd != 0 {
        return get_noise_var_highbd(
            data as *const uint16_t,
            denoised as *const uint16_t,
            w,
            h,
            stride,
            x_o,
            y_o,
            block_size_x,
            block_size_y,
        );
    }
    return get_noise_var_lowbd(
        data,
        denoised,
        w,
        h,
        stride,
        x_o,
        y_o,
        block_size_x,
        block_size_y,
    );
}
unsafe extern "C-unwind" fn equation_system_clear(mut eqns: *mut aom_equation_system_t) {
    let n: core::ffi::c_int = (*eqns).n;
    memset(
        (*eqns).A as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    );
    memset(
        (*eqns).x as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    );
    memset(
        (*eqns).b as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    );
}
unsafe extern "C-unwind" fn equation_system_copy(
    mut dst: *mut aom_equation_system_t,
    mut src: *const aom_equation_system_t,
) {
    let n: core::ffi::c_int = (*dst).n;
    memcpy(
        (*dst).A as *mut core::ffi::c_void,
        (*src).A as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    );
    memcpy(
        (*dst).x as *mut core::ffi::c_void,
        (*src).x as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    );
    memcpy(
        (*dst).b as *mut core::ffi::c_void,
        (*src).b as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    );
}
unsafe extern "C-unwind" fn equation_system_init(
    mut eqns: *mut aom_equation_system_t,
    mut n: core::ffi::c_int,
) -> core::ffi::c_int {
    (*eqns).A = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_double;
    (*eqns).b = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_double;
    (*eqns).x = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_double;
    (*eqns).n = n;
    if ((*eqns).A).is_null() || ((*eqns).b).is_null() || ((*eqns).x).is_null() {
        fprintf(
            __stderrp,
            b"Failed to allocate system of equations of size %d\n\0" as *const u8
                as *const core::ffi::c_char,
            n,
        );
        aom_free((*eqns).A as *mut core::ffi::c_void);
        aom_free((*eqns).b as *mut core::ffi::c_void);
        aom_free((*eqns).x as *mut core::ffi::c_void);
        memset(
            eqns as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_equation_system_t>() as size_t,
        );
        return 0 as core::ffi::c_int;
    }
    equation_system_clear(eqns);
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn equation_system_solve(
    mut eqns: *mut aom_equation_system_t,
) -> core::ffi::c_int {
    let n: core::ffi::c_int = (*eqns).n;
    let mut b: *mut core::ffi::c_double = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_double;
    let mut A: *mut core::ffi::c_double = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_double;
    let mut ret: core::ffi::c_int = 0 as core::ffi::c_int;
    if A.is_null() || b.is_null() {
        fprintf(
            __stderrp,
            b"Unable to allocate temp values of size %dx%d\n\0" as *const u8
                as *const core::ffi::c_char,
            n,
            n,
        );
        aom_free(b as *mut core::ffi::c_void);
        aom_free(A as *mut core::ffi::c_void);
        return 0 as core::ffi::c_int;
    }
    memcpy(
        A as *mut core::ffi::c_void,
        (*eqns).A as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    );
    memcpy(
        b as *mut core::ffi::c_void,
        (*eqns).b as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t),
    );
    ret = linsolve(n, A, (*eqns).n, b, (*eqns).x);
    aom_free(b as *mut core::ffi::c_void);
    aom_free(A as *mut core::ffi::c_void);
    if ret == 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n {
        if if ::core::mem::size_of::<core::ffi::c_float>() as usize
            == ::core::mem::size_of::<core::ffi::c_float>() as usize
        {
            __inline_isnanf(*((*eqns).x).offset(i as isize) as core::ffi::c_float)
        } else if ::core::mem::size_of::<core::ffi::c_float>() as usize
            == ::core::mem::size_of::<core::ffi::c_double>() as usize
        {
            __inline_isnand(
                *((*eqns).x).offset(i as isize) as core::ffi::c_float
                    as core::ffi::c_double,
            )
        } else {
            __inline_isnanl(
                (*((*eqns).x).offset(i as isize) as core::ffi::c_float) as core::ffi::c_double,
            )
        } != 0
        {
            return 0 as core::ffi::c_int;
        }
        i += 1;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn equation_system_add(
    mut dest: *mut aom_equation_system_t,
    mut src: *mut aom_equation_system_t,
) {
    let n: core::ffi::c_int = (*dest).n;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < n {
        j = 0 as core::ffi::c_int;
        while j < n {
            *((*dest).A).offset((i * n + j) as isize)
                += *((*src).A).offset((i * n + j) as isize);
            j += 1;
        }
        *((*dest).b).offset(i as isize) += *((*src).b).offset(i as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn equation_system_free(mut eqns: *mut aom_equation_system_t) {
    if eqns.is_null() {
        return;
    }
    aom_free((*eqns).A as *mut core::ffi::c_void);
    aom_free((*eqns).b as *mut core::ffi::c_void);
    aom_free((*eqns).x as *mut core::ffi::c_void);
    memset(
        eqns as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_equation_system_t>() as size_t,
    );
}
unsafe extern "C-unwind" fn noise_strength_solver_clear(
    mut solver: *mut aom_noise_strength_solver_t,
) {
    equation_system_clear(&raw mut (*solver).eqns);
    (*solver).num_equations = 0 as core::ffi::c_int;
    (*solver).total = 0 as core::ffi::c_int as core::ffi::c_double;
}
unsafe extern "C-unwind" fn noise_strength_solver_add(
    mut dest: *mut aom_noise_strength_solver_t,
    mut src: *mut aom_noise_strength_solver_t,
) {
    equation_system_add(&raw mut (*dest).eqns, &raw mut (*src).eqns);
    (*dest).num_equations += (*src).num_equations;
    (*dest).total += (*src).total;
}
unsafe extern "C-unwind" fn num_coeffs(params: aom_noise_model_params_t) -> core::ffi::c_int {
    let n: core::ffi::c_int = 2 as core::ffi::c_int * params.lag + 1 as core::ffi::c_int;
    match params.shape as core::ffi::c_int {
        0 => return params.lag * (params.lag + 1 as core::ffi::c_int),
        1 => return n * n / 2 as core::ffi::c_int,
        _ => {}
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn noise_state_init(
    mut state: *mut aom_noise_state_t,
    mut n: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) -> core::ffi::c_int {
    let kNumBins: core::ffi::c_int = 20 as core::ffi::c_int;
    if equation_system_init(&raw mut (*state).eqns, n) == 0 {
        fprintf(
            __stderrp,
            b"Failed initialization noise state with size %d\n\0" as *const u8
                as *const core::ffi::c_char,
            n,
        );
        return 0 as core::ffi::c_int;
    }
    (*state).ar_gain = 1.0f64;
    (*state).num_observations = 0 as core::ffi::c_int;
    return aom_noise_strength_solver_init(
        &raw mut (*state).strength_solver,
        kNumBins,
        bit_depth,
    );
}
unsafe extern "C-unwind" fn set_chroma_coefficient_fallback_soln(
    mut eqns: *mut aom_equation_system_t,
) {
    let kTolerance: core::ffi::c_double = 1e-6f64;
    let last: core::ffi::c_int = (*eqns).n - 1 as core::ffi::c_int;
    memset(
        (*eqns).x as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul((*eqns).n as size_t),
    );
    if fabs(*((*eqns).A).offset((last * (*eqns).n + last) as isize)) > kTolerance {
        *((*eqns).x).offset(last as isize) = *((*eqns).b).offset(last as isize)
            / *((*eqns).A).offset((last * (*eqns).n + last) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_lut_init(
    mut lut: *mut aom_noise_strength_lut_t,
    mut num_points: core::ffi::c_int,
) -> core::ffi::c_int {
    if lut.is_null() {
        return 0 as core::ffi::c_int;
    }
    if num_points <= 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    (*lut).num_points = 0 as core::ffi::c_int;
    (*lut).points = aom_malloc(
        (num_points as size_t)
            .wrapping_mul(::core::mem::size_of::<[core::ffi::c_double; 2]>() as size_t),
    ) as *mut [core::ffi::c_double; 2];
    if ((*lut).points).is_null() {
        return 0 as core::ffi::c_int;
    }
    (*lut).num_points = num_points;
    memset(
        (*lut).points as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<[core::ffi::c_double; 2]>() as size_t)
            .wrapping_mul(num_points as size_t),
    );
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_lut_free(
    mut lut: *mut aom_noise_strength_lut_t,
) {
    if lut.is_null() {
        return;
    }
    aom_free((*lut).points as *mut core::ffi::c_void);
    memset(
        lut as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_noise_strength_lut_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_lut_eval(
    mut lut: *const aom_noise_strength_lut_t,
    mut x: core::ffi::c_double,
) -> core::ffi::c_double {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    if x
        < (*((*lut).points)
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
    {
        return (*((*lut).points)
            .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize];
    }
    i = 0 as core::ffi::c_int;
    while i < (*lut).num_points - 1 as core::ffi::c_int {
        if x >= (*((*lut).points).offset(i as isize))[0 as core::ffi::c_int as usize]
            && x
                <= (*((*lut).points)
                    .offset(
                        (i + 1 as core::ffi::c_int) as isize,
                    ))[0 as core::ffi::c_int as usize]
        {
            let a: core::ffi::c_double = (x
                - (*((*lut).points).offset(i as isize))[0 as core::ffi::c_int as usize])
                / ((*((*lut).points)
                    .offset(
                        (i + 1 as core::ffi::c_int) as isize,
                    ))[0 as core::ffi::c_int as usize]
                    - (*((*lut).points)
                        .offset(i as isize))[0 as core::ffi::c_int as usize]);
            return (*((*lut).points)
                .offset(
                    (i + 1 as core::ffi::c_int) as isize,
                ))[1 as core::ffi::c_int as usize] * a
                + (*((*lut).points).offset(i as isize))[1 as core::ffi::c_int as usize]
                    * (1.0f64 - a);
        }
        i += 1;
    }
    return (*((*lut).points)
        .offset(
            ((*lut).num_points - 1 as core::ffi::c_int) as isize,
        ))[1 as core::ffi::c_int as usize];
}
unsafe extern "C-unwind" fn noise_strength_solver_get_bin_index(
    mut solver: *const aom_noise_strength_solver_t,
    mut value: core::ffi::c_double,
) -> core::ffi::c_double {
    let val: core::ffi::c_double = fclamp(
        value,
        (*solver).min_intensity,
        (*solver).max_intensity,
    ) as core::ffi::c_double;
    let range: core::ffi::c_double = (*solver).max_intensity - (*solver).min_intensity;
    return ((*solver).num_bins - 1 as core::ffi::c_int) as core::ffi::c_double
        * (val - (*solver).min_intensity) / range;
}
unsafe extern "C-unwind" fn noise_strength_solver_get_value(
    mut solver: *const aom_noise_strength_solver_t,
    mut x: core::ffi::c_double,
) -> core::ffi::c_double {
    let bin: core::ffi::c_double = noise_strength_solver_get_bin_index(solver, x)
        as core::ffi::c_double;
    let bin_i0: core::ffi::c_int = floor(bin) as core::ffi::c_int;
    let bin_i1: core::ffi::c_int = if ((*solver).num_bins - 1 as core::ffi::c_int)
        < bin_i0 + 1 as core::ffi::c_int
    {
        (*solver).num_bins - 1 as core::ffi::c_int
    } else {
        bin_i0 + 1 as core::ffi::c_int
    };
    let a: core::ffi::c_double = bin - bin_i0 as core::ffi::c_double;
    return (1.0f64 - a) * *((*solver).eqns.x).offset(bin_i0 as isize)
        + a * *((*solver).eqns.x).offset(bin_i1 as isize);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_solver_add_measurement(
    mut solver: *mut aom_noise_strength_solver_t,
    mut block_mean: core::ffi::c_double,
    mut noise_std: core::ffi::c_double,
) {
    let bin: core::ffi::c_double = noise_strength_solver_get_bin_index(
        solver,
        block_mean,
    ) as core::ffi::c_double;
    let bin_i0: core::ffi::c_int = floor(bin) as core::ffi::c_int;
    let bin_i1: core::ffi::c_int = if ((*solver).num_bins - 1 as core::ffi::c_int)
        < bin_i0 + 1 as core::ffi::c_int
    {
        (*solver).num_bins - 1 as core::ffi::c_int
    } else {
        bin_i0 + 1 as core::ffi::c_int
    };
    let a: core::ffi::c_double = bin - bin_i0 as core::ffi::c_double;
    let n: core::ffi::c_int = (*solver).num_bins;
    *((*solver).eqns.A).offset((bin_i0 * n + bin_i0) as isize)
        += (1.0f64 - a) * (1.0f64 - a);
    *((*solver).eqns.A).offset((bin_i1 * n + bin_i0) as isize) += a * (1.0f64 - a);
    *((*solver).eqns.A).offset((bin_i1 * n + bin_i1) as isize) += a * a;
    *((*solver).eqns.A).offset((bin_i0 * n + bin_i1) as isize) += a * (1.0f64 - a);
    *((*solver).eqns.b).offset(bin_i0 as isize) += (1.0f64 - a) * noise_std;
    *((*solver).eqns.b).offset(bin_i1 as isize) += a * noise_std;
    (*solver).total += noise_std;
    (*solver).num_equations += 1;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_solver_solve(
    mut solver: *mut aom_noise_strength_solver_t,
) -> core::ffi::c_int {
    let n: core::ffi::c_int = (*solver).num_bins;
    let kAlpha: core::ffi::c_double = 2.0f64
        * (*solver).num_equations as core::ffi::c_double / n as core::ffi::c_double;
    let mut result: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mean: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut old_A: *mut core::ffi::c_double = (*solver).eqns.A;
    let mut A: *mut core::ffi::c_double = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut core::ffi::c_double;
    if A.is_null() {
        fprintf(
            __stderrp,
            b"Unable to allocate copy of A\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    memcpy(
        A as *mut core::ffi::c_void,
        old_A as *const core::ffi::c_void,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul(n as size_t)
            .wrapping_mul(n as size_t),
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n {
        let i_lo: core::ffi::c_int = if 0 as core::ffi::c_int > i - 1 as core::ffi::c_int
        {
            0 as core::ffi::c_int
        } else {
            i - 1 as core::ffi::c_int
        };
        let i_hi: core::ffi::c_int = if (n - 1 as core::ffi::c_int)
            < i + 1 as core::ffi::c_int
        {
            n - 1 as core::ffi::c_int
        } else {
            i + 1 as core::ffi::c_int
        };
        *A.offset((i * n + i_lo) as isize) -= kAlpha;
        *A.offset((i * n + i) as isize)
            += 2 as core::ffi::c_int as core::ffi::c_double * kAlpha;
        *A.offset((i * n + i_hi) as isize) -= kAlpha;
        i += 1;
    }
    mean = (*solver).total / (*solver).num_equations as core::ffi::c_double;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < n {
        *A.offset((i_0 * n + i_0) as isize) += 1.0f64 / 8192.0f64;
        *((*solver).eqns.b).offset(i_0 as isize) += mean / 8192.0f64;
        i_0 += 1;
    }
    (*solver).eqns.A = A;
    result = equation_system_solve(&raw mut (*solver).eqns);
    (*solver).eqns.A = old_A;
    aom_free(A as *mut core::ffi::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_solver_init(
    mut solver: *mut aom_noise_strength_solver_t,
    mut num_bins: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) -> core::ffi::c_int {
    if solver.is_null() {
        return 0 as core::ffi::c_int;
    }
    memset(
        solver as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_noise_strength_solver_t>() as size_t,
    );
    (*solver).num_bins = num_bins;
    (*solver).min_intensity = 0 as core::ffi::c_int as core::ffi::c_double;
    (*solver).max_intensity = (((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int) as core::ffi::c_double;
    (*solver).total = 0 as core::ffi::c_int as core::ffi::c_double;
    (*solver).num_equations = 0 as core::ffi::c_int;
    return equation_system_init(&raw mut (*solver).eqns, num_bins);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_solver_free(
    mut solver: *mut aom_noise_strength_solver_t,
) {
    if solver.is_null() {
        return;
    }
    equation_system_free(&raw mut (*solver).eqns);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_solver_get_center(
    mut solver: *const aom_noise_strength_solver_t,
    mut i: core::ffi::c_int,
) -> core::ffi::c_double {
    let range: core::ffi::c_double = (*solver).max_intensity - (*solver).min_intensity;
    let n: core::ffi::c_int = (*solver).num_bins;
    return i as core::ffi::c_double / (n - 1 as core::ffi::c_int) as core::ffi::c_double
        * range + (*solver).min_intensity;
}
unsafe extern "C-unwind" fn update_piecewise_linear_residual(
    mut solver: *const aom_noise_strength_solver_t,
    mut lut: *const aom_noise_strength_lut_t,
    mut residual: *mut core::ffi::c_double,
    mut start: core::ffi::c_int,
    mut end: core::ffi::c_int,
) {
    let dx: core::ffi::c_double = 255.0f64 / (*solver).num_bins as core::ffi::c_double;
    let mut i: core::ffi::c_int = if start > 1 as core::ffi::c_int {
        start
    } else {
        1 as core::ffi::c_int
    };
    while i
        < (if end < (*lut).num_points - 1 as core::ffi::c_int {
            end
        } else {
            (*lut).num_points - 1 as core::ffi::c_int
        })
    {
        let lower: core::ffi::c_int = if 0 as core::ffi::c_int
            > floor(
                noise_strength_solver_get_bin_index(
                    solver,
                    (*((*lut).points)
                        .offset(
                            (i - 1 as core::ffi::c_int) as isize,
                        ))[0 as core::ffi::c_int as usize],
                ),
            ) as core::ffi::c_int
        {
            0 as core::ffi::c_int
        } else {
            floor(
                noise_strength_solver_get_bin_index(
                    solver,
                    (*((*lut).points)
                        .offset(
                            (i - 1 as core::ffi::c_int) as isize,
                        ))[0 as core::ffi::c_int as usize],
                ),
            ) as core::ffi::c_int
        };
        let upper: core::ffi::c_int = if ((*solver).num_bins - 1 as core::ffi::c_int)
            < ceil(
                noise_strength_solver_get_bin_index(
                    solver,
                    (*((*lut).points)
                        .offset(
                            (i + 1 as core::ffi::c_int) as isize,
                        ))[0 as core::ffi::c_int as usize],
                ),
            ) as core::ffi::c_int
        {
            (*solver).num_bins - 1 as core::ffi::c_int
        } else {
            ceil(
                noise_strength_solver_get_bin_index(
                    solver,
                    (*((*lut).points)
                        .offset(
                            (i + 1 as core::ffi::c_int) as isize,
                        ))[0 as core::ffi::c_int as usize],
                ),
            ) as core::ffi::c_int
        };
        let mut r: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
        let mut j: core::ffi::c_int = lower;
        while j <= upper {
            let x: core::ffi::c_double = aom_noise_strength_solver_get_center(solver, j)
                as core::ffi::c_double;
            if !(x
                < (*((*lut).points)
                    .offset(
                        (i - 1 as core::ffi::c_int) as isize,
                    ))[0 as core::ffi::c_int as usize])
            {
                if !(x
                    >= (*((*lut).points)
                        .offset(
                            (i + 1 as core::ffi::c_int) as isize,
                        ))[0 as core::ffi::c_int as usize])
                {
                    let y: core::ffi::c_double = *((*solver).eqns.x).offset(j as isize);
                    let a: core::ffi::c_double = (x
                        - (*((*lut).points)
                            .offset(
                                (i - 1 as core::ffi::c_int) as isize,
                            ))[0 as core::ffi::c_int as usize])
                        / ((*((*lut).points)
                            .offset(
                                (i + 1 as core::ffi::c_int) as isize,
                            ))[0 as core::ffi::c_int as usize]
                            - (*((*lut).points)
                                .offset(
                                    (i - 1 as core::ffi::c_int) as isize,
                                ))[0 as core::ffi::c_int as usize]);
                    let estimate_y: core::ffi::c_double = (*((*lut).points)
                        .offset(
                            (i - 1 as core::ffi::c_int) as isize,
                        ))[1 as core::ffi::c_int as usize] * (1.0f64 - a)
                        + (*((*lut).points)
                            .offset(
                                (i + 1 as core::ffi::c_int) as isize,
                            ))[1 as core::ffi::c_int as usize] * a;
                    r += fabs(y - estimate_y);
                }
            }
            j += 1;
        }
        *residual.offset(i as isize) = r * dx;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_strength_solver_fit_piecewise(
    mut solver: *const aom_noise_strength_solver_t,
    mut max_output_points: core::ffi::c_int,
    mut lut: *mut aom_noise_strength_lut_t,
) -> core::ffi::c_int {
    let kTolerance: core::ffi::c_double = (*solver).max_intensity * 0.00625f64
        / 255.0f64;
    if aom_noise_strength_lut_init(lut, (*solver).num_bins) == 0 {
        fprintf(
            __stderrp,
            b"Failed to init lut\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*solver).num_bins {
        (*((*lut).points).offset(i as isize))[0 as core::ffi::c_int as usize] = aom_noise_strength_solver_get_center(
            solver,
            i,
        );
        (*((*lut).points).offset(i as isize))[1 as core::ffi::c_int as usize] = *((*solver)
            .eqns
            .x)
            .offset(i as isize);
        i += 1;
    }
    if max_output_points < 0 as core::ffi::c_int {
        max_output_points = (*solver).num_bins;
    }
    let mut residual: *mut core::ffi::c_double = aom_malloc(
        ((*solver).num_bins as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    if residual.is_null() {
        aom_noise_strength_lut_free(lut);
        return 0 as core::ffi::c_int;
    }
    memset(
        residual as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul((*solver).num_bins as size_t),
    );
    update_piecewise_linear_residual(
        solver,
        lut,
        residual,
        0 as core::ffi::c_int,
        (*solver).num_bins,
    );
    while (*lut).num_points > 2 as core::ffi::c_int {
        let mut min_index: core::ffi::c_int = 1 as core::ffi::c_int;
        let mut j: core::ffi::c_int = 1 as core::ffi::c_int;
        while j < (*lut).num_points - 1 as core::ffi::c_int {
            if *residual.offset(j as isize) < *residual.offset(min_index as isize) {
                min_index = j;
            }
            j += 1;
        }
        let dx: core::ffi::c_double = (*((*lut).points)
            .offset(
                (min_index + 1 as core::ffi::c_int) as isize,
            ))[0 as core::ffi::c_int as usize]
            - (*((*lut).points)
                .offset(
                    (min_index - 1 as core::ffi::c_int) as isize,
                ))[0 as core::ffi::c_int as usize];
        let avg_residual: core::ffi::c_double = *residual.offset(min_index as isize)
            / dx;
        if (*lut).num_points <= max_output_points && avg_residual > kTolerance {
            break;
        }
        let num_remaining: core::ffi::c_int = (*lut).num_points - min_index
            - 1 as core::ffi::c_int;
        memmove(
            ((*lut).points).offset(min_index as isize) as *mut core::ffi::c_void,
            ((*lut).points)
                .offset(min_index as isize)
                .offset(1 as core::ffi::c_int as isize) as *const core::ffi::c_void,
            (::core::mem::size_of::<[core::ffi::c_double; 2]>() as size_t)
                .wrapping_mul(num_remaining as size_t),
        );
        (*lut).num_points -= 1;
        update_piecewise_linear_residual(
            solver,
            lut,
            residual,
            min_index - 1 as core::ffi::c_int,
            min_index + 1 as core::ffi::c_int,
        );
    }
    aom_free(residual as *mut core::ffi::c_void);
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_flat_block_finder_init(
    mut block_finder: *mut aom_flat_block_finder_t,
    mut block_size: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
) -> core::ffi::c_int {
    let n: core::ffi::c_int = block_size * block_size;
    let mut eqns: aom_equation_system_t = aom_equation_system_t {
        A: 0 as *mut core::ffi::c_double,
        b: 0 as *mut core::ffi::c_double,
        x: 0 as *mut core::ffi::c_double,
        n: 0,
    };
    let mut AtA_inv: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut A: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
    (*block_finder).A = 0 as *mut core::ffi::c_double;
    (*block_finder).AtA_inv = 0 as *mut core::ffi::c_double;
    if equation_system_init(&raw mut eqns, kLowPolyNumParams) == 0 {
        fprintf(
            __stderrp,
            b"Failed to init equation system for block_size=%d\n\0" as *const u8
                as *const core::ffi::c_char,
            block_size,
        );
        return 0 as core::ffi::c_int;
    }
    AtA_inv = aom_malloc(
        ((kLowPolyNumParams * kLowPolyNumParams) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    A = aom_malloc(
        ((kLowPolyNumParams * n) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    if AtA_inv.is_null() || A.is_null() {
        fprintf(
            __stderrp,
            b"Failed to alloc A or AtA_inv for block_size=%d\n\0" as *const u8
                as *const core::ffi::c_char,
            block_size,
        );
        aom_free(AtA_inv as *mut core::ffi::c_void);
        aom_free(A as *mut core::ffi::c_void);
        equation_system_free(&raw mut eqns);
        return 0 as core::ffi::c_int;
    }
    (*block_finder).A = A;
    (*block_finder).AtA_inv = AtA_inv;
    (*block_finder).block_size = block_size;
    (*block_finder).normalization = (((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int) as core::ffi::c_double;
    (*block_finder).use_highbd = use_highbd;
    y = 0 as core::ffi::c_int;
    while y < block_size {
        let yd: core::ffi::c_double = (y as core::ffi::c_double
            - block_size as core::ffi::c_double / 2.0f64)
            / (block_size as core::ffi::c_double / 2.0f64);
        x = 0 as core::ffi::c_int;
        while x < block_size {
            let xd: core::ffi::c_double = (x as core::ffi::c_double
                - block_size as core::ffi::c_double / 2.0f64)
                / (block_size as core::ffi::c_double / 2.0f64);
            let coords: [core::ffi::c_double; 3] = [
                yd,
                xd,
                1 as core::ffi::c_int as core::ffi::c_double,
            ];
            let row: core::ffi::c_int = y * block_size + x;
            *A.offset((kLowPolyNumParams * row + 0 as core::ffi::c_int) as isize) = yd;
            *A.offset((kLowPolyNumParams * row + 1 as core::ffi::c_int) as isize) = xd;
            *A.offset((kLowPolyNumParams * row + 2 as core::ffi::c_int) as isize) = 1
                as core::ffi::c_int as core::ffi::c_double;
            i = 0 as core::ffi::c_int;
            while i < kLowPolyNumParams {
                j = 0 as core::ffi::c_int;
                while j < kLowPolyNumParams {
                    *(eqns.A).offset((kLowPolyNumParams * i + j) as isize)
                        += coords[i as usize] * coords[j as usize];
                    j += 1;
                }
                i += 1;
            }
            x += 1;
        }
        y += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < kLowPolyNumParams {
        memset(
            eqns.b as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<core::ffi::c_double>() as size_t)
                .wrapping_mul(kLowPolyNumParams as size_t),
        );
        *(eqns.b).offset(i as isize) = 1 as core::ffi::c_int as core::ffi::c_double;
        let ret: core::ffi::c_int = equation_system_solve(&raw mut eqns) as core::ffi::c_int;
        if ret == 0 {
            return ret;
        }
        j = 0 as core::ffi::c_int;
        while j < kLowPolyNumParams {
            *AtA_inv.offset((j * kLowPolyNumParams + i) as isize) = *(eqns.x)
                .offset(j as isize);
            j += 1;
        }
        i += 1;
    }
    equation_system_free(&raw mut eqns);
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_flat_block_finder_free(
    mut block_finder: *mut aom_flat_block_finder_t,
) {
    if block_finder.is_null() {
        return;
    }
    aom_free((*block_finder).A as *mut core::ffi::c_void);
    aom_free((*block_finder).AtA_inv as *mut core::ffi::c_void);
    memset(
        block_finder as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_flat_block_finder_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_flat_block_finder_extract_block(
    mut block_finder: *const aom_flat_block_finder_t,
    data: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut offsx: core::ffi::c_int,
    mut offsy: core::ffi::c_int,
    mut plane: *mut core::ffi::c_double,
    mut block: *mut core::ffi::c_double,
) {
    let block_size: core::ffi::c_int = (*block_finder).block_size;
    let n: core::ffi::c_int = block_size * block_size;
    let mut A: *const core::ffi::c_double = (*block_finder).A;
    let mut AtA_inv: *const core::ffi::c_double = (*block_finder).AtA_inv;
    let mut plane_coords: [core::ffi::c_double; 3] = [0.; 3];
    let mut AtA_inv_b: [core::ffi::c_double; 3] = [0.; 3];
    let mut xi: core::ffi::c_int = 0;
    let mut yi: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    if (*block_finder).use_highbd != 0 {
        let data16: *const uint16_t = data as *const uint16_t;
        yi = 0 as core::ffi::c_int;
        while yi < block_size {
            let y: core::ffi::c_int = clamp(
                offsy + yi,
                0 as core::ffi::c_int,
                h - 1 as core::ffi::c_int,
            ) as core::ffi::c_int;
            xi = 0 as core::ffi::c_int;
            while xi < block_size {
                let x: core::ffi::c_int = clamp(
                    offsx + xi,
                    0 as core::ffi::c_int,
                    w - 1 as core::ffi::c_int,
                ) as core::ffi::c_int;
                *block.offset((yi * block_size + xi) as isize) = *data16
                    .offset((y * stride + x) as isize) as core::ffi::c_double
                    / (*block_finder).normalization;
                xi += 1;
            }
            yi += 1;
        }
    } else {
        yi = 0 as core::ffi::c_int;
        while yi < block_size {
            let y_0: core::ffi::c_int = clamp(
                offsy + yi,
                0 as core::ffi::c_int,
                h - 1 as core::ffi::c_int,
            ) as core::ffi::c_int;
            xi = 0 as core::ffi::c_int;
            while xi < block_size {
                let x_0: core::ffi::c_int = clamp(
                    offsx + xi,
                    0 as core::ffi::c_int,
                    w - 1 as core::ffi::c_int,
                ) as core::ffi::c_int;
                *block.offset((yi * block_size + xi) as isize) = *data
                    .offset((y_0 * stride + x_0) as isize) as core::ffi::c_double
                    / (*block_finder).normalization;
                xi += 1;
            }
            yi += 1;
        }
    }
    multiply_mat(
        block,
        A,
        AtA_inv_b.as_mut_ptr(),
        1 as core::ffi::c_int,
        n,
        kLowPolyNumParams,
    );
    multiply_mat(
        AtA_inv,
        AtA_inv_b.as_mut_ptr(),
        plane_coords.as_mut_ptr(),
        kLowPolyNumParams,
        kLowPolyNumParams,
        1 as core::ffi::c_int,
    );
    multiply_mat(
        A,
        plane_coords.as_mut_ptr(),
        plane,
        n,
        kLowPolyNumParams,
        1 as core::ffi::c_int,
    );
    i = 0 as core::ffi::c_int;
    while i < n {
        *block.offset(i as isize) -= *plane.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn compare_scores(
    mut a: *const core::ffi::c_void,
    mut b: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let diff: core::ffi::c_float = (*(a as *mut index_and_score_t)).score
        - (*(b as *mut index_and_score_t)).score;
    if diff < 0 as core::ffi::c_int as core::ffi::c_float {
        return -(1 as core::ffi::c_int)
    } else if diff > 0 as core::ffi::c_int as core::ffi::c_float {
        return 1 as core::ffi::c_int
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_flat_block_finder_run(
    mut block_finder: *const aom_flat_block_finder_t,
    data: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut flat_blocks: *mut uint8_t,
) -> core::ffi::c_int {
    let block_size: core::ffi::c_int = (*block_finder).block_size;
    let n: core::ffi::c_int = block_size * block_size;
    let kTraceThreshold: core::ffi::c_double = 0.15f64
        / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as core::ffi::c_double;
    let kRatioThreshold: core::ffi::c_double = 1.25f64;
    let kNormThreshold: core::ffi::c_double = 0.08f64
        / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as core::ffi::c_double;
    let kVarThreshold: core::ffi::c_double = 0.005f64 / n as core::ffi::c_double;
    let num_blocks_w: core::ffi::c_int = (w + block_size - 1 as core::ffi::c_int)
        / block_size;
    let num_blocks_h: core::ffi::c_int = (h + block_size - 1 as core::ffi::c_int)
        / block_size;
    let mut num_flat: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut plane: *mut core::ffi::c_double = aom_malloc(
        (n as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    let mut block: *mut core::ffi::c_double = aom_malloc(
        (n as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    let mut scores: *mut index_and_score_t = aom_malloc(
        ((num_blocks_w * num_blocks_h) as size_t)
            .wrapping_mul(::core::mem::size_of::<index_and_score_t>() as size_t),
    ) as *mut index_and_score_t;
    if plane.is_null() || block.is_null() || scores.is_null() {
        fprintf(
            __stderrp,
            b"Failed to allocate memory for block of size %d\n\0" as *const u8
                as *const core::ffi::c_char,
            n,
        );
        aom_free(plane as *mut core::ffi::c_void);
        aom_free(block as *mut core::ffi::c_void);
        aom_free(scores as *mut core::ffi::c_void);
        return -(1 as core::ffi::c_int);
    }
    let mut by: core::ffi::c_int = 0 as core::ffi::c_int;
    while by < num_blocks_h {
        let mut bx: core::ffi::c_int = 0 as core::ffi::c_int;
        while bx < num_blocks_w {
            aom_flat_block_finder_extract_block(
                block_finder,
                data,
                w,
                h,
                stride,
                bx * block_size,
                by * block_size,
                plane,
                block,
            );
            let mut Gxx: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut Gxy: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut Gyy: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut mean: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut var: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut yi: core::ffi::c_int = 1 as core::ffi::c_int;
            while yi < block_size - 1 as core::ffi::c_int {
                let mut xi: core::ffi::c_int = 1 as core::ffi::c_int;
                while xi < block_size - 1 as core::ffi::c_int {
                    let gx: core::ffi::c_double = (*block
                        .offset((yi * block_size + xi + 1 as core::ffi::c_int) as isize)
                        - *block
                            .offset(
                                (yi * block_size + xi - 1 as core::ffi::c_int) as isize,
                            )) / 2 as core::ffi::c_int as core::ffi::c_double;
                    let gy: core::ffi::c_double = (*block
                        .offset((yi * block_size + xi + block_size) as isize)
                        - *block.offset((yi * block_size + xi - block_size) as isize))
                        / 2 as core::ffi::c_int as core::ffi::c_double;
                    Gxx += gx * gx;
                    Gxy += gx * gy;
                    Gyy += gy * gy;
                    let value: core::ffi::c_double = *block
                        .offset((yi * block_size + xi) as isize);
                    mean += value;
                    var += value * value;
                    xi += 1;
                }
                yi += 1;
            }
            mean
                /= ((block_size - 2 as core::ffi::c_int)
                    * (block_size - 2 as core::ffi::c_int)) as core::ffi::c_double;
            Gxx
                /= ((block_size - 2 as core::ffi::c_int)
                    * (block_size - 2 as core::ffi::c_int)) as core::ffi::c_double;
            Gxy
                /= ((block_size - 2 as core::ffi::c_int)
                    * (block_size - 2 as core::ffi::c_int)) as core::ffi::c_double;
            Gyy
                /= ((block_size - 2 as core::ffi::c_int)
                    * (block_size - 2 as core::ffi::c_int)) as core::ffi::c_double;
            var = var
                / ((block_size - 2 as core::ffi::c_int)
                    * (block_size - 2 as core::ffi::c_int)) as core::ffi::c_double
                - mean * mean;
            let trace: core::ffi::c_double = Gxx + Gyy;
            let det: core::ffi::c_double = Gxx * Gyy - Gxy * Gxy;
            let e1: core::ffi::c_double = (trace
                + sqrt(
                    trace * trace - 4 as core::ffi::c_int as core::ffi::c_double * det,
                ) as core::ffi::c_double) / 2.0f64;
            let e2: core::ffi::c_double = (trace
                - sqrt(
                    trace * trace - 4 as core::ffi::c_int as core::ffi::c_double * det,
                ) as core::ffi::c_double) / 2.0f64;
            let norm: core::ffi::c_double = e1;
            let ratio: core::ffi::c_double = e1
                / (if e2 > 1e-6f64 { e2 } else { 1e-6f64 });
            let is_flat: core::ffi::c_int = (trace < kTraceThreshold
                && ratio < kRatioThreshold && norm < kNormThreshold
                && var > kVarThreshold) as core::ffi::c_int;
            let weights: [core::ffi::c_double; 5] = [
                -(6682 as core::ffi::c_int) as core::ffi::c_double,
                -0.2056f64,
                13087 as core::ffi::c_int as core::ffi::c_double,
                -(12434 as core::ffi::c_int) as core::ffi::c_double,
                2.5694f64,
            ];
            let mut sum_weights: core::ffi::c_double = weights[0 as core::ffi::c_int
                as usize] * var + weights[1 as core::ffi::c_int as usize] * ratio
                + weights[2 as core::ffi::c_int as usize] * trace
                + weights[3 as core::ffi::c_int as usize] * norm
                + weights[4 as core::ffi::c_int as usize];
            sum_weights = fclamp(sum_weights, -25.0f64, 100.0f64);
            let score: core::ffi::c_float = (1.0f64
                / (1 as core::ffi::c_int as core::ffi::c_double + exp(-sum_weights)))
                as core::ffi::c_float;
            *flat_blocks.offset((by * num_blocks_w + bx) as isize) = (if is_flat != 0 {
                255 as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            }) as uint8_t;
            (*scores.offset((by * num_blocks_w + bx) as isize)).score = if var
                > kVarThreshold
            {
                score
            } else {
                0 as core::ffi::c_int as core::ffi::c_float
            };
            (*scores.offset((by * num_blocks_w + bx) as isize)).index = by * num_blocks_w
                + bx;
            num_flat += is_flat;
            bx += 1;
        }
        by += 1;
    }
    qsort(
        scores as *mut core::ffi::c_void,
        (num_blocks_w * num_blocks_h) as size_t,
        ::core::mem::size_of::<index_and_score_t>() as size_t,
        Some(
            compare_scores
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    let top_nth_percentile: core::ffi::c_int = num_blocks_w * num_blocks_h
        * 90 as core::ffi::c_int / 100 as core::ffi::c_int;
    let score_threshold: core::ffi::c_float = (*scores
        .offset(top_nth_percentile as isize))
        .score;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_blocks_w * num_blocks_h {
        if (*scores.offset(i as isize)).score >= score_threshold {
            num_flat
                += (*flat_blocks.offset((*scores.offset(i as isize)).index as isize)
                    as core::ffi::c_int == 0 as core::ffi::c_int) as core::ffi::c_int;
            let fresh1 = &raw mut *flat_blocks
                .offset((*scores.offset(i as isize)).index as isize);
            *fresh1 = (*fresh1 as core::ffi::c_int | 1 as core::ffi::c_int) as uint8_t;
        }
        i += 1;
    }
    aom_free(block as *mut core::ffi::c_void);
    aom_free(plane as *mut core::ffi::c_void);
    aom_free(scores as *mut core::ffi::c_void);
    return num_flat;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_model_init(
    mut model: *mut aom_noise_model_t,
    params: aom_noise_model_params_t,
) -> core::ffi::c_int {
    let n: core::ffi::c_int = num_coeffs(params) as core::ffi::c_int;
    let lag: core::ffi::c_int = params.lag;
    let bit_depth: core::ffi::c_int = params.bit_depth;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    memset(
        model as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_noise_model_t>() as size_t,
    );
    if params.lag < 1 as core::ffi::c_int {
        fprintf(
            __stderrp,
            b"Invalid noise param: lag = %d must be >= 1\n\0" as *const u8
                as *const core::ffi::c_char,
            params.lag,
        );
        return 0 as core::ffi::c_int;
    }
    if params.lag > kMaxLag {
        fprintf(
            __stderrp,
            b"Invalid noise param: lag = %d must be <= %d\n\0" as *const u8
                as *const core::ffi::c_char,
            params.lag,
            kMaxLag,
        );
        return 0 as core::ffi::c_int;
    }
    if !(params.bit_depth == 8 as core::ffi::c_int
        || params.bit_depth == 10 as core::ffi::c_int
        || params.bit_depth == 12 as core::ffi::c_int)
    {
        return 0 as core::ffi::c_int;
    }
    (*model).params = params;
    c = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        if noise_state_init(
            &raw mut *((*model).combined_state).as_mut_ptr().offset(c as isize),
            n + (c > 0 as core::ffi::c_int) as core::ffi::c_int,
            bit_depth,
        ) == 0
        {
            fprintf(
                __stderrp,
                b"Failed to allocate noise state for channel %d\n\0" as *const u8
                    as *const core::ffi::c_char,
                c,
            );
            aom_noise_model_free(model);
            return 0 as core::ffi::c_int;
        }
        if noise_state_init(
            &raw mut *((*model).latest_state).as_mut_ptr().offset(c as isize),
            n + (c > 0 as core::ffi::c_int) as core::ffi::c_int,
            bit_depth,
        ) == 0
        {
            fprintf(
                __stderrp,
                b"Failed to allocate noise state for channel %d\n\0" as *const u8
                    as *const core::ffi::c_char,
                c,
            );
            aom_noise_model_free(model);
            return 0 as core::ffi::c_int;
        }
        c += 1;
    }
    (*model).n = n;
    (*model).coords = aom_malloc(
        (::core::mem::size_of::<[core::ffi::c_int; 2]>() as size_t)
            .wrapping_mul(n as size_t),
    ) as *mut [core::ffi::c_int; 2];
    if ((*model).coords).is_null() {
        aom_noise_model_free(model);
        return 0 as core::ffi::c_int;
    }
    y = -lag;
    while y <= 0 as core::ffi::c_int {
        let max_x: core::ffi::c_int = if y == 0 as core::ffi::c_int {
            -(1 as core::ffi::c_int)
        } else {
            lag
        };
        x = -lag;
        while x <= max_x {
            match params.shape as core::ffi::c_int {
                0 => {
                    if abs(x) <= y + lag {
                        (*((*model).coords)
                            .offset(i as isize))[0 as core::ffi::c_int as usize] = x;
                        (*((*model).coords)
                            .offset(i as isize))[1 as core::ffi::c_int as usize] = y;
                        i += 1;
                    }
                }
                1 => {
                    (*((*model).coords)
                        .offset(i as isize))[0 as core::ffi::c_int as usize] = x;
                    (*((*model).coords)
                        .offset(i as isize))[1 as core::ffi::c_int as usize] = y;
                    i += 1;
                }
                _ => {
                    fprintf(
                        __stderrp,
                        b"Invalid shape\n\0" as *const u8 as *const core::ffi::c_char,
                    );
                    aom_noise_model_free(model);
                    return 0 as core::ffi::c_int;
                }
            }
            x += 1;
        }
        y += 1;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_model_free(mut model: *mut aom_noise_model_t) {
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    if model.is_null() {
        return;
    }
    aom_free((*model).coords as *mut core::ffi::c_void);
    c = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        equation_system_free(
            &raw mut (*((*model).latest_state).as_mut_ptr().offset(c as isize)).eqns,
        );
        equation_system_free(
            &raw mut (*((*model).combined_state).as_mut_ptr().offset(c as isize)).eqns,
        );
        equation_system_free(
            &raw mut (*((*model).latest_state).as_mut_ptr().offset(c as isize))
                .strength_solver
                .eqns,
        );
        equation_system_free(
            &raw mut (*((*model).combined_state).as_mut_ptr().offset(c as isize))
                .strength_solver
                .eqns,
        );
        c += 1;
    }
    memset(
        model as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_noise_model_t>() as size_t,
    );
}
unsafe extern "C-unwind" fn extract_ar_row_lowbd(
    mut coords: *mut [core::ffi::c_int; 2],
    mut num_coords: core::ffi::c_int,
    data: *const uint8_t,
    denoised: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut sub_log2: *mut core::ffi::c_int,
    alt_data: *const uint8_t,
    alt_denoised: *const uint8_t,
    mut alt_stride: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut buffer: *mut core::ffi::c_double,
) -> core::ffi::c_double {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_coords {
        let x_i: core::ffi::c_int = x
            + (*coords.offset(i as isize))[0 as core::ffi::c_int as usize];
        let y_i: core::ffi::c_int = y
            + (*coords.offset(i as isize))[1 as core::ffi::c_int as usize];
        *buffer.offset(i as isize) = *data.offset((y_i * stride + x_i) as isize)
            as core::ffi::c_double
            - *denoised.offset((y_i * stride + x_i) as isize) as core::ffi::c_int
                as core::ffi::c_double;
        i += 1;
    }
    let val: core::ffi::c_double = *data.offset((y * stride + x) as isize)
        as core::ffi::c_double
        - *denoised.offset((y * stride + x) as isize) as core::ffi::c_int
            as core::ffi::c_double;
    if !alt_data.is_null() && !alt_denoised.is_null() {
        let mut avg_data: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut avg_denoised: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut num_samples: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut dy_i: core::ffi::c_int = 0 as core::ffi::c_int;
        while dy_i
            < (1 as core::ffi::c_int) << *sub_log2.offset(1 as core::ffi::c_int as isize)
        {
            let y_up: core::ffi::c_int = (y
                << *sub_log2.offset(1 as core::ffi::c_int as isize)) + dy_i;
            let mut dx_i: core::ffi::c_int = 0 as core::ffi::c_int;
            while dx_i
                < (1 as core::ffi::c_int)
                    << *sub_log2.offset(0 as core::ffi::c_int as isize)
            {
                let x_up: core::ffi::c_int = (x
                    << *sub_log2.offset(0 as core::ffi::c_int as isize)) + dx_i;
                avg_data
                    += *alt_data.offset((y_up * alt_stride + x_up) as isize)
                        as core::ffi::c_int as core::ffi::c_double;
                avg_denoised
                    += *alt_denoised.offset((y_up * alt_stride + x_up) as isize)
                        as core::ffi::c_int as core::ffi::c_double;
                num_samples += 1;
                dx_i += 1;
            }
            dy_i += 1;
        }
        *buffer.offset(num_coords as isize) = (avg_data - avg_denoised)
            / num_samples as core::ffi::c_double;
    }
    return val;
}
unsafe extern "C-unwind" fn extract_ar_row_highbd(
    mut coords: *mut [core::ffi::c_int; 2],
    mut num_coords: core::ffi::c_int,
    data: *const uint16_t,
    denoised: *const uint16_t,
    mut stride: core::ffi::c_int,
    mut sub_log2: *mut core::ffi::c_int,
    alt_data: *const uint16_t,
    alt_denoised: *const uint16_t,
    mut alt_stride: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    mut buffer: *mut core::ffi::c_double,
) -> core::ffi::c_double {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_coords {
        let x_i: core::ffi::c_int = x
            + (*coords.offset(i as isize))[0 as core::ffi::c_int as usize];
        let y_i: core::ffi::c_int = y
            + (*coords.offset(i as isize))[1 as core::ffi::c_int as usize];
        *buffer.offset(i as isize) = *data.offset((y_i * stride + x_i) as isize)
            as core::ffi::c_double
            - *denoised.offset((y_i * stride + x_i) as isize) as core::ffi::c_int
                as core::ffi::c_double;
        i += 1;
    }
    let val: core::ffi::c_double = *data.offset((y * stride + x) as isize)
        as core::ffi::c_double
        - *denoised.offset((y * stride + x) as isize) as core::ffi::c_int
            as core::ffi::c_double;
    if !alt_data.is_null() && !alt_denoised.is_null() {
        let mut avg_data: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut avg_denoised: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut num_samples: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut dy_i: core::ffi::c_int = 0 as core::ffi::c_int;
        while dy_i
            < (1 as core::ffi::c_int) << *sub_log2.offset(1 as core::ffi::c_int as isize)
        {
            let y_up: core::ffi::c_int = (y
                << *sub_log2.offset(1 as core::ffi::c_int as isize)) + dy_i;
            let mut dx_i: core::ffi::c_int = 0 as core::ffi::c_int;
            while dx_i
                < (1 as core::ffi::c_int)
                    << *sub_log2.offset(0 as core::ffi::c_int as isize)
            {
                let x_up: core::ffi::c_int = (x
                    << *sub_log2.offset(0 as core::ffi::c_int as isize)) + dx_i;
                avg_data
                    += *alt_data.offset((y_up * alt_stride + x_up) as isize)
                        as core::ffi::c_int as core::ffi::c_double;
                avg_denoised
                    += *alt_denoised.offset((y_up * alt_stride + x_up) as isize)
                        as core::ffi::c_int as core::ffi::c_double;
                num_samples += 1;
                dx_i += 1;
            }
            dy_i += 1;
        }
        *buffer.offset(num_coords as isize) = (avg_data - avg_denoised)
            / num_samples as core::ffi::c_double;
    }
    return val;
}
unsafe extern "C-unwind" fn add_block_observations(
    mut noise_model: *mut aom_noise_model_t,
    mut c: core::ffi::c_int,
    data: *const uint8_t,
    denoised: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut sub_log2: *mut core::ffi::c_int,
    alt_data: *const uint8_t,
    alt_denoised: *const uint8_t,
    mut alt_stride: core::ffi::c_int,
    flat_blocks: *const uint8_t,
    mut block_size: core::ffi::c_int,
    mut num_blocks_w: core::ffi::c_int,
    mut num_blocks_h: core::ffi::c_int,
) -> core::ffi::c_int {
    let lag: core::ffi::c_int = (*noise_model).params.lag;
    let num_coords: core::ffi::c_int = (*noise_model).n;
    let normalization: core::ffi::c_double = (((1 as core::ffi::c_int)
        << (*noise_model).params.bit_depth) - 1 as core::ffi::c_int)
        as core::ffi::c_double;
    let mut A: *mut core::ffi::c_double = (*noise_model).latest_state[c as usize].eqns.A;
    let mut b: *mut core::ffi::c_double = (*noise_model).latest_state[c as usize].eqns.b;
    let mut buffer: *mut core::ffi::c_double = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_double>() as size_t)
            .wrapping_mul((num_coords + 1 as core::ffi::c_int) as size_t),
    ) as *mut core::ffi::c_double;
    let n: core::ffi::c_int = (*noise_model).latest_state[c as usize].eqns.n;
    if buffer.is_null() {
        fprintf(
            __stderrp,
            b"Unable to allocate buffer of size %d\n\0" as *const u8
                as *const core::ffi::c_char,
            num_coords + 1 as core::ffi::c_int,
        );
        return 0 as core::ffi::c_int;
    }
    let mut by: core::ffi::c_int = 0 as core::ffi::c_int;
    while by < num_blocks_h {
        let y_o: core::ffi::c_int = by
            * (block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize));
        let mut bx: core::ffi::c_int = 0 as core::ffi::c_int;
        while bx < num_blocks_w {
            let x_o: core::ffi::c_int = bx
                * (block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize));
            if !(*flat_blocks.offset((by * num_blocks_w + bx) as isize) == 0) {
                let mut y_start: core::ffi::c_int = if by > 0 as core::ffi::c_int
                    && *flat_blocks
                        .offset(
                            ((by - 1 as core::ffi::c_int) * num_blocks_w + bx) as isize,
                        ) as core::ffi::c_int != 0
                {
                    0 as core::ffi::c_int
                } else {
                    lag
                };
                let mut x_start: core::ffi::c_int = if bx > 0 as core::ffi::c_int
                    && *flat_blocks
                        .offset(
                            (by * num_blocks_w + bx - 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int != 0
                {
                    0 as core::ffi::c_int
                } else {
                    lag
                };
                let mut y_end: core::ffi::c_int = if (h
                    >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                    - by
                        * (block_size
                            >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                    < block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize)
                {
                    (h >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                        - by
                            * (block_size
                                >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                } else {
                    block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize)
                };
                let mut x_end: core::ffi::c_int = if (w
                    >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                    - bx
                        * (block_size
                            >> *sub_log2.offset(0 as core::ffi::c_int as isize)) - lag
                    < (if (bx + 1 as core::ffi::c_int) < num_blocks_w
                        && *flat_blocks
                            .offset(
                                (by * num_blocks_w + bx + 1 as core::ffi::c_int) as isize,
                            ) as core::ffi::c_int != 0
                    {
                        block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize)
                    } else {
                        (block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                            - lag
                    })
                {
                    (w >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                        - bx
                            * (block_size
                                >> *sub_log2.offset(0 as core::ffi::c_int as isize)) - lag
                } else if (bx + 1 as core::ffi::c_int) < num_blocks_w
                    && *flat_blocks
                        .offset(
                            (by * num_blocks_w + bx + 1 as core::ffi::c_int) as isize,
                        ) as core::ffi::c_int != 0
                {
                    block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize)
                } else {
                    (block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                        - lag
                };
                let mut y: core::ffi::c_int = y_start;
                while y < y_end {
                    let mut x: core::ffi::c_int = x_start;
                    while x < x_end {
                        let val: core::ffi::c_double = if (*noise_model)
                            .params
                            .use_highbd != 0
                        {
                            extract_ar_row_highbd(
                                (*noise_model).coords,
                                num_coords,
                                data as *const uint16_t,
                                denoised as *const uint16_t,
                                stride,
                                sub_log2,
                                alt_data as *const uint16_t,
                                alt_denoised as *const uint16_t,
                                alt_stride,
                                x + x_o,
                                y + y_o,
                                buffer,
                            ) as core::ffi::c_double
                        } else {
                            extract_ar_row_lowbd(
                                (*noise_model).coords,
                                num_coords,
                                data,
                                denoised,
                                stride,
                                sub_log2,
                                alt_data,
                                alt_denoised,
                                alt_stride,
                                x + x_o,
                                y + y_o,
                                buffer,
                            ) as core::ffi::c_double
                        };
                        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                        while i < n {
                            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                            while j < n {
                                *A.offset((i * n + j) as isize)
                                    += *buffer.offset(i as isize) * *buffer.offset(j as isize)
                                        / (normalization * normalization);
                                j += 1;
                            }
                            *b.offset(i as isize)
                                += *buffer.offset(i as isize) * val
                                    / (normalization * normalization);
                            i += 1;
                        }
                        (*noise_model).latest_state[c as usize].num_observations += 1;
                        x += 1;
                    }
                    y += 1;
                }
            }
            bx += 1;
        }
        by += 1;
    }
    aom_free(buffer as *mut core::ffi::c_void);
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn add_noise_std_observations(
    mut noise_model: *mut aom_noise_model_t,
    mut c: core::ffi::c_int,
    mut coeffs: *const core::ffi::c_double,
    data: *const uint8_t,
    denoised: *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut sub_log2: *mut core::ffi::c_int,
    alt_data: *const uint8_t,
    mut alt_stride: core::ffi::c_int,
    flat_blocks: *const uint8_t,
    mut block_size: core::ffi::c_int,
    mut num_blocks_w: core::ffi::c_int,
    mut num_blocks_h: core::ffi::c_int,
) {
    let num_coords: core::ffi::c_int = (*noise_model).n;
    let mut noise_strength_solver: *mut aom_noise_strength_solver_t = &raw mut (*((*noise_model)
        .latest_state)
        .as_mut_ptr()
        .offset(c as isize))
        .strength_solver;
    let mut noise_strength_luma: *const aom_noise_strength_solver_t = &raw mut (*((*noise_model)
        .latest_state)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .strength_solver;
    let luma_gain: core::ffi::c_double = (*noise_model)
        .latest_state[0 as core::ffi::c_int as usize]
        .ar_gain;
    let noise_gain: core::ffi::c_double = (*noise_model)
        .latest_state[c as usize]
        .ar_gain;
    let mut by: core::ffi::c_int = 0 as core::ffi::c_int;
    while by < num_blocks_h {
        let y_o: core::ffi::c_int = by
            * (block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize));
        let mut bx: core::ffi::c_int = 0 as core::ffi::c_int;
        while bx < num_blocks_w {
            let x_o: core::ffi::c_int = bx
                * (block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize));
            if !(*flat_blocks.offset((by * num_blocks_w + bx) as isize) == 0) {
                let num_samples_h: core::ffi::c_int = if (h
                    >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                    - by
                        * (block_size
                            >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                    < block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize)
                {
                    (h >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                        - by
                            * (block_size
                                >> *sub_log2.offset(1 as core::ffi::c_int as isize))
                } else {
                    block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize)
                };
                let num_samples_w: core::ffi::c_int = if (w
                    >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                    - bx
                        * (block_size
                            >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                    < block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize)
                {
                    (w >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                        - bx
                            * (block_size
                                >> *sub_log2.offset(0 as core::ffi::c_int as isize))
                } else {
                    block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize)
                };
                if num_samples_w * num_samples_h > block_size {
                    let block_mean: core::ffi::c_double = get_block_mean(
                        if !alt_data.is_null() { alt_data } else { data },
                        w,
                        h,
                        if !alt_data.is_null() { alt_stride } else { stride },
                        x_o << *sub_log2.offset(0 as core::ffi::c_int as isize),
                        y_o << *sub_log2.offset(1 as core::ffi::c_int as isize),
                        block_size,
                        (*noise_model).params.use_highbd,
                    ) as core::ffi::c_double;
                    let noise_var: core::ffi::c_double = get_noise_var(
                        data,
                        denoised,
                        stride,
                        w >> *sub_log2.offset(0 as core::ffi::c_int as isize),
                        h >> *sub_log2.offset(1 as core::ffi::c_int as isize),
                        x_o,
                        y_o,
                        block_size >> *sub_log2.offset(0 as core::ffi::c_int as isize),
                        block_size >> *sub_log2.offset(1 as core::ffi::c_int as isize),
                        (*noise_model).params.use_highbd,
                    ) as core::ffi::c_double;
                    let luma_strength: core::ffi::c_double = if c > 0 as core::ffi::c_int
                    {
                        luma_gain
                            * noise_strength_solver_get_value(
                                noise_strength_luma,
                                block_mean,
                            ) as core::ffi::c_double
                    } else {
                        0 as core::ffi::c_int as core::ffi::c_double
                    };
                    let corr: core::ffi::c_double = if c > 0 as core::ffi::c_int {
                        *coeffs.offset(num_coords as isize)
                    } else {
                        0 as core::ffi::c_int as core::ffi::c_double
                    };
                    let uncorr_std: core::ffi::c_double = sqrt(
                        if noise_var / 16 as core::ffi::c_int as core::ffi::c_double
                            > noise_var
                                - pow(
                                    corr * luma_strength,
                                    2 as core::ffi::c_int as core::ffi::c_double,
                                )
                        {
                            noise_var / 16 as core::ffi::c_int as core::ffi::c_double
                        } else {
                            noise_var
                                - pow(
                                    corr * luma_strength,
                                    2 as core::ffi::c_int as core::ffi::c_double,
                                )
                        },
                    ) as core::ffi::c_double;
                    let adjusted_strength: core::ffi::c_double = uncorr_std / noise_gain;
                    aom_noise_strength_solver_add_measurement(
                        noise_strength_solver,
                        block_mean,
                        adjusted_strength,
                    );
                }
            }
            bx += 1;
        }
        by += 1;
    }
}
unsafe extern "C-unwind" fn is_noise_model_different(
    noise_model: *mut aom_noise_model_t,
) -> core::ffi::c_int {
    let kCoeffThreshold: core::ffi::c_double = 0.9f64;
    let kStrengthThreshold: core::ffi::c_double = 0.005f64
        * ((1 as core::ffi::c_int)
            << (*noise_model).params.bit_depth - 8 as core::ffi::c_int)
            as core::ffi::c_double;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < 1 as core::ffi::c_int {
        let corr: core::ffi::c_double = aom_normalized_cross_correlation(
            (*noise_model).latest_state[c as usize].eqns.x,
            (*noise_model).combined_state[c as usize].eqns.x,
            (*noise_model).combined_state[c as usize].eqns.n,
        ) as core::ffi::c_double;
        if corr < kCoeffThreshold {
            return 1 as core::ffi::c_int;
        }
        let dx: core::ffi::c_double = 1.0f64
            / (*noise_model).latest_state[c as usize].strength_solver.num_bins
                as core::ffi::c_double;
        let mut latest_eqns: *const aom_equation_system_t = &raw mut (*((*noise_model)
            .latest_state)
            .as_mut_ptr()
            .offset(c as isize))
            .strength_solver
            .eqns;
        let mut combined_eqns: *const aom_equation_system_t = &raw mut (*((*noise_model)
            .combined_state)
            .as_mut_ptr()
            .offset(c as isize))
            .strength_solver
            .eqns;
        let mut diff: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
        let mut total_weight: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < (*latest_eqns).n {
            let mut weight: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < (*latest_eqns).n {
                weight
                    += *((*latest_eqns).A).offset((i * (*latest_eqns).n + j) as isize);
                i += 1;
            }
            weight = sqrt(weight);
            diff
                += weight
                    * fabs(
                        *((*latest_eqns).x).offset(j as isize)
                            - *((*combined_eqns).x).offset(j as isize),
                    );
            total_weight += weight;
            j += 1;
        }
        if diff * dx / total_weight > kStrengthThreshold {
            return 1 as core::ffi::c_int;
        }
        c += 1;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn ar_equation_system_solve(
    mut state: *mut aom_noise_state_t,
    mut is_chroma: core::ffi::c_int,
) -> core::ffi::c_int {
    let ret: core::ffi::c_int = equation_system_solve(&raw mut (*state).eqns)
        as core::ffi::c_int;
    (*state).ar_gain = 1.0f64;
    if ret == 0 {
        return ret;
    }
    let mut var: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let n: core::ffi::c_int = (*state).eqns.n;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*state).eqns.n - is_chroma {
        var
            += *((*state).eqns.A).offset((i * n + i) as isize)
                / (*state).num_observations as core::ffi::c_double;
        i += 1;
    }
    var /= (n - is_chroma) as core::ffi::c_double;
    let mut sum_covar: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < (*state).eqns.n - is_chroma {
        let mut bi: core::ffi::c_double = *((*state).eqns.b).offset(i_0 as isize);
        if is_chroma != 0 {
            bi
                -= *((*state).eqns.A)
                    .offset((i_0 * n + (n - 1 as core::ffi::c_int)) as isize)
                    * *((*state).eqns.x).offset((n - 1 as core::ffi::c_int) as isize);
        }
        sum_covar
            += bi * *((*state).eqns.x).offset(i_0 as isize)
                / (*state).num_observations as core::ffi::c_double;
        i_0 += 1;
    }
    let noise_var: core::ffi::c_double = if var - sum_covar > 1e-6f64 {
        var - sum_covar
    } else {
        1e-6f64
    };
    (*state).ar_gain = if 1 as core::ffi::c_int as core::ffi::c_double
        > sqrt((if var / noise_var > 1e-6f64 { var / noise_var } else { 1e-6f64 }))
    {
        1 as core::ffi::c_int as core::ffi::c_double
    } else {
        sqrt(if var / noise_var > 1e-6f64 { var / noise_var } else { 1e-6f64 })
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_model_update(
    noise_model: *mut aom_noise_model_t,
    mut data: *const *const uint8_t,
    mut denoised: *const *const uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: *mut core::ffi::c_int,
    mut chroma_sub_log2: *mut core::ffi::c_int,
    flat_blocks: *const uint8_t,
    mut block_size: core::ffi::c_int,
) -> aom_noise_status_t {
    let num_blocks_w: core::ffi::c_int = (w + block_size - 1 as core::ffi::c_int)
        / block_size;
    let num_blocks_h: core::ffi::c_int = (h + block_size - 1 as core::ffi::c_int)
        / block_size;
    let mut y_model_different: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut num_blocks: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut channel: core::ffi::c_int = 0 as core::ffi::c_int;
    if block_size <= 1 as core::ffi::c_int {
        fprintf(
            __stderrp,
            b"block_size = %d must be > 1\n\0" as *const u8 as *const core::ffi::c_char,
            block_size,
        );
        return AOM_NOISE_STATUS_INVALID_ARGUMENT as core::ffi::c_int
            as aom_noise_status_t;
    }
    if block_size
        < (*noise_model).params.lag * 2 as core::ffi::c_int + 1 as core::ffi::c_int
    {
        fprintf(
            __stderrp,
            b"block_size = %d must be >= %d\n\0" as *const u8
                as *const core::ffi::c_char,
            block_size,
            (*noise_model).params.lag * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
        );
        return AOM_NOISE_STATUS_INVALID_ARGUMENT as core::ffi::c_int
            as aom_noise_status_t;
    }
    i = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        equation_system_clear(
            &raw mut (*((*noise_model).latest_state).as_mut_ptr().offset(i as isize)).eqns,
        );
        (*noise_model).latest_state[i as usize].num_observations = 0 as core::ffi::c_int;
        noise_strength_solver_clear(
            &raw mut (*((*noise_model).latest_state).as_mut_ptr().offset(i as isize))
                .strength_solver,
        );
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < num_blocks_h * num_blocks_w {
        if *flat_blocks.offset(i as isize) != 0 {
            num_blocks += 1;
        }
        i += 1;
    }
    if num_blocks <= 1 as core::ffi::c_int {
        fprintf(
            __stderrp,
            b"Not enough flat blocks to update noise estimate\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AOM_NOISE_STATUS_INSUFFICIENT_FLAT_BLOCKS as core::ffi::c_int
            as aom_noise_status_t;
    }
    channel = 0 as core::ffi::c_int;
    while channel < 3 as core::ffi::c_int {
        let mut no_subsampling: [core::ffi::c_int; 2] = [
            0 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ];
        let mut alt_data: *const uint8_t = if channel > 0 as core::ffi::c_int {
            *data.offset(0 as core::ffi::c_int as isize)
        } else {
            0 as *const uint8_t
        };
        let mut alt_denoised: *const uint8_t = if channel > 0 as core::ffi::c_int {
            *denoised.offset(0 as core::ffi::c_int as isize)
        } else {
            0 as *const uint8_t
        };
        let mut sub: *mut core::ffi::c_int = if channel > 0 as core::ffi::c_int {
            chroma_sub_log2 as *mut core::ffi::c_int
        } else {
            no_subsampling.as_mut_ptr()
        };
        let is_chroma: core::ffi::c_int = (channel != 0 as core::ffi::c_int)
            as core::ffi::c_int;
        if (*data.offset(channel as isize)).is_null()
            || (*denoised.offset(channel as isize)).is_null()
        {
            break;
        }
        if add_block_observations(
            noise_model,
            channel,
            *data.offset(channel as isize),
            *denoised.offset(channel as isize),
            w,
            h,
            *stride.offset(channel as isize),
            sub as *mut core::ffi::c_int,
            alt_data,
            alt_denoised,
            *stride.offset(0 as core::ffi::c_int as isize),
            flat_blocks,
            block_size,
            num_blocks_w,
            num_blocks_h,
        ) == 0
        {
            fprintf(
                __stderrp,
                b"Adding block observation failed\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AOM_NOISE_STATUS_INTERNAL_ERROR as core::ffi::c_int
                as aom_noise_status_t;
        }
        if ar_equation_system_solve(
            &raw mut *((*noise_model).latest_state).as_mut_ptr().offset(channel as isize),
            is_chroma,
        ) == 0
        {
            if is_chroma != 0 {
                set_chroma_coefficient_fallback_soln(
                    &raw mut (*((*noise_model).latest_state)
                        .as_mut_ptr()
                        .offset(channel as isize))
                        .eqns,
                );
            } else {
                fprintf(
                    __stderrp,
                    b"Solving latest noise equation system failed %d!\n\0" as *const u8
                        as *const core::ffi::c_char,
                    channel,
                );
                return AOM_NOISE_STATUS_INTERNAL_ERROR as core::ffi::c_int
                    as aom_noise_status_t;
            }
        }
        add_noise_std_observations(
            noise_model,
            channel,
            (*noise_model).latest_state[channel as usize].eqns.x,
            *data.offset(channel as isize),
            *denoised.offset(channel as isize),
            w,
            h,
            *stride.offset(channel as isize),
            sub as *mut core::ffi::c_int,
            alt_data,
            *stride.offset(0 as core::ffi::c_int as isize),
            flat_blocks,
            block_size,
            num_blocks_w,
            num_blocks_h,
        );
        if aom_noise_strength_solver_solve(
            &raw mut (*((*noise_model).latest_state).as_mut_ptr().offset(channel as isize))
                .strength_solver,
        ) == 0
        {
            fprintf(
                __stderrp,
                b"Solving latest noise strength failed!\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return AOM_NOISE_STATUS_INTERNAL_ERROR as core::ffi::c_int
                as aom_noise_status_t;
        }
        if channel == 0 as core::ffi::c_int
            && (*noise_model)
                .combined_state[channel as usize]
                .strength_solver
                .num_equations > 0 as core::ffi::c_int
            && is_noise_model_different(noise_model) != 0
        {
            y_model_different = 1 as core::ffi::c_int;
        }
        if !(y_model_different != 0) {
            (*noise_model).combined_state[channel as usize].num_observations
                += (*noise_model).latest_state[channel as usize].num_observations;
            equation_system_add(
                &raw mut (*((*noise_model).combined_state)
                    .as_mut_ptr()
                    .offset(channel as isize))
                    .eqns,
                &raw mut (*((*noise_model).latest_state)
                    .as_mut_ptr()
                    .offset(channel as isize))
                    .eqns,
            );
            if ar_equation_system_solve(
                &raw mut *((*noise_model).combined_state)
                    .as_mut_ptr()
                    .offset(channel as isize),
                is_chroma,
            ) == 0
            {
                if is_chroma != 0 {
                    set_chroma_coefficient_fallback_soln(
                        &raw mut (*((*noise_model).combined_state)
                            .as_mut_ptr()
                            .offset(channel as isize))
                            .eqns,
                    );
                } else {
                    fprintf(
                        __stderrp,
                        b"Solving combined noise equation system failed %d!\n\0"
                            as *const u8 as *const core::ffi::c_char,
                        channel,
                    );
                    return AOM_NOISE_STATUS_INTERNAL_ERROR as core::ffi::c_int
                        as aom_noise_status_t;
                }
            }
            noise_strength_solver_add(
                &raw mut (*((*noise_model).combined_state)
                    .as_mut_ptr()
                    .offset(channel as isize))
                    .strength_solver,
                &raw mut (*((*noise_model).latest_state)
                    .as_mut_ptr()
                    .offset(channel as isize))
                    .strength_solver,
            );
            if aom_noise_strength_solver_solve(
                &raw mut (*((*noise_model).combined_state)
                    .as_mut_ptr()
                    .offset(channel as isize))
                    .strength_solver,
            ) == 0
            {
                fprintf(
                    __stderrp,
                    b"Solving combined noise strength failed!\n\0" as *const u8
                        as *const core::ffi::c_char,
                );
                return AOM_NOISE_STATUS_INTERNAL_ERROR as core::ffi::c_int
                    as aom_noise_status_t;
            }
        }
        channel += 1;
    }
    return (if y_model_different != 0 {
        AOM_NOISE_STATUS_DIFFERENT_NOISE_TYPE as core::ffi::c_int
    } else {
        AOM_NOISE_STATUS_OK as core::ffi::c_int
    }) as aom_noise_status_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_model_save_latest(
    mut noise_model: *mut aom_noise_model_t,
) {
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        equation_system_copy(
            &raw mut (*((*noise_model).combined_state).as_mut_ptr().offset(c as isize)).eqns,
            &raw mut (*((*noise_model).latest_state).as_mut_ptr().offset(c as isize)).eqns,
        );
        equation_system_copy(
            &raw mut (*((*noise_model).combined_state).as_mut_ptr().offset(c as isize))
                .strength_solver
                .eqns,
            &raw mut (*((*noise_model).latest_state).as_mut_ptr().offset(c as isize))
                .strength_solver
                .eqns,
        );
        (*noise_model).combined_state[c as usize].strength_solver.num_equations = (*noise_model)
            .latest_state[c as usize]
            .strength_solver
            .num_equations;
        (*noise_model).combined_state[c as usize].num_observations = (*noise_model)
            .latest_state[c as usize]
            .num_observations;
        (*noise_model).combined_state[c as usize].ar_gain = (*noise_model)
            .latest_state[c as usize]
            .ar_gain;
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_model_get_grain_parameters(
    noise_model: *mut aom_noise_model_t,
    mut film_grain: *mut aom_film_grain_t,
) -> core::ffi::c_int {
    if (*noise_model).params.lag > 3 as core::ffi::c_int {
        fprintf(
            __stderrp,
            b"params.lag = %d > 3\n\0" as *const u8 as *const core::ffi::c_char,
            (*noise_model).params.lag,
        );
        return 0 as core::ffi::c_int;
    }
    let mut random_seed: uint16_t = (*film_grain).random_seed;
    memset(
        film_grain as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_film_grain_t>() as size_t,
    );
    (*film_grain).random_seed = random_seed;
    (*film_grain).apply_grain = 1 as core::ffi::c_int;
    (*film_grain).update_parameters = 1 as core::ffi::c_int;
    (*film_grain).ar_coeff_lag = (*noise_model).params.lag;
    let mut scaling_points: [aom_noise_strength_lut_t; 3] = [aom_noise_strength_lut_t {
        points: 0 as *mut [core::ffi::c_double; 2],
        num_points: 0,
    }; 3];
    if aom_noise_strength_solver_fit_piecewise(
        &raw mut (*((*noise_model).combined_state)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .strength_solver,
        14 as core::ffi::c_int,
        scaling_points.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
    ) == 0
    {
        return 0 as core::ffi::c_int;
    }
    if aom_noise_strength_solver_fit_piecewise(
        &raw mut (*((*noise_model).combined_state)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize))
            .strength_solver,
        10 as core::ffi::c_int,
        scaling_points.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
    ) == 0
    {
        aom_noise_strength_lut_free(
            scaling_points.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        );
        return 0 as core::ffi::c_int;
    }
    if aom_noise_strength_solver_fit_piecewise(
        &raw mut (*((*noise_model).combined_state)
            .as_mut_ptr()
            .offset(2 as core::ffi::c_int as isize))
            .strength_solver,
        10 as core::ffi::c_int,
        scaling_points.as_mut_ptr().offset(2 as core::ffi::c_int as isize),
    ) == 0
    {
        aom_noise_strength_lut_free(
            scaling_points.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        );
        aom_noise_strength_lut_free(
            scaling_points.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
        );
        return 0 as core::ffi::c_int;
    }
    let strength_divisor: core::ffi::c_double = ((1 as core::ffi::c_int)
        << (*noise_model).params.bit_depth - 8 as core::ffi::c_int)
        as core::ffi::c_double;
    let mut max_scaling_value: core::ffi::c_double = 1e-4f64;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < scaling_points[c as usize].num_points {
            (*(scaling_points[c as usize].points)
                .offset(i as isize))[0 as core::ffi::c_int as usize] = if (255
                as core::ffi::c_int as core::ffi::c_double)
                < (*(scaling_points[c as usize].points)
                    .offset(i as isize))[0 as core::ffi::c_int as usize]
                    / strength_divisor
            {
                255 as core::ffi::c_int as core::ffi::c_double
            } else {
                (*(scaling_points[c as usize].points)
                    .offset(i as isize))[0 as core::ffi::c_int as usize]
                    / strength_divisor
            };
            (*(scaling_points[c as usize].points)
                .offset(i as isize))[1 as core::ffi::c_int as usize] = if (255
                as core::ffi::c_int as core::ffi::c_double)
                < (*(scaling_points[c as usize].points)
                    .offset(i as isize))[1 as core::ffi::c_int as usize]
                    / strength_divisor
            {
                255 as core::ffi::c_int as core::ffi::c_double
            } else {
                (*(scaling_points[c as usize].points)
                    .offset(i as isize))[1 as core::ffi::c_int as usize]
                    / strength_divisor
            };
            max_scaling_value = if (*(scaling_points[c as usize].points)
                .offset(i as isize))[1 as core::ffi::c_int as usize] > max_scaling_value
            {
                (*(scaling_points[c as usize].points)
                    .offset(i as isize))[1 as core::ffi::c_int as usize]
            } else {
                max_scaling_value
            };
            i += 1;
        }
        c += 1;
    }
    let max_scaling_value_log2: core::ffi::c_int = clamp(
        floor(log2(max_scaling_value) + 1 as core::ffi::c_int as core::ffi::c_double)
            as core::ffi::c_int,
        2 as core::ffi::c_int,
        5 as core::ffi::c_int,
    ) as core::ffi::c_int;
    (*film_grain).scaling_shift = 5 as core::ffi::c_int
        + (8 as core::ffi::c_int - max_scaling_value_log2);
    let scale_factor: core::ffi::c_double = ((1 as core::ffi::c_int)
        << 8 as core::ffi::c_int - max_scaling_value_log2) as core::ffi::c_double;
    (*film_grain).num_y_points = scaling_points[0 as core::ffi::c_int as usize]
        .num_points;
    (*film_grain).num_cb_points = scaling_points[1 as core::ffi::c_int as usize]
        .num_points;
    (*film_grain).num_cr_points = scaling_points[2 as core::ffi::c_int as usize]
        .num_points;
    let mut film_grain_scaling: [*mut [core::ffi::c_int; 2]; 3] = [
        ((*film_grain).scaling_points_y).as_mut_ptr(),
        ((*film_grain).scaling_points_cb).as_mut_ptr(),
        ((*film_grain).scaling_points_cr).as_mut_ptr(),
    ];
    let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_0 < 3 as core::ffi::c_int {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < scaling_points[c_0 as usize].num_points {
            (*(film_grain_scaling[c_0 as usize])
                .offset(i_0 as isize))[0 as core::ffi::c_int as usize] = ((*(scaling_points[c_0
                    as usize]
                .points)
                .offset(i_0 as isize))[0 as core::ffi::c_int as usize] + 0.5f64)
                as core::ffi::c_int;
            (*(film_grain_scaling[c_0 as usize])
                .offset(i_0 as isize))[1 as core::ffi::c_int as usize] = clamp(
                (scale_factor
                    * (*(scaling_points[c_0 as usize].points)
                        .offset(i_0 as isize))[1 as core::ffi::c_int as usize] + 0.5f64)
                    as core::ffi::c_int,
                0 as core::ffi::c_int,
                255 as core::ffi::c_int,
            );
            i_0 += 1;
        }
        c_0 += 1;
    }
    aom_noise_strength_lut_free(
        scaling_points.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
    );
    aom_noise_strength_lut_free(
        scaling_points.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
    );
    aom_noise_strength_lut_free(
        scaling_points.as_mut_ptr().offset(2 as core::ffi::c_int as isize),
    );
    let n_coeff: core::ffi::c_int = (*noise_model)
        .combined_state[0 as core::ffi::c_int as usize]
        .eqns
        .n;
    let mut max_coeff: core::ffi::c_double = 1e-4f64;
    let mut min_coeff: core::ffi::c_double = -1e-4f64;
    let mut y_corr: [core::ffi::c_double; 2] = [
        0 as core::ffi::c_int as core::ffi::c_double,
        0 as core::ffi::c_int as core::ffi::c_double,
    ];
    let mut avg_luma_strength: core::ffi::c_double = 0 as core::ffi::c_int
        as core::ffi::c_double;
    let mut c_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_1 < 3 as core::ffi::c_int {
        let mut eqns: *mut aom_equation_system_t = &raw mut (*((*noise_model).combined_state)
            .as_mut_ptr()
            .offset(c_1 as isize))
            .eqns;
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < n_coeff {
            max_coeff = if max_coeff > *((*eqns).x).offset(i_1 as isize) {
                max_coeff
            } else {
                *((*eqns).x).offset(i_1 as isize)
            };
            min_coeff = if min_coeff < *((*eqns).x).offset(i_1 as isize) {
                min_coeff
            } else {
                *((*eqns).x).offset(i_1 as isize)
            };
            i_1 += 1;
        }
        let mut solver: *mut aom_noise_strength_solver_t = &raw mut (*((*noise_model)
            .combined_state)
            .as_mut_ptr()
            .offset(c_1 as isize))
            .strength_solver;
        let mut average_strength: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut total_weight: core::ffi::c_double = 0 as core::ffi::c_int
            as core::ffi::c_double;
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < (*solver).eqns.n {
            let mut w: core::ffi::c_double = 0 as core::ffi::c_int
                as core::ffi::c_double;
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < (*solver).eqns.n {
                w += *((*solver).eqns.A).offset((i_2 * (*solver).eqns.n + j) as isize);
                j += 1;
            }
            w = sqrt(w);
            average_strength += *((*solver).eqns.x).offset(i_2 as isize) * w;
            total_weight += w;
            i_2 += 1;
        }
        if total_weight == 0 as core::ffi::c_int as core::ffi::c_double {
            average_strength = 1 as core::ffi::c_int as core::ffi::c_double;
        } else {
            average_strength /= total_weight;
        }
        if c_1 == 0 as core::ffi::c_int {
            avg_luma_strength = average_strength;
        } else {
            y_corr[(c_1 - 1 as core::ffi::c_int) as usize] = if average_strength
                > 1e-6f64
            {
                avg_luma_strength * *((*eqns).x).offset(n_coeff as isize)
                    / average_strength
            } else {
                0 as core::ffi::c_int as core::ffi::c_double
            };
            max_coeff = if max_coeff > y_corr[(c_1 - 1 as core::ffi::c_int) as usize] {
                max_coeff
            } else {
                y_corr[(c_1 - 1 as core::ffi::c_int) as usize]
            };
            min_coeff = if min_coeff < y_corr[(c_1 - 1 as core::ffi::c_int) as usize] {
                min_coeff
            } else {
                y_corr[(c_1 - 1 as core::ffi::c_int) as usize]
            };
        }
        c_1 += 1;
    }
    (*film_grain).ar_coeff_shift = clamp(
        7 as core::ffi::c_int
            - (if 1 as core::ffi::c_int as core::ffi::c_double + floor(log2(max_coeff))
                > ceil(log2(-min_coeff))
            {
                1 as core::ffi::c_int as core::ffi::c_double + floor(log2(max_coeff))
            } else {
                ceil(log2(-min_coeff))
            }) as core::ffi::c_int,
        6 as core::ffi::c_int,
        9 as core::ffi::c_int,
    );
    let mut scale_ar_coeff: core::ffi::c_double = ((1 as core::ffi::c_int)
        << (*film_grain).ar_coeff_shift) as core::ffi::c_double;
    let mut ar_coeffs: [*mut core::ffi::c_int; 3] = [
        ((*film_grain).ar_coeffs_y).as_mut_ptr(),
        ((*film_grain).ar_coeffs_cb).as_mut_ptr(),
        ((*film_grain).ar_coeffs_cr).as_mut_ptr(),
    ];
    let mut c_2: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_2 < 3 as core::ffi::c_int {
        let mut eqns_0: *mut aom_equation_system_t = &raw mut (*((*noise_model)
            .combined_state)
            .as_mut_ptr()
            .offset(c_2 as isize))
            .eqns;
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 < n_coeff {
            *(ar_coeffs[c_2 as usize]).offset(i_3 as isize) = clamp(
                round(scale_ar_coeff * *((*eqns_0).x).offset(i_3 as isize))
                    as core::ffi::c_int,
                -(128 as core::ffi::c_int),
                127 as core::ffi::c_int,
            );
            i_3 += 1;
        }
        if c_2 > 0 as core::ffi::c_int {
            *(ar_coeffs[c_2 as usize]).offset(n_coeff as isize) = clamp(
                round(scale_ar_coeff * y_corr[(c_2 - 1 as core::ffi::c_int) as usize])
                    as core::ffi::c_int,
                -(128 as core::ffi::c_int),
                127 as core::ffi::c_int,
            );
        }
        c_2 += 1;
    }
    (*film_grain).cb_mult = 128 as core::ffi::c_int;
    (*film_grain).cb_luma_mult = 192 as core::ffi::c_int;
    (*film_grain).cb_offset = 256 as core::ffi::c_int;
    (*film_grain).cr_mult = 128 as core::ffi::c_int;
    (*film_grain).cr_luma_mult = 192 as core::ffi::c_int;
    (*film_grain).cr_offset = 256 as core::ffi::c_int;
    (*film_grain).chroma_scaling_from_luma = 0 as core::ffi::c_int;
    (*film_grain).grain_scale_shift = 0 as core::ffi::c_int;
    (*film_grain).overlap_flag = 1 as core::ffi::c_int;
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn pointwise_multiply(
    mut a: *const core::ffi::c_float,
    mut b: *mut core::ffi::c_float,
    mut n: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n {
        *b.offset(i as isize) *= *a.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn get_half_cos_window(
    mut block_size: core::ffi::c_int,
) -> *mut core::ffi::c_float {
    let mut window_function: *mut core::ffi::c_float = aom_malloc(
        ((block_size * block_size) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    if window_function.is_null() {
        return 0 as *mut core::ffi::c_float;
    }
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < block_size {
        let cos_yd: core::ffi::c_double = cos(
            (0.5f64 + y as core::ffi::c_double) * PI / block_size as core::ffi::c_double
                - PI / 2 as core::ffi::c_int as core::ffi::c_double,
        ) as core::ffi::c_double;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < block_size {
            let cos_xd: core::ffi::c_double = cos(
                (0.5f64 + x as core::ffi::c_double) * PI
                    / block_size as core::ffi::c_double
                    - PI / 2 as core::ffi::c_int as core::ffi::c_double,
            ) as core::ffi::c_double;
            *window_function.offset((y * block_size + x) as isize) = (cos_yd * cos_xd)
                as core::ffi::c_float;
            x += 1;
        }
        y += 1;
    }
    return window_function;
}
unsafe extern "C-unwind" fn dither_and_quantize_lowbd(
    mut result: *mut core::ffi::c_float,
    mut result_stride: core::ffi::c_int,
    mut denoised: *mut uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut chroma_sub_w: core::ffi::c_int,
    mut chroma_sub_h: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut block_normalization: core::ffi::c_float,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h >> chroma_sub_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w >> chroma_sub_w {
            let result_idx: core::ffi::c_int = (y + (block_size >> chroma_sub_h))
                * result_stride + x + (block_size >> chroma_sub_w);
            let mut new_val: uint8_t = (if (if *result.offset(result_idx as isize)
                * block_normalization + 0.5f32
                > 0 as core::ffi::c_int as core::ffi::c_float
            {
                *result.offset(result_idx as isize) * block_normalization + 0.5f32
            } else {
                0 as core::ffi::c_int as core::ffi::c_float
            }) < block_normalization
            {
                if *result.offset(result_idx as isize) * block_normalization + 0.5f32
                    > 0 as core::ffi::c_int as core::ffi::c_float
                {
                    *result.offset(result_idx as isize) * block_normalization + 0.5f32
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                }
            } else {
                block_normalization
            }) as uint8_t;
            let mut err: core::ffi::c_float = -(new_val as core::ffi::c_float
                / block_normalization - *result.offset(result_idx as isize));
            if fabsf(err) < 1e-6f32 {
                err = 0.0f32;
            }
            *denoised.offset((y * stride + x) as isize) = new_val;
            if (x + 1 as core::ffi::c_int) < w >> chroma_sub_w {
                *result.offset((result_idx + 1 as core::ffi::c_int) as isize)
                    += err * 7.0f32 / 16.0f32;
            }
            if (y + 1 as core::ffi::c_int) < h >> chroma_sub_h {
                if x > 0 as core::ffi::c_int {
                    *result
                        .offset(
                            (result_idx + result_stride - 1 as core::ffi::c_int) as isize,
                        ) += err * 3.0f32 / 16.0f32;
                }
                *result.offset((result_idx + result_stride) as isize)
                    += err * 5.0f32 / 16.0f32;
                if (x + 1 as core::ffi::c_int) < w >> chroma_sub_w {
                    *result
                        .offset(
                            (result_idx + result_stride + 1 as core::ffi::c_int) as isize,
                        ) += err * 1.0f32 / 16.0f32;
                }
            }
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C-unwind" fn dither_and_quantize_highbd(
    mut result: *mut core::ffi::c_float,
    mut result_stride: core::ffi::c_int,
    mut denoised: *mut uint16_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut chroma_sub_w: core::ffi::c_int,
    mut chroma_sub_h: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut block_normalization: core::ffi::c_float,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h >> chroma_sub_h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w >> chroma_sub_w {
            let result_idx: core::ffi::c_int = (y + (block_size >> chroma_sub_h))
                * result_stride + x + (block_size >> chroma_sub_w);
            let mut new_val: uint16_t = (if (if *result.offset(result_idx as isize)
                * block_normalization + 0.5f32
                > 0 as core::ffi::c_int as core::ffi::c_float
            {
                *result.offset(result_idx as isize) * block_normalization + 0.5f32
            } else {
                0 as core::ffi::c_int as core::ffi::c_float
            }) < block_normalization
            {
                if *result.offset(result_idx as isize) * block_normalization + 0.5f32
                    > 0 as core::ffi::c_int as core::ffi::c_float
                {
                    *result.offset(result_idx as isize) * block_normalization + 0.5f32
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                }
            } else {
                block_normalization
            }) as uint16_t;
            let mut err: core::ffi::c_float = -(new_val as core::ffi::c_float
                / block_normalization - *result.offset(result_idx as isize));
            if fabsf(err) < 1e-6f32 {
                err = 0.0f32;
            }
            *denoised.offset((y * stride + x) as isize) = new_val;
            if (x + 1 as core::ffi::c_int) < w >> chroma_sub_w {
                *result.offset((result_idx + 1 as core::ffi::c_int) as isize)
                    += err * 7.0f32 / 16.0f32;
            }
            if (y + 1 as core::ffi::c_int) < h >> chroma_sub_h {
                if x > 0 as core::ffi::c_int {
                    *result
                        .offset(
                            (result_idx + result_stride - 1 as core::ffi::c_int) as isize,
                        ) += err * 3.0f32 / 16.0f32;
                }
                *result.offset((result_idx + result_stride) as isize)
                    += err * 5.0f32 / 16.0f32;
                if (x + 1 as core::ffi::c_int) < w >> chroma_sub_w {
                    *result
                        .offset(
                            (result_idx + result_stride + 1 as core::ffi::c_int) as isize,
                        ) += err * 1.0f32 / 16.0f32;
                }
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_wiener_denoise_2d(
    mut data: *const *const uint8_t,
    mut denoised: *mut *mut uint8_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut stride: *mut core::ffi::c_int,
    mut chroma_sub: *mut core::ffi::c_int,
    mut noise_psd: *mut *mut core::ffi::c_float,
    mut block_size: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
    mut use_highbd: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut plane: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut block: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut window_full: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut window_chroma: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut block_d: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut plane_d: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut tx_full: *mut aom_noise_tx_t = 0 as *mut aom_noise_tx_t;
    let mut tx_chroma: *mut aom_noise_tx_t = 0 as *mut aom_noise_tx_t;
    let num_blocks_w: core::ffi::c_int = (w + block_size - 1 as core::ffi::c_int)
        / block_size;
    let num_blocks_h: core::ffi::c_int = (h + block_size - 1 as core::ffi::c_int)
        / block_size;
    let result_stride: core::ffi::c_int = (num_blocks_w + 2 as core::ffi::c_int)
        * block_size;
    let result_height: core::ffi::c_int = (num_blocks_h + 2 as core::ffi::c_int)
        * block_size;
    let mut result: *mut core::ffi::c_float = 0 as *mut core::ffi::c_float;
    let mut init_success: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut block_finder_full: aom_flat_block_finder_t = aom_flat_block_finder_t {
        AtA_inv: 0 as *mut core::ffi::c_double,
        A: 0 as *mut core::ffi::c_double,
        num_params: 0,
        block_size: 0,
        normalization: 0.,
        use_highbd: 0,
    };
    let mut block_finder_chroma: aom_flat_block_finder_t = aom_flat_block_finder_t {
        AtA_inv: 0 as *mut core::ffi::c_double,
        A: 0 as *mut core::ffi::c_double,
        num_params: 0,
        block_size: 0,
        normalization: 0.,
        use_highbd: 0,
    };
    let kBlockNormalization: core::ffi::c_float = (((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int) as core::ffi::c_float;
    if *chroma_sub.offset(0 as core::ffi::c_int as isize)
        != *chroma_sub.offset(1 as core::ffi::c_int as isize)
    {
        fprintf(
            __stderrp,
            b"aom_wiener_denoise_2d doesn't handle different chroma subsampling\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    init_success
        &= aom_flat_block_finder_init(
            &raw mut block_finder_full,
            block_size,
            bit_depth,
            use_highbd,
        );
    result = aom_malloc(
        (((num_blocks_h + 2 as core::ffi::c_int) * block_size * result_stride) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    plane = aom_malloc(
        ((block_size * block_size) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    block = aom_memalign(
        32 as size_t,
        ((2 as core::ffi::c_int * block_size * block_size) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    block_d = aom_malloc(
        ((block_size * block_size) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    plane_d = aom_malloc(
        ((block_size * block_size) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_double>() as size_t),
    ) as *mut core::ffi::c_double;
    window_full = get_half_cos_window(block_size);
    tx_full = aom_noise_tx_malloc(block_size);
    if *chroma_sub.offset(0 as core::ffi::c_int as isize) != 0 as core::ffi::c_int {
        init_success
            &= aom_flat_block_finder_init(
                &raw mut block_finder_chroma,
                block_size >> *chroma_sub.offset(0 as core::ffi::c_int as isize),
                bit_depth,
                use_highbd,
            );
        window_chroma = get_half_cos_window(
            block_size >> *chroma_sub.offset(0 as core::ffi::c_int as isize),
        );
        tx_chroma = aom_noise_tx_malloc(
            block_size >> *chroma_sub.offset(0 as core::ffi::c_int as isize),
        );
    } else {
        window_chroma = window_full;
        tx_chroma = tx_full;
    }
    init_success
        &= (!tx_full.is_null() && !tx_chroma.is_null() && !plane.is_null()
            && !plane_d.is_null() && !block.is_null() && !block_d.is_null()
            && !window_full.is_null() && !window_chroma.is_null() && !result.is_null())
            as core::ffi::c_int;
    let mut c: core::ffi::c_int = if init_success != 0 {
        0 as core::ffi::c_int
    } else {
        3 as core::ffi::c_int
    };
    while c < 3 as core::ffi::c_int {
        let mut window_function: *mut core::ffi::c_float = if c == 0 as core::ffi::c_int
        {
            window_full
        } else {
            window_chroma
        };
        let mut block_finder: *mut aom_flat_block_finder_t = &raw mut block_finder_full;
        let chroma_sub_h: core::ffi::c_int = if c > 0 as core::ffi::c_int {
            *chroma_sub.offset(1 as core::ffi::c_int as isize)
        } else {
            0 as core::ffi::c_int
        };
        let chroma_sub_w: core::ffi::c_int = if c > 0 as core::ffi::c_int {
            *chroma_sub.offset(0 as core::ffi::c_int as isize)
        } else {
            0 as core::ffi::c_int
        };
        let mut tx: *mut aom_noise_tx_t = if c > 0 as core::ffi::c_int
            && *chroma_sub.offset(0 as core::ffi::c_int as isize) > 0 as core::ffi::c_int
        {
            tx_chroma
        } else {
            tx_full
        };
        if !((*data.offset(c as isize)).is_null()
            || (*denoised.offset(c as isize)).is_null())
        {
            if c > 0 as core::ffi::c_int
                && *chroma_sub.offset(0 as core::ffi::c_int as isize)
                    != 0 as core::ffi::c_int
            {
                block_finder = &raw mut block_finder_chroma;
            }
            memset(
                result as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<core::ffi::c_float>() as size_t)
                    .wrapping_mul(result_stride as size_t)
                    .wrapping_mul(result_height as size_t),
            );
            let mut offsy: core::ffi::c_int = 0 as core::ffi::c_int;
            while offsy < block_size >> chroma_sub_h {
                let mut offsx: core::ffi::c_int = 0 as core::ffi::c_int;
                while offsx < block_size >> chroma_sub_w {
                    let mut by: core::ffi::c_int = -(1 as core::ffi::c_int);
                    while by < num_blocks_h {
                        let mut bx: core::ffi::c_int = -(1 as core::ffi::c_int);
                        while bx < num_blocks_w {
                            let pixels_per_block: core::ffi::c_int = (block_size
                                >> chroma_sub_w) * (block_size >> chroma_sub_h);
                            aom_flat_block_finder_extract_block(
                                block_finder,
                                *data.offset(c as isize),
                                w >> chroma_sub_w,
                                h >> chroma_sub_h,
                                *stride.offset(c as isize),
                                bx * (block_size >> chroma_sub_w) + offsx,
                                by * (block_size >> chroma_sub_h) + offsy,
                                plane_d,
                                block_d,
                            );
                            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                            while j < pixels_per_block {
                                *block.offset(j as isize) = *block_d.offset(j as isize)
                                    as core::ffi::c_float;
                                *plane.offset(j as isize) = *plane_d.offset(j as isize)
                                    as core::ffi::c_float;
                                j += 1;
                            }
                            pointwise_multiply(window_function, block, pixels_per_block);
                            aom_noise_tx_forward(tx, block);
                            aom_noise_tx_filter(tx, *noise_psd.offset(c as isize));
                            aom_noise_tx_inverse(tx, block);
                            pointwise_multiply(window_function, plane, pixels_per_block);
                            let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
                            while y < block_size >> chroma_sub_h {
                                let y_result: core::ffi::c_int = y
                                    + (by + 1 as core::ffi::c_int)
                                        * (block_size >> chroma_sub_h) + offsy;
                                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                                while x < block_size >> chroma_sub_w {
                                    let x_result: core::ffi::c_int = x
                                        + (bx + 1 as core::ffi::c_int)
                                            * (block_size >> chroma_sub_w) + offsx;
                                    *result
                                        .offset((y_result * result_stride + x_result) as isize)
                                        += (*block
                                            .offset((y * (block_size >> chroma_sub_w) + x) as isize)
                                            + *plane
                                                .offset((y * (block_size >> chroma_sub_w) + x) as isize))
                                            * *window_function
                                                .offset((y * (block_size >> chroma_sub_w) + x) as isize);
                                    x += 1;
                                }
                                y += 1;
                            }
                            bx += 1;
                        }
                        by += 1;
                    }
                    offsx += (block_size >> chroma_sub_w) / 2 as core::ffi::c_int;
                }
                offsy += (block_size >> chroma_sub_h) / 2 as core::ffi::c_int;
            }
            if use_highbd != 0 {
                dither_and_quantize_highbd(
                    result,
                    result_stride,
                    *denoised.offset(c as isize) as *mut uint16_t,
                    w,
                    h,
                    *stride.offset(c as isize),
                    chroma_sub_w,
                    chroma_sub_h,
                    block_size,
                    kBlockNormalization,
                );
            } else {
                dither_and_quantize_lowbd(
                    result,
                    result_stride,
                    *denoised.offset(c as isize),
                    w,
                    h,
                    *stride.offset(c as isize),
                    chroma_sub_w,
                    chroma_sub_h,
                    block_size,
                    kBlockNormalization,
                );
            }
        }
        c += 1;
    }
    aom_free(result as *mut core::ffi::c_void);
    aom_free(plane as *mut core::ffi::c_void);
    aom_free(block as *mut core::ffi::c_void);
    aom_free(plane_d as *mut core::ffi::c_void);
    aom_free(block_d as *mut core::ffi::c_void);
    aom_free(window_full as *mut core::ffi::c_void);
    aom_noise_tx_free(tx_full);
    aom_flat_block_finder_free(&raw mut block_finder_full);
    if *chroma_sub.offset(0 as core::ffi::c_int as isize) != 0 as core::ffi::c_int {
        aom_flat_block_finder_free(&raw mut block_finder_chroma);
        aom_free(window_chroma as *mut core::ffi::c_void);
        aom_noise_tx_free(tx_chroma);
    }
    return init_success;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_denoise_and_model_alloc(
    mut bit_depth: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut noise_level: core::ffi::c_float,
) -> *mut aom_denoise_and_model_t {
    let mut ctx: *mut aom_denoise_and_model_t = aom_malloc(
        ::core::mem::size_of::<aom_denoise_and_model_t>() as size_t,
    ) as *mut aom_denoise_and_model_t;
    if ctx.is_null() {
        fprintf(
            __stderrp,
            b"Unable to allocate denoise_and_model struct\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as *mut aom_denoise_and_model_t;
    }
    memset(
        ctx as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_denoise_and_model_t>() as size_t,
    );
    (*ctx).block_size = block_size;
    (*ctx).noise_level = noise_level;
    (*ctx).bit_depth = bit_depth;
    (*ctx).noise_psd[0 as core::ffi::c_int as usize] = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    ) as *mut core::ffi::c_float;
    (*ctx).noise_psd[1 as core::ffi::c_int as usize] = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    ) as *mut core::ffi::c_float;
    (*ctx).noise_psd[2 as core::ffi::c_int as usize] = aom_malloc(
        (::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    ) as *mut core::ffi::c_float;
    if ((*ctx).noise_psd[0 as core::ffi::c_int as usize]).is_null()
        || ((*ctx).noise_psd[1 as core::ffi::c_int as usize]).is_null()
        || ((*ctx).noise_psd[2 as core::ffi::c_int as usize]).is_null()
    {
        fprintf(
            __stderrp,
            b"Unable to allocate noise PSD buffers\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        aom_denoise_and_model_free(ctx);
        return 0 as *mut aom_denoise_and_model_t;
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_denoise_and_model_free(
    mut ctx: *mut aom_denoise_and_model_t,
) {
    aom_free((*ctx).flat_blocks as *mut core::ffi::c_void);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        aom_free((*ctx).denoised[i as usize] as *mut core::ffi::c_void);
        aom_free((*ctx).noise_psd[i as usize] as *mut core::ffi::c_void);
        i += 1;
    }
    aom_noise_model_free(&raw mut (*ctx).noise_model);
    aom_flat_block_finder_free(&raw mut (*ctx).flat_block_finder);
    aom_free(ctx as *mut core::ffi::c_void);
}
unsafe extern "C-unwind" fn denoise_and_model_realloc_if_necessary(
    mut ctx: *mut aom_denoise_and_model_t,
    mut sd: *const YV12_BUFFER_CONFIG,
) -> core::ffi::c_int {
    if (*ctx).width == (*sd).c2rust_unnamed.c2rust_unnamed.y_width
        && (*ctx).height == (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height
        && (*ctx).y_stride == (*sd).c2rust_unnamed_3.c2rust_unnamed.y_stride
        && (*ctx).uv_stride == (*sd).c2rust_unnamed_3.c2rust_unnamed.uv_stride
    {
        return 1 as core::ffi::c_int;
    }
    let use_highbd: core::ffi::c_int = ((*sd).flags & YV12_FLAG_HIGHBITDEPTH
        != 0 as core::ffi::c_int) as core::ffi::c_int;
    let block_size: core::ffi::c_int = (*ctx).block_size;
    (*ctx).width = (*sd).c2rust_unnamed.c2rust_unnamed.y_width;
    (*ctx).height = (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height;
    (*ctx).y_stride = (*sd).c2rust_unnamed_3.c2rust_unnamed.y_stride;
    (*ctx).uv_stride = (*sd).c2rust_unnamed_3.c2rust_unnamed.uv_stride;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 3 as core::ffi::c_int {
        aom_free((*ctx).denoised[i as usize] as *mut core::ffi::c_void);
        (*ctx).denoised[i as usize] = 0 as *mut uint8_t;
        i += 1;
    }
    aom_free((*ctx).flat_blocks as *mut core::ffi::c_void);
    (*ctx).flat_blocks = 0 as *mut uint8_t;
    (*ctx).denoised[0 as core::ffi::c_int as usize] = aom_malloc(
        ((*sd).c2rust_unnamed_3.c2rust_unnamed.y_stride
            * (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height << use_highbd) as size_t,
    ) as *mut uint8_t;
    (*ctx).denoised[1 as core::ffi::c_int as usize] = aom_malloc(
        ((*sd).c2rust_unnamed_3.c2rust_unnamed.uv_stride
            * (*sd).c2rust_unnamed_0.c2rust_unnamed.uv_height << use_highbd) as size_t,
    ) as *mut uint8_t;
    (*ctx).denoised[2 as core::ffi::c_int as usize] = aom_malloc(
        ((*sd).c2rust_unnamed_3.c2rust_unnamed.uv_stride
            * (*sd).c2rust_unnamed_0.c2rust_unnamed.uv_height << use_highbd) as size_t,
    ) as *mut uint8_t;
    if ((*ctx).denoised[0 as core::ffi::c_int as usize]).is_null()
        || ((*ctx).denoised[1 as core::ffi::c_int as usize]).is_null()
        || ((*ctx).denoised[2 as core::ffi::c_int as usize]).is_null()
    {
        fprintf(
            __stderrp,
            b"Unable to allocate denoise buffers\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    (*ctx).num_blocks_w = ((*sd).c2rust_unnamed.c2rust_unnamed.y_width
        + (*ctx).block_size - 1 as core::ffi::c_int) / (*ctx).block_size;
    (*ctx).num_blocks_h = ((*sd).c2rust_unnamed_0.c2rust_unnamed.y_height
        + (*ctx).block_size - 1 as core::ffi::c_int) / (*ctx).block_size;
    (*ctx).flat_blocks = aom_malloc(
        ((*ctx).num_blocks_w * (*ctx).num_blocks_h) as size_t,
    ) as *mut uint8_t;
    if ((*ctx).flat_blocks).is_null() {
        fprintf(
            __stderrp,
            b"Unable to allocate flat_blocks buffer\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    aom_flat_block_finder_free(&raw mut (*ctx).flat_block_finder);
    if aom_flat_block_finder_init(
        &raw mut (*ctx).flat_block_finder,
        (*ctx).block_size,
        (*ctx).bit_depth,
        use_highbd,
    ) == 0
    {
        fprintf(
            __stderrp,
            b"Unable to init flat block finder\n\0" as *const u8
                as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    let params: aom_noise_model_params_t = {
        let mut init = aom_noise_model_params_t {
            shape: AOM_NOISE_SHAPE_SQUARE as core::ffi::c_int as aom_noise_shape,
            lag: 3 as core::ffi::c_int,
            bit_depth: (*ctx).bit_depth,
            use_highbd: use_highbd,
        };
        init
    };
    aom_noise_model_free(&raw mut (*ctx).noise_model);
    if aom_noise_model_init(&raw mut (*ctx).noise_model, params) == 0 {
        fprintf(
            __stderrp,
            b"Unable to init noise model\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    let y_noise_level: core::ffi::c_float = aom_noise_psd_get_default_value(
        (*ctx).block_size,
        (*ctx).noise_level,
    ) as core::ffi::c_float;
    let uv_noise_level: core::ffi::c_float = aom_noise_psd_get_default_value(
        (*ctx).block_size >> (*sd).subsampling_x,
        (*ctx).noise_level,
    ) as core::ffi::c_float;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < block_size * block_size {
        *((*ctx).noise_psd[0 as core::ffi::c_int as usize]).offset(i_0 as isize) = y_noise_level;
        let fresh2 = &raw mut *((*ctx).noise_psd[2 as core::ffi::c_int as usize])
            .offset(i_0 as isize);
        *fresh2 = uv_noise_level;
        *((*ctx).noise_psd[1 as core::ffi::c_int as usize]).offset(i_0 as isize) = *fresh2;
        i_0 += 1;
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_denoise_and_model_run(
    mut ctx: *mut aom_denoise_and_model_t,
    mut sd: *const YV12_BUFFER_CONFIG,
    mut film_grain: *mut aom_film_grain_t,
    mut apply_denoise: core::ffi::c_int,
) -> core::ffi::c_int {
    let block_size: core::ffi::c_int = (*ctx).block_size;
    let use_highbd: core::ffi::c_int = ((*sd).flags & YV12_FLAG_HIGHBITDEPTH
        != 0 as core::ffi::c_int) as core::ffi::c_int;
    let mut raw_data: [*mut uint8_t; 3] = [
        if use_highbd != 0 {
            (((*sd).c2rust_unnamed_4.c2rust_unnamed.y_buffer as uintptr_t)
                << 1 as core::ffi::c_int) as *mut uint16_t as *mut uint8_t
        } else {
            (*sd).c2rust_unnamed_4.c2rust_unnamed.y_buffer
        },
        if use_highbd != 0 {
            (((*sd).c2rust_unnamed_4.c2rust_unnamed.u_buffer as uintptr_t)
                << 1 as core::ffi::c_int) as *mut uint16_t as *mut uint8_t
        } else {
            (*sd).c2rust_unnamed_4.c2rust_unnamed.u_buffer
        },
        if use_highbd != 0 {
            (((*sd).c2rust_unnamed_4.c2rust_unnamed.v_buffer as uintptr_t)
                << 1 as core::ffi::c_int) as *mut uint16_t as *mut uint8_t
        } else {
            (*sd).c2rust_unnamed_4.c2rust_unnamed.v_buffer
        },
    ];
    let data: [*const uint8_t; 3] = [
        raw_data[0 as core::ffi::c_int as usize] as *const uint8_t,
        raw_data[1 as core::ffi::c_int as usize] as *const uint8_t,
        raw_data[2 as core::ffi::c_int as usize] as *const uint8_t,
    ];
    let mut strides: [core::ffi::c_int; 3] = [
        (*sd).c2rust_unnamed_3.c2rust_unnamed.y_stride,
        (*sd).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
        (*sd).c2rust_unnamed_3.c2rust_unnamed.uv_stride,
    ];
    let mut chroma_sub_log2: [core::ffi::c_int; 2] = [
        (*sd).subsampling_x,
        (*sd).subsampling_y,
    ];
    if denoise_and_model_realloc_if_necessary(ctx, sd) == 0 {
        fprintf(
            __stderrp,
            b"Unable to realloc buffers\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    aom_flat_block_finder_run(
        &raw mut (*ctx).flat_block_finder,
        data[0 as core::ffi::c_int as usize],
        (*sd).c2rust_unnamed.c2rust_unnamed.y_width,
        (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height,
        strides[0 as core::ffi::c_int as usize],
        (*ctx).flat_blocks,
    );
    if aom_wiener_denoise_2d(
        data.as_ptr(),
        ((*ctx).denoised).as_mut_ptr(),
        (*sd).c2rust_unnamed.c2rust_unnamed.y_width,
        (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height,
        strides.as_mut_ptr(),
        chroma_sub_log2.as_mut_ptr(),
        ((*ctx).noise_psd).as_mut_ptr(),
        block_size,
        (*ctx).bit_depth,
        use_highbd,
    ) == 0
    {
        fprintf(
            __stderrp,
            b"Unable to denoise image\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return 0 as core::ffi::c_int;
    }
    let status: aom_noise_status_t = aom_noise_model_update(
        &raw mut (*ctx).noise_model,
        data.as_ptr(),
        ((*ctx).denoised).as_mut_ptr() as *const *const uint8_t,
        (*sd).c2rust_unnamed.c2rust_unnamed.y_width,
        (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height,
        strides.as_mut_ptr(),
        chroma_sub_log2.as_mut_ptr(),
        (*ctx).flat_blocks,
        block_size,
    ) as aom_noise_status_t;
    let mut have_noise_estimate: core::ffi::c_int = 0 as core::ffi::c_int;
    if status as core::ffi::c_int == AOM_NOISE_STATUS_OK as core::ffi::c_int {
        have_noise_estimate = 1 as core::ffi::c_int;
    } else if status as core::ffi::c_int
        == AOM_NOISE_STATUS_DIFFERENT_NOISE_TYPE as core::ffi::c_int
    {
        aom_noise_model_save_latest(&raw mut (*ctx).noise_model);
        have_noise_estimate = 1 as core::ffi::c_int;
    } else {
        have_noise_estimate = ((*ctx)
            .noise_model
            .combined_state[0 as core::ffi::c_int as usize]
            .strength_solver
            .num_equations > 0 as core::ffi::c_int) as core::ffi::c_int;
    }
    (*film_grain).apply_grain = 0 as core::ffi::c_int;
    if have_noise_estimate != 0 {
        if aom_noise_model_get_grain_parameters(&raw mut (*ctx).noise_model, film_grain) == 0
        {
            fprintf(
                __stderrp,
                b"Unable to get grain parameters.\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            return 0 as core::ffi::c_int;
        }
        if (*film_grain).random_seed == 0 {
            (*film_grain).random_seed = 7391 as uint16_t;
        }
        if apply_denoise != 0 {
            memcpy(
                raw_data[0 as core::ffi::c_int as usize] as *mut core::ffi::c_void,
                (*ctx).denoised[0 as core::ffi::c_int as usize]
                    as *const core::ffi::c_void,
                (strides[0 as core::ffi::c_int as usize]
                    * (*sd).c2rust_unnamed_0.c2rust_unnamed.y_height << use_highbd)
                    as size_t,
            );
            if (*sd).monochrome == 0 {
                memcpy(
                    raw_data[1 as core::ffi::c_int as usize] as *mut core::ffi::c_void,
                    (*ctx).denoised[1 as core::ffi::c_int as usize]
                        as *const core::ffi::c_void,
                    (strides[1 as core::ffi::c_int as usize]
                        * (*sd).c2rust_unnamed_0.c2rust_unnamed.uv_height << use_highbd)
                        as size_t,
                );
                memcpy(
                    raw_data[2 as core::ffi::c_int as usize] as *mut core::ffi::c_void,
                    (*ctx).denoised[2 as core::ffi::c_int as usize]
                        as *const core::ffi::c_void,
                    (strides[2 as core::ffi::c_int as usize]
                        * (*sd).c2rust_unnamed_0.c2rust_unnamed.uv_height << use_highbd)
                        as size_t,
                );
            }
        }
    }
    return 1 as core::ffi::c_int;
}

extern "C" {
    fn cos(_: core::ffi::c_double) -> core::ffi::c_double;
    fn exp(_: core::ffi::c_double) -> core::ffi::c_double;
    fn log2(_: core::ffi::c_double) -> core::ffi::c_double;
    fn fabsf(_: core::ffi::c_float) -> core::ffi::c_float;
    fn fabs(_: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(_: core::ffi::c_double, _: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(_: core::ffi::c_double) -> core::ffi::c_double;
    fn ceil(_: core::ffi::c_double) -> core::ffi::c_double;
    fn floor(_: core::ffi::c_double) -> core::ffi::c_double;
    fn round(_: core::ffi::c_double) -> core::ffi::c_double;
    fn fprintf(_: *mut FILE, _: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const core::ffi::c_void,
                *const core::ffi::c_void,
            ) -> core::ffi::c_int,
        >,
    );
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
