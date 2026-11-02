/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/vmaf.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type VmafFeatureDictionary;
    pub type VmafModel;
    pub type VmafRef;
    pub type VmafContext;
    pub type image_pyramid;
    pub type corner_list;
    static mut stderr: *mut FILE;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
pub type VmafModelFlags = core::ffi::c_uint;
pub const VMAF_MODEL_FLAG_DISABLE_TRANSFORM: VmafModelFlags = 4;
pub const VMAF_MODEL_FLAG_ENABLE_TRANSFORM: VmafModelFlags = 2;
pub const VMAF_MODEL_FLAG_DISABLE_CLIP: VmafModelFlags = 1;
pub const VMAF_MODEL_FLAGS_DEFAULT: VmafModelFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VmafModelConfig {
    pub name: *const core::ffi::c_char,
    pub flags: uint64_t,
}
pub type ptrdiff_t = isize;
pub type VmafPixelFormat = core::ffi::c_uint;
pub const VMAF_PIX_FMT_YUV400P: VmafPixelFormat = 4;
pub const VMAF_PIX_FMT_YUV444P: VmafPixelFormat = 3;
pub const VMAF_PIX_FMT_YUV422P: VmafPixelFormat = 2;
pub const VMAF_PIX_FMT_YUV420P: VmafPixelFormat = 1;
pub const VMAF_PIX_FMT_UNKNOWN: VmafPixelFormat = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VmafPicture {
    pub pix_fmt: VmafPixelFormat,
    pub bpc: core::ffi::c_uint,
    pub w: [core::ffi::c_uint; 3],
    pub h: [core::ffi::c_uint; 3],
    pub stride: [ptrdiff_t; 3],
    pub data: [*mut core::ffi::c_void; 3],
    pub ref_0: *mut VmafRef,
    pub priv_0: *mut core::ffi::c_void,
}
pub type VmafLogLevel = core::ffi::c_uint;
pub const VMAF_LOG_LEVEL_DEBUG: VmafLogLevel = 4;
pub const VMAF_LOG_LEVEL_INFO: VmafLogLevel = 3;
pub const VMAF_LOG_LEVEL_WARNING: VmafLogLevel = 2;
pub const VMAF_LOG_LEVEL_ERROR: VmafLogLevel = 1;
pub const VMAF_LOG_LEVEL_NONE: VmafLogLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VmafConfiguration {
    pub log_level: VmafLogLevel,
    pub n_threads: core::ffi::c_uint,
    pub n_subsample: core::ffi::c_uint,
    pub cpumask: uint64_t,
    pub gpumask: uint64_t,
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
unsafe extern "C-unwind" fn vmaf_fatal_error(mut message: *const core::ffi::c_char) {
    fprintf(
        stderr,
        b"Fatal error: %s\n\0" as *const u8 as *const core::ffi::c_char,
        message,
    );
    exit(EXIT_FAILURE);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_init_vmaf_model(
    mut vmaf_model: *mut *mut VmafModel,
    mut model_path: *const core::ffi::c_char,
) {
    if !(*vmaf_model).is_null() {
        return;
    }
    let mut model_cfg: VmafModelConfig = VmafModelConfig {
        name: 0 as *const core::ffi::c_char,
        flags: 0,
    };
    model_cfg.flags = VMAF_MODEL_FLAG_DISABLE_CLIP as core::ffi::c_int as uint64_t;
    model_cfg.name = b"vmaf\0" as *const u8 as *const core::ffi::c_char;
    if vmaf_model_load_from_path(vmaf_model, &raw mut model_cfg, model_path) != 0 {
        vmaf_fatal_error(
            b"Failed to load VMAF model.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_close_vmaf_model(mut vmaf_model: *mut VmafModel) {
    vmaf_model_destroy(vmaf_model);
}
unsafe extern "C-unwind" fn copy_picture(
    bit_depth: core::ffi::c_int,
    mut src: *const YV12_BUFFER_CONFIG,
    mut dst: *mut VmafPicture,
) {
    let width: core::ffi::c_int = (*src).c2rust_unnamed.c2rust_unnamed.y_width;
    let height: core::ffi::c_int = (*src).c2rust_unnamed_0.c2rust_unnamed.y_height;
    if bit_depth > 8 as core::ffi::c_int {
        let mut src_ptr: *mut uint16_t = (((*src)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t;
        let mut dst_ptr: *mut uint16_t = (*dst).data[0 as core::ffi::c_int as usize]
            as *mut uint16_t;
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < height {
            memcpy(
                dst_ptr as *mut core::ffi::c_void,
                src_ptr as *const core::ffi::c_void,
                (width as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            src_ptr = src_ptr
                .offset((*src).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
            dst_ptr = dst_ptr
                .offset(
                    ((*dst).stride[0 as core::ffi::c_int as usize] / 2 as ptrdiff_t)
                        as isize,
                );
            row += 1;
        }
    } else {
        let mut src_ptr_0: *mut uint8_t = (*src)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer;
        let mut dst_ptr_0: *mut uint8_t = (*dst).data[0 as core::ffi::c_int as usize]
            as *mut uint8_t;
        let mut row_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while row_0 < height {
            memcpy(
                dst_ptr_0 as *mut core::ffi::c_void,
                src_ptr_0 as *const core::ffi::c_void,
                (width as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            );
            src_ptr_0 = src_ptr_0
                .offset((*src).c2rust_unnamed_3.c2rust_unnamed.y_stride as isize);
            dst_ptr_0 = dst_ptr_0
                .offset((*dst).stride[0 as core::ffi::c_int as usize] as isize);
            row_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_init_vmaf_context(
    mut vmaf_context: *mut *mut VmafContext,
    mut vmaf_model: *mut VmafModel,
    mut cal_vmaf_neg: bool,
) {
    let mut cfg: VmafConfiguration = VmafConfiguration {
        log_level: VMAF_LOG_LEVEL_NONE,
        n_threads: 0,
        n_subsample: 0,
        cpumask: 0,
        gpumask: 0,
    };
    cfg.log_level = VMAF_LOG_LEVEL_NONE;
    cfg.n_threads = 0 as core::ffi::c_uint;
    cfg.n_subsample = 0 as core::ffi::c_uint;
    cfg.cpumask = 0 as uint64_t;
    if vmaf_init(vmaf_context, cfg) != 0 {
        vmaf_fatal_error(
            b"Failed to init VMAF context.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if cal_vmaf_neg {
        let mut vif_feature: *mut VmafFeatureDictionary = 0
            as *mut VmafFeatureDictionary;
        if vmaf_feature_dictionary_set(
            &raw mut vif_feature,
            b"vif_enhn_gain_limit\0" as *const u8 as *const core::ffi::c_char,
            b"1.0\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            vmaf_fatal_error(
                b"Failed to set vif_enhn_gain_limit.\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if vmaf_model_feature_overload(
            vmaf_model,
            b"float_vif\0" as *const u8 as *const core::ffi::c_char,
            vif_feature,
        ) != 0
        {
            vmaf_fatal_error(
                b"Failed to use feature float_vif.\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut adm_feature: *mut VmafFeatureDictionary = 0
            as *mut VmafFeatureDictionary;
        if vmaf_feature_dictionary_set(
            &raw mut adm_feature,
            b"adm_enhn_gain_limit\0" as *const u8 as *const core::ffi::c_char,
            b"1.0\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            vmaf_fatal_error(
                b"Failed to set adm_enhn_gain_limit.\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if vmaf_model_feature_overload(
            vmaf_model,
            b"adm\0" as *const u8 as *const core::ffi::c_char,
            adm_feature,
        ) != 0
        {
            vmaf_fatal_error(
                b"Failed to use feature float_adm.\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    let mut motion_force_zero: *mut VmafFeatureDictionary = 0
        as *mut VmafFeatureDictionary;
    if vmaf_feature_dictionary_set(
        &raw mut motion_force_zero,
        b"motion_force_zero\0" as *const u8 as *const core::ffi::c_char,
        b"1\0" as *const u8 as *const core::ffi::c_char,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to set motion_force_zero.\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if vmaf_model_feature_overload(
        vmaf_model,
        b"float_motion\0" as *const u8 as *const core::ffi::c_char,
        motion_force_zero,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to use feature float_motion.\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if vmaf_use_features_from_model(*vmaf_context, vmaf_model) != 0 {
        vmaf_fatal_error(
            b"Failed to load feature extractors from VMAF model.\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_close_vmaf_context(mut vmaf_context: *mut VmafContext) {
    if vmaf_close(vmaf_context) != 0 {
        vmaf_fatal_error(
            b"Failed to close VMAF context.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_calc_vmaf(
    mut vmaf_model: *mut VmafModel,
    mut source: *const YV12_BUFFER_CONFIG,
    mut distorted: *const YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut cal_vmaf_neg: bool,
    mut vmaf: *mut core::ffi::c_double,
) {
    let mut vmaf_context: *mut VmafContext = 0 as *mut VmafContext;
    aom_init_vmaf_context(&raw mut vmaf_context, vmaf_model, cal_vmaf_neg);
    let frame_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ref_0: VmafPicture = VmafPicture {
        pix_fmt: VMAF_PIX_FMT_UNKNOWN,
        bpc: 0,
        w: [0; 3],
        h: [0; 3],
        stride: [0; 3],
        data: [0 as *mut core::ffi::c_void; 3],
        ref_0: 0 as *mut VmafRef,
        priv_0: 0 as *mut core::ffi::c_void,
    };
    let mut dist: VmafPicture = VmafPicture {
        pix_fmt: VMAF_PIX_FMT_UNKNOWN,
        bpc: 0,
        w: [0; 3],
        h: [0; 3],
        stride: [0; 3],
        data: [0 as *mut core::ffi::c_void; 3],
        ref_0: 0 as *mut VmafRef,
        priv_0: 0 as *mut core::ffi::c_void,
    };
    if vmaf_picture_alloc(
        &raw mut ref_0,
        VMAF_PIX_FMT_YUV420P,
        bit_depth as core::ffi::c_uint,
        (*source).c2rust_unnamed.c2rust_unnamed.y_width as core::ffi::c_uint,
        (*source).c2rust_unnamed_0.c2rust_unnamed.y_height as core::ffi::c_uint,
    ) != 0
        || vmaf_picture_alloc(
            &raw mut dist,
            VMAF_PIX_FMT_YUV420P,
            bit_depth as core::ffi::c_uint,
            (*source).c2rust_unnamed.c2rust_unnamed.y_width as core::ffi::c_uint,
            (*source).c2rust_unnamed_0.c2rust_unnamed.y_height as core::ffi::c_uint,
        ) != 0
    {
        vmaf_fatal_error(
            b"Failed to alloc VMAF pictures.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    copy_picture(bit_depth, source, &raw mut ref_0);
    copy_picture(bit_depth, distorted, &raw mut dist);
    if vmaf_read_pictures(
        vmaf_context,
        &raw mut ref_0,
        &raw mut dist,
        frame_index as core::ffi::c_uint,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to read VMAF pictures.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if vmaf_read_pictures(
        vmaf_context,
        0 as *mut VmafPicture,
        0 as *mut VmafPicture,
        0 as core::ffi::c_uint,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to flush context.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    vmaf_picture_unref(&raw mut ref_0);
    vmaf_picture_unref(&raw mut dist);
    vmaf_score_at_index(
        vmaf_context,
        vmaf_model,
        vmaf,
        frame_index as core::ffi::c_uint,
    );
    aom_close_vmaf_context(vmaf_context);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_read_vmaf_image(
    mut vmaf_context: *mut VmafContext,
    mut source: *const YV12_BUFFER_CONFIG,
    mut distorted: *const YV12_BUFFER_CONFIG,
    mut bit_depth: core::ffi::c_int,
    mut frame_index: core::ffi::c_int,
) {
    let mut ref_0: VmafPicture = VmafPicture {
        pix_fmt: VMAF_PIX_FMT_UNKNOWN,
        bpc: 0,
        w: [0; 3],
        h: [0; 3],
        stride: [0; 3],
        data: [0 as *mut core::ffi::c_void; 3],
        ref_0: 0 as *mut VmafRef,
        priv_0: 0 as *mut core::ffi::c_void,
    };
    let mut dist: VmafPicture = VmafPicture {
        pix_fmt: VMAF_PIX_FMT_UNKNOWN,
        bpc: 0,
        w: [0; 3],
        h: [0; 3],
        stride: [0; 3],
        data: [0 as *mut core::ffi::c_void; 3],
        ref_0: 0 as *mut VmafRef,
        priv_0: 0 as *mut core::ffi::c_void,
    };
    if vmaf_picture_alloc(
        &raw mut ref_0,
        VMAF_PIX_FMT_YUV420P,
        bit_depth as core::ffi::c_uint,
        (*source).c2rust_unnamed.c2rust_unnamed.y_width as core::ffi::c_uint,
        (*source).c2rust_unnamed_0.c2rust_unnamed.y_height as core::ffi::c_uint,
    ) != 0
        || vmaf_picture_alloc(
            &raw mut dist,
            VMAF_PIX_FMT_YUV420P,
            bit_depth as core::ffi::c_uint,
            (*source).c2rust_unnamed.c2rust_unnamed.y_width as core::ffi::c_uint,
            (*source).c2rust_unnamed_0.c2rust_unnamed.y_height as core::ffi::c_uint,
        ) != 0
    {
        vmaf_fatal_error(
            b"Failed to alloc VMAF pictures.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    copy_picture(bit_depth, source, &raw mut ref_0);
    copy_picture(bit_depth, distorted, &raw mut dist);
    if vmaf_read_pictures(
        vmaf_context,
        &raw mut ref_0,
        &raw mut dist,
        frame_index as core::ffi::c_uint,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to read VMAF pictures.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    vmaf_picture_unref(&raw mut ref_0);
    vmaf_picture_unref(&raw mut dist);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_calc_vmaf_at_index(
    mut vmaf_context: *mut VmafContext,
    mut vmaf_model: *mut VmafModel,
    mut frame_index: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut vmaf: core::ffi::c_double = 0.;
    if vmaf_score_at_index(
        vmaf_context,
        vmaf_model,
        &raw mut vmaf,
        frame_index as core::ffi::c_uint,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to calc VMAF scores.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    return vmaf;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_flush_vmaf_context(mut vmaf_context: *mut VmafContext) {
    if vmaf_read_pictures(
        vmaf_context,
        0 as *mut VmafPicture,
        0 as *mut VmafPicture,
        0 as core::ffi::c_uint,
    ) != 0
    {
        vmaf_fatal_error(
            b"Failed to flush context.\0" as *const u8 as *const core::ffi::c_char,
        );
    }
}
pub const EXIT_FAILURE: core::ffi::c_int = 1 as core::ffi::c_int;

extern "C" {
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn vmaf_feature_dictionary_set(
        dict: *mut *mut VmafFeatureDictionary,
        key: *const core::ffi::c_char,
        val: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn vmaf_model_load_from_path(
        model: *mut *mut VmafModel,
        cfg: *mut VmafModelConfig,
        path: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn vmaf_model_feature_overload(
        model: *mut VmafModel,
        feature_name: *const core::ffi::c_char,
        opts_dict: *mut VmafFeatureDictionary,
    ) -> core::ffi::c_int;
    fn vmaf_model_destroy(model: *mut VmafModel);
    fn vmaf_picture_alloc(
        pic: *mut VmafPicture,
        pix_fmt: VmafPixelFormat,
        bpc: core::ffi::c_uint,
        w: core::ffi::c_uint,
        h: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn vmaf_picture_unref(pic: *mut VmafPicture) -> core::ffi::c_int;
    fn vmaf_init(
        vmaf: *mut *mut VmafContext,
        cfg: VmafConfiguration,
    ) -> core::ffi::c_int;
    fn vmaf_use_features_from_model(
        vmaf: *mut VmafContext,
        model: *mut VmafModel,
    ) -> core::ffi::c_int;
    fn vmaf_read_pictures(
        vmaf: *mut VmafContext,
        ref_0: *mut VmafPicture,
        dist: *mut VmafPicture,
        index: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn vmaf_score_at_index(
        vmaf: *mut VmafContext,
        model: *mut VmafModel,
        score: *mut core::ffi::c_double,
        index: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn vmaf_close(vmaf: *mut VmafContext) -> core::ffi::c_int;
    fn exit(__status: core::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
