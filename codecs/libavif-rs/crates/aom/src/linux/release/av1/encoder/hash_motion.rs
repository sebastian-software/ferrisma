/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/encoder/hash_motion.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn av1_get_crc32c_value_c(
        crc_calculator: *mut core::ffi::c_void,
        p: *const uint8_t,
        length: size_t,
    ) -> uint32_t;
    fn av1_crc32c_calculator_init(p_crc32c: *mut CRC32C);
    fn aom_vector_setup(
        vector: *mut Vector,
        capacity: size_t,
        element_size: size_t,
    ) -> core::ffi::c_int;
    fn aom_vector_destroy(vector: *mut Vector) -> core::ffi::c_int;
    fn aom_vector_push_back(
        vector: *mut Vector,
        element: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn aom_vector_byte_size(vector: *const Vector) -> size_t;
    fn aom_vector_begin(vector: *mut Vector) -> Iterator_0;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type CRC32C = _CRC32C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CRC32C {
    pub table: [[uint32_t; 256]; 8],
}
pub type IntraBCHashInfo = intrabc_hash_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intrabc_hash_info {
    pub hash_value_buffer: [*mut uint32_t; 2],
    pub intrabc_hash_table: hash_table,
    pub crc_calculator: CRC32C,
    pub crc_initialized: core::ffi::c_int,
}
pub type hash_table = _hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hash_table {
    pub p_lookup_table: *mut *mut Vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
    pub size: size_t,
    pub capacity: size_t,
    pub element_size: size_t,
    pub data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Iterator_0 {
    pub pointer: *mut core::ffi::c_void,
    pub element_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_hash {
    pub x: int16_t,
    pub y: int16_t,
    pub hash_value2: uint32_t,
}
pub type block_hash = _block_hash;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const VECTOR_ERROR: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const kSrcBits: core::ffi::c_int = 16 as core::ffi::c_int;
pub const kMaxAddr: core::ffi::c_int = (6 as core::ffi::c_int) << kSrcBits;
pub const kMaxCandidatesPerHashBucket: core::ffi::c_int = 256 as core::ffi::c_int;
unsafe extern "C-unwind" fn get_pixels_in_1D_char_array_by_block_2x2(
    mut y_src: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut p_pixels_in1D: *mut uint8_t,
) {
    let mut p_pel: *const uint8_t = y_src;
    let mut index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 2 as core::ffi::c_int {
            let fresh1 = index;
            index = index + 1;
            *p_pixels_in1D.offset(fresh1 as isize) = *p_pel.offset(j as isize);
            j += 1;
        }
        p_pel = p_pel.offset(stride as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn get_pixels_in_1D_short_array_by_block_2x2(
    mut y_src: *const uint16_t,
    mut stride: core::ffi::c_int,
    mut p_pixels_in1D: *mut uint16_t,
) {
    let mut p_pel: *const uint16_t = y_src;
    let mut index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 2 as core::ffi::c_int {
            let fresh2 = index;
            index = index + 1;
            *p_pixels_in1D.offset(fresh2 as isize) = *p_pel.offset(j as isize);
            j += 1;
        }
        p_pel = p_pel.offset(stride as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn hash_block_size_to_index(
    mut block_size: core::ffi::c_int,
) -> core::ffi::c_int {
    match block_size {
        4 => return 0 as core::ffi::c_int,
        8 => return 1 as core::ffi::c_int,
        16 => return 2 as core::ffi::c_int,
        32 => return 3 as core::ffi::c_int,
        64 => return 4 as core::ffi::c_int,
        128 => return 5 as core::ffi::c_int,
        _ => return -(1 as core::ffi::c_int),
    };
}
unsafe extern "C-unwind" fn get_identity_hash_value(
    a: uint8_t,
    b: uint8_t,
    c: uint8_t,
    d: uint8_t,
) -> uint32_t {
    return ((a as uint32_t) << 24 as core::ffi::c_int)
        .wrapping_add((b as uint32_t) << 16 as core::ffi::c_int)
        .wrapping_add((c as uint32_t) << 8 as core::ffi::c_int)
        .wrapping_add(d as uint32_t);
}
unsafe extern "C-unwind" fn get_xor_hash_value_hbd(
    a: uint16_t,
    b: uint16_t,
    c: uint16_t,
    d: uint16_t,
) -> uint32_t {
    let mut result: uint32_t = 0;
    result = (((a as core::ffi::c_int & 0xff as core::ffi::c_int) as uint32_t)
        << 24 as core::ffi::c_int)
        .wrapping_add(
            ((b as core::ffi::c_int & 0xff as core::ffi::c_int) as uint32_t)
                << 16 as core::ffi::c_int,
        )
        .wrapping_add(
            ((c as core::ffi::c_int & 0xff as core::ffi::c_int) as uint32_t)
                << 8 as core::ffi::c_int,
        )
        .wrapping_add((d as core::ffi::c_int & 0xff as core::ffi::c_int) as uint32_t);
    result
        ^= (((a as core::ffi::c_int & 0xff00 as core::ffi::c_int) as uint32_t)
            << 16 as core::ffi::c_int)
            .wrapping_add(
                ((b as core::ffi::c_int & 0xff00 as core::ffi::c_int) as uint32_t)
                    << 8 as core::ffi::c_int,
            )
            .wrapping_add(
                (c as core::ffi::c_int & 0xff00 as core::ffi::c_int) as uint32_t,
            )
            .wrapping_add(
                (d as core::ffi::c_int & 0xff00 as core::ffi::c_int) as uint32_t
                    >> 8 as core::ffi::c_int,
            );
    return result;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_table_init(
    mut intrabc_hash_info: *mut IntraBCHashInfo,
) {
    if (*intrabc_hash_info).crc_initialized == 0 {
        av1_crc32c_calculator_init(&raw mut (*intrabc_hash_info).crc_calculator);
        (*intrabc_hash_info).crc_initialized = 1 as core::ffi::c_int;
    }
    (*intrabc_hash_info).intrabc_hash_table.p_lookup_table = 0 as *mut *mut Vector;
}
unsafe extern "C-unwind" fn clear_all(mut p_hash_table: *mut hash_table) {
    if ((*p_hash_table).p_lookup_table).is_null() {
        return;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < kMaxAddr {
        if !(*((*p_hash_table).p_lookup_table).offset(i as isize)).is_null() {
            aom_vector_destroy(*((*p_hash_table).p_lookup_table).offset(i as isize));
            aom_free(
                *((*p_hash_table).p_lookup_table).offset(i as isize)
                    as *mut core::ffi::c_void,
            );
            let fresh0 = &raw mut *((*p_hash_table).p_lookup_table).offset(i as isize);
            *fresh0 = 0 as *mut Vector;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_table_destroy(mut p_hash_table: *mut hash_table) {
    clear_all(p_hash_table);
    aom_free((*p_hash_table).p_lookup_table as *mut core::ffi::c_void);
    (*p_hash_table).p_lookup_table = 0 as *mut *mut Vector;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_table_create(
    mut p_hash_table: *mut hash_table,
) -> bool {
    if !((*p_hash_table).p_lookup_table).is_null() {
        clear_all(p_hash_table);
        return true_0 != 0;
    }
    (*p_hash_table).p_lookup_table = aom_calloc(
        kMaxAddr as size_t,
        ::core::mem::size_of::<*mut Vector>() as size_t,
    ) as *mut *mut Vector;
    if ((*p_hash_table).p_lookup_table).is_null() {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C-unwind" fn hash_table_add_to_table(
    mut p_hash_table: *mut hash_table,
    mut hash_value: uint32_t,
    mut curr_block_hash: *const block_hash,
) -> bool {
    if (*((*p_hash_table).p_lookup_table).offset(hash_value as isize)).is_null() {
        let fresh3 = &raw mut *((*p_hash_table).p_lookup_table)
            .offset(hash_value as isize);
        *fresh3 = aom_malloc(::core::mem::size_of::<Vector>() as size_t) as *mut Vector;
        if (*((*p_hash_table).p_lookup_table).offset(hash_value as isize)).is_null() {
            return false_0 != 0;
        }
        if aom_vector_setup(
            *((*p_hash_table).p_lookup_table).offset(hash_value as isize),
            10 as size_t,
            ::core::mem::size_of::<block_hash>() as size_t,
        ) == VECTOR_ERROR
        {
            return false_0 != 0;
        }
    }
    if aom_vector_byte_size(
        *((*p_hash_table).p_lookup_table).offset(hash_value as isize),
    )
        < (kMaxCandidatesPerHashBucket as usize)
            .wrapping_mul(::core::mem::size_of::<block_hash>() as usize)
    {
        if aom_vector_push_back(
            *((*p_hash_table).p_lookup_table).offset(hash_value as isize),
            curr_block_hash as *mut core::ffi::c_void,
        ) == VECTOR_ERROR
        {
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_table_count(
    mut p_hash_table: *const hash_table,
    mut hash_value: uint32_t,
) -> int32_t {
    if (*((*p_hash_table).p_lookup_table).offset(hash_value as isize)).is_null() {
        return 0 as int32_t
    } else {
        return (**((*p_hash_table).p_lookup_table).offset(hash_value as isize)).size
            as int32_t
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_get_first_iterator(
    mut p_hash_table: *mut hash_table,
    mut hash_value: uint32_t,
) -> Iterator_0 {
    return aom_vector_begin(
        *((*p_hash_table).p_lookup_table).offset(hash_value as isize),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_generate_block_2x2_hash_value(
    mut picture: *const YV12_BUFFER_CONFIG,
    mut pic_block_hash: *mut uint32_t,
) {
    let width: core::ffi::c_int = 2 as core::ffi::c_int;
    let height: core::ffi::c_int = 2 as core::ffi::c_int;
    let x_end: core::ffi::c_int = (*picture).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        - width + 1 as core::ffi::c_int;
    let y_end: core::ffi::c_int = (*picture)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .y_crop_height - height + 1 as core::ffi::c_int;
    if (*picture).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut p: [uint16_t; 4] = [0; 4];
        let mut pos: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut y_pos: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_pos < y_end {
            let mut x_pos: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_pos < x_end {
                get_pixels_in_1D_short_array_by_block_2x2(
                    ((((*picture).c2rust_unnamed_4.c2rust_unnamed.y_buffer as uintptr_t)
                        << 1 as core::ffi::c_int) as *mut uint16_t)
                        .offset(
                            (y_pos * (*picture).c2rust_unnamed_3.c2rust_unnamed.y_stride)
                                as isize,
                        )
                        .offset(x_pos as isize),
                    (*picture).c2rust_unnamed_3.c2rust_unnamed.y_stride,
                    p.as_mut_ptr(),
                );
                *pic_block_hash.offset(pos as isize) = get_xor_hash_value_hbd(
                    p[0 as core::ffi::c_int as usize],
                    p[1 as core::ffi::c_int as usize],
                    p[2 as core::ffi::c_int as usize],
                    p[3 as core::ffi::c_int as usize],
                );
                pos += 1;
                x_pos += 1;
            }
            pos += width - 1 as core::ffi::c_int;
            y_pos += 1;
        }
    } else {
        let mut p_0: [uint8_t; 4] = [0; 4];
        let mut pos_0: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut y_pos_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_pos_0 < y_end {
            let mut x_pos_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_pos_0 < x_end {
                get_pixels_in_1D_char_array_by_block_2x2(
                    ((*picture).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
                        .offset(
                            (y_pos_0
                                * (*picture).c2rust_unnamed_3.c2rust_unnamed.y_stride)
                                as isize,
                        )
                        .offset(x_pos_0 as isize),
                    (*picture).c2rust_unnamed_3.c2rust_unnamed.y_stride,
                    p_0.as_mut_ptr(),
                );
                *pic_block_hash.offset(pos_0 as isize) = get_identity_hash_value(
                    p_0[0 as core::ffi::c_int as usize],
                    p_0[1 as core::ffi::c_int as usize],
                    p_0[2 as core::ffi::c_int as usize],
                    p_0[3 as core::ffi::c_int as usize],
                );
                pos_0 += 1;
                x_pos_0 += 1;
            }
            pos_0 += width - 1 as core::ffi::c_int;
            y_pos_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_generate_block_hash_value(
    mut intrabc_hash_info: *mut IntraBCHashInfo,
    mut picture: *const YV12_BUFFER_CONFIG,
    mut block_size: core::ffi::c_int,
    mut src_pic_block_hash: *const uint32_t,
    mut dst_pic_block_hash: *mut uint32_t,
) {
    let mut calc: *mut CRC32C = &raw mut (*intrabc_hash_info).crc_calculator;
    let pic_width: core::ffi::c_int = (*picture)
        .c2rust_unnamed_1
        .c2rust_unnamed
        .y_crop_width;
    let x_end: core::ffi::c_int = (*picture).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        - block_size + 1 as core::ffi::c_int;
    let y_end: core::ffi::c_int = (*picture)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .y_crop_height - block_size + 1 as core::ffi::c_int;
    let src_size: core::ffi::c_int = block_size >> 1 as core::ffi::c_int;
    let mut p: [uint32_t; 4] = [0; 4];
    let length: core::ffi::c_int = ::core::mem::size_of::<[uint32_t; 4]>()
        as core::ffi::c_int;
    let mut pos: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y_pos: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_pos < y_end {
        let mut x_pos: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_pos < x_end {
            p[0 as core::ffi::c_int as usize] = *src_pic_block_hash.offset(pos as isize);
            p[1 as core::ffi::c_int as usize] = *src_pic_block_hash
                .offset((pos + src_size) as isize);
            p[2 as core::ffi::c_int as usize] = *src_pic_block_hash
                .offset((pos + src_size * pic_width) as isize);
            p[3 as core::ffi::c_int as usize] = *src_pic_block_hash
                .offset((pos + src_size * pic_width + src_size) as isize);
            *dst_pic_block_hash.offset(pos as isize) = av1_get_crc32c_value_c(
                calc as *mut core::ffi::c_void,
                p.as_mut_ptr() as *mut uint8_t,
                length as size_t,
            );
            pos += 1;
            x_pos += 1;
        }
        pos += block_size - 1 as core::ffi::c_int;
        y_pos += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_add_to_hash_map_by_row_with_precal_data(
    mut p_hash_table: *mut hash_table,
    mut pic_hash: *const uint32_t,
    mut pic_width: core::ffi::c_int,
    mut pic_height: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
) -> bool {
    let x_end: core::ffi::c_int = pic_width - block_size + 1 as core::ffi::c_int;
    let y_end: core::ffi::c_int = pic_height - block_size + 1 as core::ffi::c_int;
    let mut add_value: core::ffi::c_int = hash_block_size_to_index(block_size);
    add_value <<= kSrcBits;
    let crc_mask: core::ffi::c_int = ((1 as core::ffi::c_int) << kSrcBits)
        - 1 as core::ffi::c_int;
    let mut step: core::ffi::c_int = block_size;
    let mut x_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y_offset: core::ffi::c_int = 0 as core::ffi::c_int;
    while step > 1 as core::ffi::c_int {
        let mut x_pos: core::ffi::c_int = x_offset;
        while x_pos < x_end {
            let mut y_pos: core::ffi::c_int = y_offset;
            while y_pos < y_end {
                let pos: core::ffi::c_int = y_pos * pic_width + x_pos;
                let mut curr_block_hash: block_hash = _block_hash {
                    x: 0,
                    y: 0,
                    hash_value2: 0,
                };
                curr_block_hash.x = x_pos as int16_t;
                curr_block_hash.y = y_pos as int16_t;
                let hash_value1: uint32_t = (*pic_hash.offset(pos as isize)
                    & crc_mask as uint32_t)
                    .wrapping_add(add_value as uint32_t);
                curr_block_hash.hash_value2 = *pic_hash.offset(pos as isize);
                if !hash_table_add_to_table(
                    p_hash_table,
                    hash_value1,
                    &raw mut curr_block_hash,
                ) {
                    return false_0 != 0;
                }
                y_pos += step;
            }
            x_pos += step;
        }
        if x_offset == 0 as core::ffi::c_int && y_offset == 0 as core::ffi::c_int {
            x_offset = step / 2 as core::ffi::c_int;
        } else if x_offset == step / 2 as core::ffi::c_int
            && y_offset == 0 as core::ffi::c_int
        {
            x_offset = 0 as core::ffi::c_int;
            y_offset = step / 2 as core::ffi::c_int;
        } else if x_offset == 0 as core::ffi::c_int
            && y_offset == step / 2 as core::ffi::c_int
        {
            x_offset = step / 2 as core::ffi::c_int;
        } else {
            step /= 2 as core::ffi::c_int;
            x_offset = step / 2 as core::ffi::c_int;
            y_offset = 0 as core::ffi::c_int;
        }
    }
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_is_horizontal_perfect(
    mut picture: *const YV12_BUFFER_CONFIG,
    mut block_size: core::ffi::c_int,
    mut x_start: core::ffi::c_int,
    mut y_start: core::ffi::c_int,
) -> core::ffi::c_int {
    let stride: core::ffi::c_int = (*picture).c2rust_unnamed_3.c2rust_unnamed.y_stride;
    let mut p: *const uint8_t = ((*picture).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
        .offset((y_start * stride) as isize)
        .offset(x_start as isize);
    if (*picture).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut p16: *const uint16_t = ((p as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < block_size {
            let mut j: core::ffi::c_int = 1 as core::ffi::c_int;
            while j < block_size {
                if *p16.offset(j as isize) as core::ffi::c_int
                    != *p16.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                {
                    return 0 as core::ffi::c_int;
                }
                j += 1;
            }
            p16 = p16.offset(stride as isize);
            i += 1;
        }
    } else {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < block_size {
            let mut j_0: core::ffi::c_int = 1 as core::ffi::c_int;
            while j_0 < block_size {
                if *p.offset(j_0 as isize) as core::ffi::c_int
                    != *p.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                {
                    return 0 as core::ffi::c_int;
                }
                j_0 += 1;
            }
            p = p.offset(stride as isize);
            i_0 += 1;
        }
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_hash_is_vertical_perfect(
    mut picture: *const YV12_BUFFER_CONFIG,
    mut block_size: core::ffi::c_int,
    mut x_start: core::ffi::c_int,
    mut y_start: core::ffi::c_int,
) -> core::ffi::c_int {
    let stride: core::ffi::c_int = (*picture).c2rust_unnamed_3.c2rust_unnamed.y_stride;
    let mut p: *const uint8_t = ((*picture).c2rust_unnamed_4.c2rust_unnamed.y_buffer)
        .offset((y_start * stride) as isize)
        .offset(x_start as isize);
    if (*picture).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        let mut p16: *const uint16_t = ((p as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < block_size {
            let mut j: core::ffi::c_int = 1 as core::ffi::c_int;
            while j < block_size {
                if *p16.offset((j * stride + i) as isize) as core::ffi::c_int
                    != *p16.offset(i as isize) as core::ffi::c_int
                {
                    return 0 as core::ffi::c_int;
                }
                j += 1;
            }
            i += 1;
        }
    } else {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < block_size {
            let mut j_0: core::ffi::c_int = 1 as core::ffi::c_int;
            while j_0 < block_size {
                if *p.offset((j_0 * stride + i_0) as isize) as core::ffi::c_int
                    != *p.offset(i_0 as isize) as core::ffi::c_int
                {
                    return 0 as core::ffi::c_int;
                }
                j_0 += 1;
            }
            i_0 += 1;
        }
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_block_hash_value(
    mut intra_bc_hash_info: *mut IntraBCHashInfo,
    mut y_src: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut block_size: core::ffi::c_int,
    mut hash_value1: *mut uint32_t,
    mut hash_value2: *mut uint32_t,
    mut use_highbitdepth: core::ffi::c_int,
) {
    let mut add_value: core::ffi::c_int = hash_block_size_to_index(block_size);
    add_value <<= kSrcBits;
    let crc_mask: core::ffi::c_int = ((1 as core::ffi::c_int) << kSrcBits)
        - 1 as core::ffi::c_int;
    let mut calc: *mut CRC32C = &raw mut (*intra_bc_hash_info).crc_calculator;
    let mut buf: *mut *mut uint32_t = ((*intra_bc_hash_info).hash_value_buffer)
        .as_mut_ptr();
    let mut sub_block_in_width: core::ffi::c_int = block_size >> 1 as core::ffi::c_int;
    if use_highbitdepth != 0 {
        let mut pixel_to_hash: [uint16_t; 4] = [0; 4];
        let mut y16_src: *mut uint16_t = ((y_src as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        let mut y_pos: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_pos < block_size {
            let mut x_pos: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_pos < block_size {
                let mut pos: core::ffi::c_int = (y_pos >> 1 as core::ffi::c_int)
                    * sub_block_in_width + (x_pos >> 1 as core::ffi::c_int);
                get_pixels_in_1D_short_array_by_block_2x2(
                    y16_src.offset((y_pos * stride) as isize).offset(x_pos as isize),
                    stride,
                    pixel_to_hash.as_mut_ptr(),
                );
                *(*buf.offset(0 as core::ffi::c_int as isize)).offset(pos as isize) = get_xor_hash_value_hbd(
                    pixel_to_hash[0 as core::ffi::c_int as usize],
                    pixel_to_hash[1 as core::ffi::c_int as usize],
                    pixel_to_hash[2 as core::ffi::c_int as usize],
                    pixel_to_hash[3 as core::ffi::c_int as usize],
                );
                x_pos += 2 as core::ffi::c_int;
            }
            y_pos += 2 as core::ffi::c_int;
        }
    } else {
        let mut pixel_to_hash_0: [uint8_t; 4] = [0; 4];
        let mut y_pos_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_pos_0 < block_size {
            let mut x_pos_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_pos_0 < block_size {
                let mut pos_0: core::ffi::c_int = (y_pos_0 >> 1 as core::ffi::c_int)
                    * sub_block_in_width + (x_pos_0 >> 1 as core::ffi::c_int);
                get_pixels_in_1D_char_array_by_block_2x2(
                    y_src.offset((y_pos_0 * stride) as isize).offset(x_pos_0 as isize),
                    stride,
                    pixel_to_hash_0.as_mut_ptr(),
                );
                *(*buf.offset(0 as core::ffi::c_int as isize)).offset(pos_0 as isize) = get_identity_hash_value(
                    pixel_to_hash_0[0 as core::ffi::c_int as usize],
                    pixel_to_hash_0[1 as core::ffi::c_int as usize],
                    pixel_to_hash_0[2 as core::ffi::c_int as usize],
                    pixel_to_hash_0[3 as core::ffi::c_int as usize],
                );
                x_pos_0 += 2 as core::ffi::c_int;
            }
            y_pos_0 += 2 as core::ffi::c_int;
        }
    }
    let mut src_sub_block_in_width: core::ffi::c_int = sub_block_in_width;
    sub_block_in_width >>= 1 as core::ffi::c_int;
    let mut src_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dst_idx: core::ffi::c_int = (src_idx == 0) as core::ffi::c_int;
    let mut to_hash: [uint32_t; 4] = [0; 4];
    let mut sub_width: core::ffi::c_int = 4 as core::ffi::c_int;
    while sub_width <= block_size {
        dst_idx = (src_idx == 0) as core::ffi::c_int;
        let mut dst_pos: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut y_pos_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_pos_1 < sub_block_in_width {
            let mut x_pos_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_pos_1 < sub_block_in_width {
                let mut srcPos: core::ffi::c_int = (y_pos_1 << 1 as core::ffi::c_int)
                    * src_sub_block_in_width + (x_pos_1 << 1 as core::ffi::c_int);
                to_hash[0 as core::ffi::c_int as usize] = *(*buf
                    .offset(src_idx as isize))
                    .offset(srcPos as isize);
                to_hash[1 as core::ffi::c_int as usize] = *(*buf
                    .offset(src_idx as isize))
                    .offset((srcPos + 1 as core::ffi::c_int) as isize);
                to_hash[2 as core::ffi::c_int as usize] = *(*buf
                    .offset(src_idx as isize))
                    .offset((srcPos + src_sub_block_in_width) as isize);
                to_hash[3 as core::ffi::c_int as usize] = *(*buf
                    .offset(src_idx as isize))
                    .offset(
                        (srcPos + src_sub_block_in_width + 1 as core::ffi::c_int)
                            as isize,
                    );
                *(*buf.offset(dst_idx as isize)).offset(dst_pos as isize) = av1_get_crc32c_value_c(
                    calc as *mut core::ffi::c_void,
                    to_hash.as_mut_ptr() as *mut uint8_t,
                    ::core::mem::size_of::<[uint32_t; 4]>() as size_t,
                );
                dst_pos += 1;
                x_pos_1 += 1;
            }
            y_pos_1 += 1;
        }
        src_sub_block_in_width = sub_block_in_width;
        sub_block_in_width >>= 1 as core::ffi::c_int;
        sub_width *= 2 as core::ffi::c_int;
        src_idx = (src_idx == 0) as core::ffi::c_int;
    }
    *hash_value1 = (*(*buf.offset(dst_idx as isize))
        .offset(0 as core::ffi::c_int as isize) & crc_mask as uint32_t)
        .wrapping_add(add_value as uint32_t);
    *hash_value2 = *(*buf.offset(dst_idx as isize))
        .offset(0 as core::ffi::c_int as isize);
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
