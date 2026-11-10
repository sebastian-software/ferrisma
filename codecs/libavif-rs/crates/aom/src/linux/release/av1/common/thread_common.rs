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

// Mechanically translated from upstream/aom/av1/common/thread_common.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    fn aom_extend_frame_borders_plane_row_c(
        ybf: *const yv12_buffer_config,
        plane: core::ffi::c_int,
        v_start: core::ffi::c_int,
        v_end: core::ffi::c_int,
    );
    fn aom_yv12_partial_coloc_copy_u_c(
        src_bc: *const yv12_buffer_config,
        dst_bc: *mut yv12_buffer_config,
        hstart: core::ffi::c_int,
        hend: core::ffi::c_int,
        vstart: core::ffi::c_int,
        vend: core::ffi::c_int,
    );
    fn aom_yv12_partial_coloc_copy_v_c(
        src_bc: *const yv12_buffer_config,
        dst_bc: *mut yv12_buffer_config,
        hstart: core::ffi::c_int,
        hend: core::ffi::c_int,
        vstart: core::ffi::c_int,
        vend: core::ffi::c_int,
    );
    fn aom_yv12_partial_coloc_copy_y_c(
        src_ybc: *const yv12_buffer_config,
        dst_ybc: *mut yv12_buffer_config,
        hstart: core::ffi::c_int,
        hend: core::ffi::c_int,
        vstart: core::ffi::c_int,
        vend: core::ffi::c_int,
    );
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_get_worker_interface() -> *const AVxWorkerInterface;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_internal_error_copy(
        info: *mut aom_internal_error_info,
        src: *const aom_internal_error_info,
    );
    fn av1_loop_filter_frame_init(
        cm: *mut AV1Common,
        plane_start: core::ffi::c_int,
        plane_end: core::ffi::c_int,
    );
    fn av1_filter_block_plane_vert(
        cm: *const AV1Common,
        xd: *const MACROBLOCKD,
        plane: core::ffi::c_int,
        plane_ptr: *const MACROBLOCKD_PLANE,
        mi_row: uint32_t,
        mi_col: uint32_t,
    );
    fn av1_filter_block_plane_horz(
        cm: *const AV1Common,
        xd: *const MACROBLOCKD,
        plane: core::ffi::c_int,
        plane_ptr: *const MACROBLOCKD_PLANE,
        mi_row: uint32_t,
        mi_col: uint32_t,
    );
    fn av1_filter_block_plane_vert_opt(
        cm: *const AV1Common,
        xd: *const MACROBLOCKD,
        plane_ptr: *const MACROBLOCKD_PLANE,
        mi_row: uint32_t,
        mi_col: uint32_t,
        params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
        tx_buf: *mut TX_SIZE,
        num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
    );
    fn av1_filter_block_plane_vert_opt_chroma(
        cm: *const AV1Common,
        xd: *const MACROBLOCKD,
        plane_ptr: *const MACROBLOCKD_PLANE,
        mi_row: uint32_t,
        mi_col: uint32_t,
        params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
        tx_buf: *mut TX_SIZE,
        plane: core::ffi::c_int,
        joint_filter_chroma: bool,
        num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
    );
    fn av1_filter_block_plane_horz_opt(
        cm: *const AV1Common,
        xd: *const MACROBLOCKD,
        plane_ptr: *const MACROBLOCKD_PLANE,
        mi_row: uint32_t,
        mi_col: uint32_t,
        params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
        tx_buf: *mut TX_SIZE,
        num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
    );
    fn av1_filter_block_plane_horz_opt_chroma(
        cm: *const AV1Common,
        xd: *const MACROBLOCKD,
        plane_ptr: *const MACROBLOCKD_PLANE,
        mi_row: uint32_t,
        mi_col: uint32_t,
        params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
        tx_buf: *mut TX_SIZE,
        plane: core::ffi::c_int,
        joint_filter_chroma: bool,
        num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
    );
    fn av1_cdef_fb_row(
        cm: *const AV1_COMMON,
        xd: *mut MACROBLOCKD,
        linebuf: *mut *mut uint16_t,
        colbuf: *mut *mut uint16_t,
        src: *mut uint16_t,
        fbr: core::ffi::c_int,
        cdef_init_fb_row_fn: cdef_init_fb_row_t,
        cdef_sync: *mut AV1CdefSyncData,
        error_info: *mut aom_internal_error_info,
    );
    fn av1_loop_restoration_filter_frame_init(
        lr_ctxt: *mut AV1LrStruct,
        frame: *mut YV12_BUFFER_CONFIG,
        cm: *mut AV1Common,
        optimized_lr: core::ffi::c_int,
        num_planes: core::ffi::c_int,
    );
    fn av1_foreach_rest_unit_in_row(
        limits: *mut RestorationTileLimits,
        plane_w: core::ffi::c_int,
        on_rest_unit: rest_unit_visitor_t,
        row_number: core::ffi::c_int,
        unit_size: core::ffi::c_int,
        hnum_rest_units: core::ffi::c_int,
        vnum_rest_units: core::ffi::c_int,
        plane: core::ffi::c_int,
        priv_0: *mut core::ffi::c_void,
        tmpbuf: *mut int32_t,
        rlbs: *mut RestorationLineBuffers,
        on_sync_read: sync_read_fn_t,
        on_sync_write: sync_write_fn_t,
        lr_sync: *mut AV1LrSyncData,
        error_info: *mut aom_internal_error_info,
    );
    fn av1_lr_count_units(
        unit_size: core::ffi::c_int,
        plane_size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_lr_sync_read_dummy(
        lr_sync: *mut core::ffi::c_void,
        r: core::ffi::c_int,
        c: core::ffi::c_int,
        plane: core::ffi::c_int,
    );
    fn av1_lr_sync_write_dummy(
        lr_sync: *mut core::ffi::c_void,
        r: core::ffi::c_int,
        c: core::ffi::c_int,
        sb_cols: core::ffi::c_int,
        plane: core::ffi::c_int,
    );
    fn av1_setup_dst_planes(
        planes: *mut macroblockd_plane,
        bsize: BLOCK_SIZE,
        src: *const YV12_BUFFER_CONFIG,
        mi_row: core::ffi::c_int,
        mi_col: core::ffi::c_int,
        plane_start: core::ffi::c_int,
        plane_end: core::ffi::c_int,
    );
    fn av1_cdef_copy_sb8_16(
        cm: *const AV1_COMMON,
        dst: *mut uint16_t,
        dstride: core::ffi::c_int,
        src: *const uint8_t,
        src_voffset: core::ffi::c_int,
        src_hoffset: core::ffi::c_int,
        sstride: core::ffi::c_int,
        vsize: core::ffi::c_int,
        hsize: core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct corner_list {
    pub mutex: pthread_mutex_t,
    pub valid: bool,
    pub num_corners: core::ffi::c_int,
    pub corners: [core::ffi::c_int; 8192],
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
pub type qm_val_t = uint8_t;
pub type TX_SIZE = uint8_t;
pub type TX_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub __low: core::ffi::c_uint,
    pub __high: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [core::ffi::c_uint; 2],
    pub __g_size: [core::ffi::c_uint; 2],
    pub __g1_orig_size: core::ffi::c_uint,
    pub __wrefs: core::ffi::c_uint,
    pub __g_signals: [core::ffi::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [core::ffi::c_char; 4],
    pub __align: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [core::ffi::c_char; 4],
    pub __align: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [core::ffi::c_char; 48],
    pub __align: core::ffi::c_longlong,
}
pub type __jmp_buf = [core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorker {
    pub impl_: *mut AVxWorkerImpl,
    pub status_: AVxWorkerStatus,
    pub thread_name: *const core::ffi::c_char,
    pub hook: AVxWorkerHook,
    pub data1: *mut core::ffi::c_void,
    pub data2: *mut core::ffi::c_void,
    pub had_error: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorkerInterface {
    pub init: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub reset: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub sync: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub launch: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub execute: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub end: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
}
pub type aom_codec_err_t = core::ffi::c_uint;
pub const AOM_CODEC_LIST_END: aom_codec_err_t = 9;
pub const AOM_CODEC_INVALID_PARAM: aom_codec_err_t = 8;
pub const AOM_CODEC_CORRUPT_FRAME: aom_codec_err_t = 7;
pub const AOM_CODEC_UNSUP_FEATURE: aom_codec_err_t = 6;
pub const AOM_CODEC_UNSUP_BITSTREAM: aom_codec_err_t = 5;
pub const AOM_CODEC_INCAPABLE: aom_codec_err_t = 4;
pub const AOM_CODEC_ABI_MISMATCH: aom_codec_err_t = 3;
pub const AOM_CODEC_MEM_ERROR: aom_codec_err_t = 2;
pub const AOM_CODEC_ERROR: aom_codec_err_t = 1;
pub const AOM_CODEC_OK: aom_codec_err_t = 0;
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type aom_release_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
pub type aom_codec_frame_buffer_t = aom_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_get_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        size_t,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type BITSTREAM_PROFILE = int8_t;
pub type BLOCK_SIZE = core::ffi::c_uchar;
pub const BLOCK_LARGEST: BLOCK_SIZE = 15;
pub const BLOCK_INVALID: BLOCK_SIZE = 255;
pub const BLOCK_SIZES: BLOCK_SIZE = 16;
pub const BLOCK_SIZES_ALL: BLOCK_SIZE = 22;
pub const BLOCK_64X16: BLOCK_SIZE = 21;
pub const BLOCK_16X64: BLOCK_SIZE = 20;
pub const BLOCK_32X8: BLOCK_SIZE = 19;
pub const BLOCK_8X32: BLOCK_SIZE = 18;
pub const BLOCK_16X4: BLOCK_SIZE = 17;
pub const BLOCK_4X16: BLOCK_SIZE = 16;
pub const BLOCK_128X128: BLOCK_SIZE = 15;
pub const BLOCK_128X64: BLOCK_SIZE = 14;
pub const BLOCK_64X128: BLOCK_SIZE = 13;
pub const BLOCK_64X64: BLOCK_SIZE = 12;
pub const BLOCK_64X32: BLOCK_SIZE = 11;
pub const BLOCK_32X64: BLOCK_SIZE = 10;
pub const BLOCK_32X32: BLOCK_SIZE = 9;
pub const BLOCK_32X16: BLOCK_SIZE = 8;
pub const BLOCK_16X32: BLOCK_SIZE = 7;
pub const BLOCK_16X16: BLOCK_SIZE = 6;
pub const BLOCK_16X8: BLOCK_SIZE = 5;
pub const BLOCK_8X16: BLOCK_SIZE = 4;
pub const BLOCK_8X8: BLOCK_SIZE = 3;
pub const BLOCK_8X4: BLOCK_SIZE = 2;
pub const BLOCK_4X8: BLOCK_SIZE = 1;
pub const BLOCK_4X4: BLOCK_SIZE = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type TX_MODE = uint8_t;
pub type PLANE_TYPE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation {
    pub enabled: uint8_t,
    pub update_map: uint8_t,
    pub update_data: uint8_t,
    pub temporal_update: uint8_t,
    pub feature_data: [[int16_t; 8]; 8],
    pub feature_mask: [core::ffi::c_uint; 8],
    pub last_active_segid: core::ffi::c_int,
    pub segid_preskip: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1Common {
    pub current_frame: CurrentFrame,
    pub error: *mut aom_internal_error_info,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub superres_upscaled_width: core::ffi::c_int,
    pub superres_upscaled_height: core::ffi::c_int,
    pub superres_scale_denominator: uint8_t,
    pub buffer_removal_times: [uint32_t; 33],
    pub frame_presentation_time: uint32_t,
    pub prev_frame: *mut RefCntBuffer,
    pub cur_frame: *mut RefCntBuffer,
    pub remapped_ref_idx: [core::ffi::c_int; 8],
    pub sf_identity: scale_factors,
    pub ref_scale_factors: [scale_factors; 8],
    pub ref_frame_map: [*mut RefCntBuffer; 8],
    pub show_frame: core::ffi::c_int,
    pub showable_frame: core::ffi::c_int,
    pub show_existing_frame: core::ffi::c_int,
    pub features: FeatureFlags,
    pub mi_params: CommonModeInfoParams,
    pub quant_params: CommonQuantParams,
    pub seg: segmentation,
    pub last_frame_seg_map: *mut uint8_t,
    pub lf_info: loop_filter_info_n,
    pub lf: loopfilter,
    pub rst_info: [RestorationInfo; 3],
    pub rst_tmpbuf: *mut int32_t,
    pub rlbs: *mut RestorationLineBuffers,
    pub rst_frame: YV12_BUFFER_CONFIG,
    pub cdef_info: CdefInfo,
    pub film_grain_params: aom_film_grain_t,
    pub delta_q_info: DeltaQInfo,
    pub global_motion: [WarpedMotionParams; 8],
    pub seq_params: *mut SequenceHeader,
    pub fc: *mut FRAME_CONTEXT,
    pub default_frame_context: *mut FRAME_CONTEXT,
    pub tiles: CommonTileParams,
    pub buffer_pool: *mut BufferPool,
    pub above_contexts: CommonContexts,
    pub current_frame_id: core::ffi::c_int,
    pub ref_frame_id: [core::ffi::c_int; 8],
    pub tpl_mvs: *mut TPL_MV_REF,
    pub tpl_mvs_mem_size: core::ffi::c_int,
    pub ref_frame_sign_bias: [core::ffi::c_int; 8],
    pub ref_frame_side: [int8_t; 8],
    pub temporal_layer_id: core::ffi::c_int,
    pub spatial_layer_id: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPL_MV_REF {
    pub mfmv0: int_mv,
    pub ref_frame_offset: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
    pub as_fullmv: FULLPEL_MV,
}
pub type FULLPEL_MV = fullpel_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fullpel_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type MV = mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv {
    pub row: int16_t,
    pub col: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonContexts {
    pub partition: *mut *mut PARTITION_CONTEXT,
    pub entropy: [*mut *mut ENTROPY_CONTEXT; 3],
    pub txfm: *mut *mut TXFM_CONTEXT,
    pub num_planes: core::ffi::c_int,
    pub num_tile_rows: core::ffi::c_int,
    pub num_mi_cols: core::ffi::c_int,
}
pub type ENTROPY_CONTEXT = core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferPool {
    pub pool_mutex: pthread_mutex_t,
    pub cb_priv: *mut core::ffi::c_void,
    pub get_fb_cb: aom_get_frame_buffer_cb_fn_t,
    pub release_fb_cb: aom_release_frame_buffer_cb_fn_t,
    pub frame_bufs: *mut RefCntBuffer,
    pub num_frame_bufs: uint8_t,
    pub int_frame_buffers: InternalFrameBufferList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBufferList {
    pub num_internal_frame_buffers: core::ffi::c_int,
    pub int_fb: *mut InternalFrameBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBuffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub in_use: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefCntBuffer {
    pub ref_count: core::ffi::c_int,
    pub order_hint: core::ffi::c_uint,
    pub ref_order_hints: [core::ffi::c_uint; 7],
    pub display_order_hint: core::ffi::c_uint,
    pub ref_display_order_hint: [core::ffi::c_uint; 7],
    pub pyramid_level: core::ffi::c_uint,
    pub base_qindex: core::ffi::c_int,
    pub mvs: *mut MV_REF,
    pub seg_map: *mut uint8_t,
    pub seg: segmentation,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub global_motion: [WarpedMotionParams; 8],
    pub showable_frame: core::ffi::c_int,
    pub film_grain_params_present: uint8_t,
    pub film_grain_params: aom_film_grain_t,
    pub raw_frame_buffer: aom_codec_frame_buffer_t,
    pub buf: YV12_BUFFER_CONFIG,
    pub temporal_id: core::ffi::c_int,
    pub spatial_id: core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub interp_filter_selected: [core::ffi::c_int; 4],
    pub ref_deltas: [int8_t; 8],
    pub mode_deltas: [int8_t; 2],
    pub frame_context: FRAME_CONTEXT,
    pub filter_level: [core::ffi::c_int; 2],
}
pub type FRAME_CONTEXT = frame_contexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_contexts {
    pub txb_skip_cdf: [[[aom_cdf_prob; 3]; 13]; 5],
    pub eob_extra_cdf: [[[[aom_cdf_prob; 3]; 9]; 2]; 5],
    pub dc_sign_cdf: [[[aom_cdf_prob; 3]; 3]; 2],
    pub eob_flag_cdf16: [[[aom_cdf_prob; 6]; 2]; 2],
    pub eob_flag_cdf32: [[[aom_cdf_prob; 7]; 2]; 2],
    pub eob_flag_cdf64: [[[aom_cdf_prob; 8]; 2]; 2],
    pub eob_flag_cdf128: [[[aom_cdf_prob; 9]; 2]; 2],
    pub eob_flag_cdf256: [[[aom_cdf_prob; 10]; 2]; 2],
    pub eob_flag_cdf512: [[[aom_cdf_prob; 11]; 2]; 2],
    pub eob_flag_cdf1024: [[[aom_cdf_prob; 12]; 2]; 2],
    pub coeff_base_eob_cdf: [[[[aom_cdf_prob; 4]; 4]; 2]; 5],
    pub coeff_base_cdf: [[[[aom_cdf_prob; 5]; 42]; 2]; 5],
    pub coeff_br_cdf: [[[[aom_cdf_prob; 5]; 21]; 2]; 5],
    pub newmv_cdf: [[aom_cdf_prob; 3]; 6],
    pub zeromv_cdf: [[aom_cdf_prob; 3]; 2],
    pub refmv_cdf: [[aom_cdf_prob; 3]; 6],
    pub drl_cdf: [[aom_cdf_prob; 3]; 3],
    pub inter_compound_mode_cdf: [[aom_cdf_prob; 9]; 8],
    pub compound_type_cdf: [[aom_cdf_prob; 3]; 22],
    pub wedge_idx_cdf: [[aom_cdf_prob; 17]; 22],
    pub interintra_cdf: [[aom_cdf_prob; 3]; 4],
    pub wedge_interintra_cdf: [[aom_cdf_prob; 3]; 22],
    pub interintra_mode_cdf: [[aom_cdf_prob; 5]; 4],
    pub motion_mode_cdf: [[aom_cdf_prob; 4]; 22],
    pub obmc_cdf: [[aom_cdf_prob; 3]; 22],
    pub palette_y_size_cdf: [[aom_cdf_prob; 8]; 7],
    pub palette_uv_size_cdf: [[aom_cdf_prob; 8]; 7],
    pub palette_y_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7],
    pub palette_uv_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7],
    pub palette_y_mode_cdf: [[[aom_cdf_prob; 3]; 3]; 7],
    pub palette_uv_mode_cdf: [[aom_cdf_prob; 3]; 2],
    pub comp_inter_cdf: [[aom_cdf_prob; 3]; 5],
    pub single_ref_cdf: [[[aom_cdf_prob; 3]; 6]; 3],
    pub comp_ref_type_cdf: [[aom_cdf_prob; 3]; 5],
    pub uni_comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3],
    pub comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3],
    pub comp_bwdref_cdf: [[[aom_cdf_prob; 3]; 2]; 3],
    pub txfm_partition_cdf: [[aom_cdf_prob; 3]; 21],
    pub compound_index_cdf: [[aom_cdf_prob; 3]; 6],
    pub comp_group_idx_cdf: [[aom_cdf_prob; 3]; 6],
    pub skip_mode_cdfs: [[aom_cdf_prob; 3]; 3],
    pub skip_txfm_cdfs: [[aom_cdf_prob; 3]; 3],
    pub intra_inter_cdf: [[aom_cdf_prob; 3]; 4],
    pub nmvc: nmv_context,
    pub ndvc: nmv_context,
    pub intrabc_cdf: [aom_cdf_prob; 3],
    pub seg: segmentation_probs,
    pub filter_intra_cdfs: [[aom_cdf_prob; 3]; 22],
    pub filter_intra_mode_cdf: [aom_cdf_prob; 6],
    pub switchable_restore_cdf: [aom_cdf_prob; 4],
    pub wiener_restore_cdf: [aom_cdf_prob; 3],
    pub sgrproj_restore_cdf: [aom_cdf_prob; 3],
    pub y_mode_cdf: [[aom_cdf_prob; 14]; 4],
    pub uv_mode_cdf: [[[aom_cdf_prob; 15]; 13]; 2],
    pub partition_cdf: [[aom_cdf_prob; 11]; 20],
    pub switchable_interp_cdf: [[aom_cdf_prob; 4]; 16],
    pub kf_y_cdf: [[[aom_cdf_prob; 14]; 5]; 5],
    pub angle_delta_cdf: [[aom_cdf_prob; 8]; 8],
    pub tx_size_cdf: [[[aom_cdf_prob; 4]; 3]; 4],
    pub delta_q_cdf: [aom_cdf_prob; 5],
    pub delta_lf_multi_cdf: [[aom_cdf_prob; 5]; 4],
    pub delta_lf_cdf: [aom_cdf_prob; 5],
    pub intra_ext_tx_cdf: [[[[aom_cdf_prob; 17]; 13]; 4]; 3],
    pub inter_ext_tx_cdf: [[[aom_cdf_prob; 17]; 4]; 4],
    pub cfl_sign_cdf: [aom_cdf_prob; 9],
    pub cfl_alpha_cdf: [[aom_cdf_prob; 17]; 6],
    pub initialized: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmv_context {
    pub joints_cdf: [aom_cdf_prob; 5],
    pub comps: [nmv_component; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmv_component {
    pub classes_cdf: [aom_cdf_prob; 12],
    pub class0_fp_cdf: [[aom_cdf_prob; 5]; 2],
    pub fp_cdf: [aom_cdf_prob; 5],
    pub sign_cdf: [aom_cdf_prob; 3],
    pub class0_hp_cdf: [aom_cdf_prob; 3],
    pub hp_cdf: [aom_cdf_prob; 3],
    pub class0_cdf: [aom_cdf_prob; 3],
    pub bits_cdf: [[aom_cdf_prob; 3]; 10],
}
pub type FRAME_TYPE = uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WarpedMotionParams {
    pub wmmat: [int32_t; 6],
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
    pub wmtype: TransformationType,
    pub invalid: int8_t,
}
pub type TransformationType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV_REF {
    pub mv: int_mv,
    pub ref_frame: MV_REFERENCE_FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonTileParams {
    pub cols: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub max_width_sb: core::ffi::c_int,
    pub max_height_sb: core::ffi::c_int,
    pub min_inner_width: core::ffi::c_int,
    pub uniform_spacing: core::ffi::c_int,
    pub log2_cols: core::ffi::c_int,
    pub log2_rows: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub min_log2_cols: core::ffi::c_int,
    pub min_log2_rows: core::ffi::c_int,
    pub max_log2_cols: core::ffi::c_int,
    pub max_log2_rows: core::ffi::c_int,
    pub min_log2: core::ffi::c_int,
    pub col_start_sb: [core::ffi::c_int; 65],
    pub row_start_sb: [core::ffi::c_int; 65],
    pub large_scale: core::ffi::c_uint,
    pub single_tile_decoding: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SequenceHeader {
    pub num_bits_width: core::ffi::c_int,
    pub num_bits_height: core::ffi::c_int,
    pub max_frame_width: core::ffi::c_int,
    pub max_frame_height: core::ffi::c_int,
    pub frame_id_numbers_present_flag: uint8_t,
    pub frame_id_length: core::ffi::c_int,
    pub delta_frame_id_length: core::ffi::c_int,
    pub sb_size: BLOCK_SIZE,
    pub mib_size: core::ffi::c_int,
    pub mib_size_log2: core::ffi::c_int,
    pub order_hint_info: OrderHintInfo,
    pub force_screen_content_tools: uint8_t,
    pub still_picture: uint8_t,
    pub reduced_still_picture_hdr: uint8_t,
    pub force_integer_mv: uint8_t,
    pub enable_filter_intra: uint8_t,
    pub enable_intra_edge_filter: uint8_t,
    pub enable_interintra_compound: uint8_t,
    pub enable_masked_compound: uint8_t,
    pub enable_dual_filter: uint8_t,
    pub enable_warped_motion: uint8_t,
    pub enable_superres: uint8_t,
    pub enable_cdef: uint8_t,
    pub enable_restoration: uint8_t,
    pub profile: BITSTREAM_PROFILE,
    pub bit_depth: aom_bit_depth_t,
    pub use_highbitdepth: uint8_t,
    pub monochrome: uint8_t,
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub color_range: core::ffi::c_int,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub separate_uv_delta_q: uint8_t,
    pub film_grain_params_present: uint8_t,
    pub operating_points_cnt_minus_1: core::ffi::c_int,
    pub operating_point_idc: [core::ffi::c_int; 32],
    pub has_nonzero_operating_point_idc: bool,
    pub timing_info_present: core::ffi::c_int,
    pub timing_info: aom_timing_info_t,
    pub decoder_model_info_present_flag: uint8_t,
    pub decoder_model_info: aom_dec_model_info_t,
    pub display_model_info_present_flag: uint8_t,
    pub seq_level_idx: [AV1_LEVEL; 32],
    pub tier: [uint8_t; 32],
    pub op_params: [aom_dec_model_op_parameters_t; 33],
}
pub type aom_dec_model_op_parameters_t = aom_dec_model_op_parameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_op_parameters {
    pub decoder_model_param_present_flag: core::ffi::c_int,
    pub bitrate: int64_t,
    pub buffer_size: int64_t,
    pub decoder_buffer_delay: uint32_t,
    pub encoder_buffer_delay: uint32_t,
    pub low_delay_mode_flag: core::ffi::c_int,
    pub display_model_param_present_flag: core::ffi::c_int,
    pub initial_display_delay: core::ffi::c_int,
}
pub type aom_dec_model_info_t = aom_dec_model_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_info {
    pub num_units_in_decoding_tick: uint32_t,
    pub encoder_decoder_buffer_delay_length: core::ffi::c_int,
    pub buffer_removal_time_length: core::ffi::c_int,
    pub frame_presentation_time_length: core::ffi::c_int,
}
pub type aom_timing_info_t = aom_timing;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_timing {
    pub num_units_in_display_tick: uint32_t,
    pub time_scale: uint32_t,
    pub equal_picture_interval: core::ffi::c_int,
    pub num_ticks_per_picture: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OrderHintInfo {
    pub enable_order_hint: core::ffi::c_int,
    pub order_hint_bits_minus_1: core::ffi::c_int,
    pub enable_dist_wtd_comp: core::ffi::c_int,
    pub enable_ref_frame_mvs: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeltaQInfo {
    pub delta_q_present_flag: core::ffi::c_int,
    pub delta_q_res: core::ffi::c_int,
    pub delta_lf_present_flag: core::ffi::c_int,
    pub delta_lf_res: core::ffi::c_int,
    pub delta_lf_multi: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdefInfo {
    pub colbuf: [*mut uint16_t; 3],
    pub linebuf: [*mut uint16_t; 3],
    pub srcbuf: *mut uint16_t,
    pub allocated_colbuf_size: [size_t; 3],
    pub allocated_linebuf_size: [size_t; 3],
    pub allocated_srcbuf_size: size_t,
    pub cdef_damping: core::ffi::c_int,
    pub nb_cdef_strengths: core::ffi::c_int,
    pub cdef_strengths: [core::ffi::c_int; 16],
    pub cdef_uv_strengths: [core::ffi::c_int; 16],
    pub cdef_bits: core::ffi::c_int,
    pub allocated_mi_rows: core::ffi::c_int,
    pub allocated_num_workers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationLineBuffers {
    pub tmp_save_above: [[uint16_t; 392]; 3],
    pub tmp_save_below: [[uint16_t; 392]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationInfo {
    pub frame_restoration_type: RestorationType,
    pub restoration_unit_size: core::ffi::c_int,
    pub num_rest_units: core::ffi::c_int,
    pub vert_units: core::ffi::c_int,
    pub horz_units: core::ffi::c_int,
    pub unit_info: *mut RestorationUnitInfo,
    pub boundaries: RestorationStripeBoundaries,
    pub optimized_lr: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationStripeBoundaries {
    pub stripe_boundary_above: *mut uint8_t,
    pub stripe_boundary_below: *mut uint8_t,
    pub stripe_boundary_stride: core::ffi::c_int,
    pub stripe_boundary_size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationUnitInfo {
    pub restoration_type: RestorationType,
    pub wiener_info: WienerInfo,
    pub sgrproj_info: SgrprojInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SgrprojInfo {
    pub ep: core::ffi::c_int,
    pub xqd: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WienerInfo {
    pub __aom_align_vfilter: [crate::platform::Align16; 0],

    pub vfilter: InterpKernel,
    pub __aom_align_hfilter: [crate::platform::Align16; 0],
    pub hfilter: InterpKernel,
}
pub type InterpKernel = [int16_t; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loopfilter {
    pub filter_level: [core::ffi::c_int; 2],
    pub filter_level_u: core::ffi::c_int,
    pub filter_level_v: core::ffi::c_int,
    pub backup_filter_level: [core::ffi::c_int; 2],
    pub backup_filter_level_u: core::ffi::c_int,
    pub backup_filter_level_v: core::ffi::c_int,
    pub sharpness_level: core::ffi::c_int,
    pub mode_ref_delta_enabled: uint8_t,
    pub mode_ref_delta_update: uint8_t,
    pub ref_deltas: [int8_t; 8],
    pub mode_deltas: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub lfthr: [loop_filter_thresh; 64],
    pub lvl: [[[[[uint8_t; 2]; 8]; 2]; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_thresh {
    pub __aom_align_mblim: [crate::platform::Align16; 0],

    pub mblim: [uint8_t; 16],
    pub __aom_align_lim: [crate::platform::Align16; 0],
    pub lim: [uint8_t; 16],
    pub __aom_align_hev_thr: [crate::platform::Align16; 0],
    pub hev_thr: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonQuantParams {
    pub base_qindex: core::ffi::c_int,
    pub sharpness: core::ffi::c_int,
    pub y_dc_delta_q: core::ffi::c_int,
    pub u_dc_delta_q: core::ffi::c_int,
    pub v_dc_delta_q: core::ffi::c_int,
    pub u_ac_delta_q: core::ffi::c_int,
    pub v_ac_delta_q: core::ffi::c_int,
    pub y_dequant_QTX: [[int16_t; 2]; 8],
    pub u_dequant_QTX: [[int16_t; 2]; 8],
    pub v_dequant_QTX: [[int16_t; 2]; 8],
    pub giqmatrix: [[[*const qm_val_t; 19]; 3]; 16],
    pub gqmatrix: [[[*const qm_val_t; 19]; 3]; 16],
    pub y_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub u_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub v_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub using_qmatrix: bool,
    pub qmatrix_level_y: core::ffi::c_int,
    pub qmatrix_level_u: core::ffi::c_int,
    pub qmatrix_level_v: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonModeInfoParams {
    pub mb_rows: core::ffi::c_int,
    pub mb_cols: core::ffi::c_int,
    pub MBs: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_alloc: *mut MB_MODE_INFO,
    pub mi_alloc_size: core::ffi::c_int,
    pub mi_alloc_stride: core::ffi::c_int,
    pub mi_alloc_bsize: BLOCK_SIZE,
    pub mi_grid_base: *mut *mut MB_MODE_INFO,
    pub mi_grid_size: core::ffi::c_int,
    pub mi_stride: core::ffi::c_int,
    pub tx_type_map: *mut TX_TYPE,
    pub free_mi: Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>,
    pub setup_mi: Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>,
    pub set_mb_mi: Option<
        unsafe extern "C-unwind" fn(
            *mut CommonModeInfoParams,
            core::ffi::c_int,
            core::ffi::c_int,
            BLOCK_SIZE,
        ) -> (),
    >,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct MB_MODE_INFO {
    pub bsize: BLOCK_SIZE,
    pub partition: PARTITION_TYPE,
    pub mode: PREDICTION_MODE,
    pub uv_mode: UV_PREDICTION_MODE,
    pub current_qindex: core::ffi::c_int,
    pub mv: [int_mv; 2],
    pub ref_frame: [MV_REFERENCE_FRAME; 2],
    pub interp_filters: int_interpfilters,
    pub motion_mode: MOTION_MODE,
    pub num_proj_ref: uint8_t,
    pub overlappable_neighbors: uint8_t,
    pub wm_params: WarpedMotionParams,
    pub interintra_mode: INTERINTRA_MODE,
    pub interintra_wedge_index: int8_t,
    pub interinter_comp: INTERINTER_COMPOUND_DATA,
    pub angle_delta: [int8_t; 2],
    pub filter_intra_mode_info: FILTER_INTRA_MODE_INFO,
    pub cfl_alpha_signs: int8_t,
    pub cfl_alpha_idx: uint8_t,
    pub palette_mode_info: PALETTE_MODE_INFO,
    pub skip_txfm: uint8_t,
    pub tx_size: TX_SIZE,
    pub inter_tx_size: [TX_SIZE; 16],
    pub delta_lf_from_base: int8_t,
    pub delta_lf: [int8_t; 4],
    #[bitfield(name = "segment_id", ty = "uint8_t", bits = "0..=2")]
    #[bitfield(name = "seg_id_predicted", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "ref_mv_idx", ty = "uint8_t", bits = "4..=5")]
    #[bitfield(name = "skip_mode", ty = "uint8_t", bits = "6..=6")]
    #[bitfield(name = "use_intrabc", ty = "uint8_t", bits = "7..=7")]
    #[bitfield(name = "comp_group_idx", ty = "uint8_t", bits = "8..=8")]
    #[bitfield(name = "compound_idx", ty = "uint8_t", bits = "9..=9")]
    #[bitfield(name = "use_wedge_interintra", ty = "uint8_t", bits = "10..=10")]
    #[bitfield(name = "cdef_strength", ty = "int8_t", bits = "11..=14")]
    pub segment_id_seg_id_predicted_ref_mv_idx_skip_mode_use_intrabc_comp_group_idx_compound_idx_use_wedge_interintra_cdef_strength: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTE_MODE_INFO {
    pub palette_colors: [uint16_t; 24],
    pub palette_size: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER_INTRA_MODE_INFO {
    pub filter_intra_mode: FILTER_INTRA_MODE,
    pub use_filter_intra: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERINTER_COMPOUND_DATA {
    pub seg_mask: *mut uint8_t,
    pub wedge_index: int8_t,
    pub wedge_sign: int8_t,
    pub mask_type: DIFFWTD_MASK_TYPE,
    pub type_0: COMPOUND_TYPE,
}
pub type DIFFWTD_MASK_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_interpfilters {
    pub as_int: uint32_t,
    pub as_filters: InterpFilters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilters {
    pub y_filter: uint16_t,
    pub x_filter: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureFlags {
    pub disable_cdf_update: bool,
    pub allow_high_precision_mv: bool,
    pub cur_frame_force_integer_mv: bool,
    pub allow_screen_content_tools: bool,
    pub allow_intrabc: bool,
    pub allow_warped_motion: bool,
    pub allow_ref_frame_mvs: bool,
    pub coded_lossless: bool,
    pub all_lossless: bool,
    pub reduced_tx_set_used: bool,
    pub error_resilient_mode: bool,
    pub switchable_motion_mode: bool,
    pub tx_mode: TX_MODE,
    pub interp_filter: InterpFilter,
    pub primary_ref_frame: core::ffi::c_int,
    pub byte_alignment: core::ffi::c_int,
    pub refresh_frame_context: REFRESH_FRAME_CONTEXT_MODE,
}
pub type REFRESH_FRAME_CONTEXT_MODE = uint8_t;
pub type InterpFilter = core::ffi::c_uchar;
pub const INTERP_INVALID: InterpFilter = 255;
pub const EXTRA_FILTERS: InterpFilter = 2;
pub const SWITCHABLE: InterpFilter = 4;
pub const SWITCHABLE_FILTERS: InterpFilter = 3;
pub const INTERP_FILTERS_ALL: InterpFilter = 5;
pub const MULTITAP_SHARP2: InterpFilter = 4;
pub const BILINEAR: InterpFilter = 3;
pub const MULTITAP_SHARP: InterpFilter = 2;
pub const EIGHTTAP_SMOOTH: InterpFilter = 1;
pub const EIGHTTAP_REGULAR: InterpFilter = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CurrentFrame {
    pub frame_type: FRAME_TYPE,
    pub reference_mode: REFERENCE_MODE,
    pub order_hint: core::ffi::c_uint,
    pub display_order_hint: core::ffi::c_uint,
    pub pyramid_level: core::ffi::c_uint,
    pub frame_number: core::ffi::c_uint,
    pub skip_mode_info: SkipModeInfo,
    pub refresh_frame_flags: core::ffi::c_int,
    pub frame_refs_short_signaling: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkipModeInfo {
    pub skip_mode_allowed: core::ffi::c_int,
    pub skip_mode_flag: core::ffi::c_int,
    pub ref_frame_idx_0: core::ffi::c_int,
    pub ref_frame_idx_1: core::ffi::c_int,
}
pub type REFERENCE_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
    pub mi_row: core::ffi::c_int,
    pub mi_col: core::ffi::c_int,
    pub mi_stride: core::ffi::c_int,
    pub is_chroma_ref: bool,
    pub plane: [macroblockd_plane; 3],
    pub tile: TileInfo,
    pub mi: *mut *mut MB_MODE_INFO,
    pub up_available: bool,
    pub left_available: bool,
    pub chroma_up_available: bool,
    pub chroma_left_available: bool,
    pub left_mbmi: *mut MB_MODE_INFO,
    pub above_mbmi: *mut MB_MODE_INFO,
    pub chroma_left_mbmi: *mut MB_MODE_INFO,
    pub chroma_above_mbmi: *mut MB_MODE_INFO,
    pub tx_type_map: *mut uint8_t,
    pub tx_type_map_stride: core::ffi::c_int,
    pub mb_to_left_edge: core::ffi::c_int,
    pub mb_to_right_edge: core::ffi::c_int,
    pub mb_to_top_edge: core::ffi::c_int,
    pub mb_to_bottom_edge: core::ffi::c_int,
    pub block_ref_scale_factors: [*const scale_factors; 2],
    pub cur_buf: *const YV12_BUFFER_CONFIG,
    pub above_entropy_context: [*mut ENTROPY_CONTEXT; 3],
    pub left_entropy_context: [[ENTROPY_CONTEXT; 32]; 3],
    pub above_partition_context: *mut PARTITION_CONTEXT,
    pub left_partition_context: [PARTITION_CONTEXT; 32],
    pub above_txfm_context: *mut TXFM_CONTEXT,
    pub left_txfm_context: *mut TXFM_CONTEXT,
    pub left_txfm_context_buffer: [TXFM_CONTEXT; 32],
    pub wiener_info: [WienerInfo; 3],
    pub sgrproj_info: [SgrprojInfo; 3],
    pub width: uint8_t,
    pub height: uint8_t,
    pub ref_mv_stack: [[CANDIDATE_MV; 8]; 29],
    pub weight: [[uint16_t; 8]; 29],
    pub is_last_vertical_rect: bool,
    pub is_first_horizontal_rect: bool,
    pub neighbors_ref_counts: [uint8_t; 8],
    pub tile_ctx: *mut FRAME_CONTEXT,
    pub bd: core::ffi::c_int,
    pub qindex: [core::ffi::c_int; 8],
    pub lossless: [core::ffi::c_int; 8],
    pub current_base_qindex: core::ffi::c_int,
    pub cur_frame_force_integer_mv: core::ffi::c_int,
    pub error_info: *mut aom_internal_error_info,
    pub global_motion: *const WarpedMotionParams,
    pub delta_lf_from_base: int8_t,
    pub delta_lf: [int8_t; 4],
    pub cdef_transmitted: [bool; 4],
    pub seg_mask: *mut uint8_t,
    pub cfl: CFL_CTX,
    pub color_index_map_offset: [uint16_t; 2],
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub tmp_upsample_pred: *mut uint8_t,
}
pub type CONV_BUF_TYPE = uint16_t;
pub type CFL_CTX = cfl_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfl_ctx {
    pub recon_buf_q3: [uint16_t; 1024],
    pub ac_buf_q3: [int16_t; 1024],
    pub dc_pred_is_cached: [bool; 2],
    pub use_dc_pred_cache: bool,
    pub dc_pred_cache: [[int16_t; 32]; 2],
    pub buf_height: core::ffi::c_int,
    pub buf_width: core::ffi::c_int,
    pub are_parameters_computed: core::ffi::c_int,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub store_y: core::ffi::c_int,
}
pub type CANDIDATE_MV = candidate_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct candidate_mv {
    pub this_mv: int_mv,
    pub comp_mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileInfo {
    pub mi_row_start: core::ffi::c_int,
    pub mi_row_end: core::ffi::c_int,
    pub mi_col_start: core::ffi::c_int,
    pub mi_col_end: core::ffi::c_int,
    pub tile_row: core::ffi::c_int,
    pub tile_col: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd_plane {
    pub plane_type: PLANE_TYPE,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub dst: buf_2d,
    pub pre: [buf_2d; 2],
    pub above_entropy_context: *mut ENTROPY_CONTEXT,
    pub left_entropy_context: *mut ENTROPY_CONTEXT,
    pub seg_dequant_QTX: [[int16_t; 2]; 8],
    pub color_index_map: *mut uint8_t,
    pub width: uint8_t,
    pub height: uint8_t,
    pub seg_iqmatrix: [[*mut qm_val_t; 19]; 8],
    pub seg_qmatrix: [[*mut qm_val_t; 19]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_2d {
    pub buf: *mut uint8_t,
    pub buf0: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_12 = 4;
pub const S_FRAME: C2RustUnnamed_12 = 3;
pub const INTRA_ONLY_FRAME: C2RustUnnamed_12 = 2;
pub const INTER_FRAME: C2RustUnnamed_12 = 1;
pub const KEY_FRAME: C2RustUnnamed_12 = 0;
pub type MACROBLOCKD_PLANE = macroblockd_plane;
pub type MACROBLOCKD = macroblockd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_DEBLOCKING_PARAMETERS {
    pub filter_length: uint8_t,
    pub lfthr: *const loop_filter_thresh,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopFilterWorkerData {
    pub frame_buffer: *mut YV12_BUFFER_CONFIG,
    pub cm: *mut AV1Common,
    pub planes: [macroblockd_plane; 3],
    pub xd: *mut MACROBLOCKD,
    pub params_buf: [AV1_DEBLOCKING_PARAMETERS; 32],
    pub tx_buf: [TX_SIZE; 32],
    pub error_info: aom_internal_error_info,
}
pub type LFWorkerData = LoopFilterWorkerData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LfSyncData {
    pub mutex_: [*mut pthread_mutex_t; 3],
    pub cond_: [*mut pthread_cond_t; 3],
    pub cur_sb_col: [*mut core::ffi::c_int; 3],
    pub sync_range: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub lfdata: *mut LFWorkerData,
    pub num_workers: core::ffi::c_int,
    pub job_mutex: *mut pthread_mutex_t,
    pub job_queue: *mut AV1LfMTInfo,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub lf_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LfMTInfo {
    pub mi_row: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub dir: core::ffi::c_int,
    pub lpf_opt_level: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationTileLimits {
    pub h_start: core::ffi::c_int,
    pub h_end: core::ffi::c_int,
    pub v_start: core::ffi::c_int,
    pub v_end: core::ffi::c_int,
}
pub type rest_unit_visitor_t = Option<
    unsafe extern "C-unwind" fn(
        *const RestorationTileLimits,
        core::ffi::c_int,
        *mut core::ffi::c_void,
        *mut int32_t,
        *mut RestorationLineBuffers,
        *mut aom_internal_error_info,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterFrameCtxt {
    pub rsi: *const RestorationInfo,
    pub ss_x: core::ffi::c_int,
    pub ss_y: core::ffi::c_int,
    pub plane_w: core::ffi::c_int,
    pub plane_h: core::ffi::c_int,
    pub highbd: core::ffi::c_int,
    pub bit_depth: core::ffi::c_int,
    pub data8: *mut uint8_t,
    pub dst8: *mut uint8_t,
    pub data_stride: core::ffi::c_int,
    pub dst_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrStruct {
    pub on_rest_unit: rest_unit_visitor_t,
    pub ctxt: [FilterFrameCtxt; 3],
    pub frame: *mut YV12_BUFFER_CONFIG,
    pub dst: *mut YV12_BUFFER_CONFIG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrSyncData {
    pub mutex_: [*mut pthread_mutex_t; 3],
    pub cond_: [*mut pthread_cond_t; 3],
    pub cur_sb_col: [*mut core::ffi::c_int; 3],
    pub sync_range: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub num_planes: core::ffi::c_int,
    pub num_workers: core::ffi::c_int,
    pub job_mutex: *mut pthread_mutex_t,
    pub lrworkerdata: *mut LRWorkerData,
    pub job_queue: *mut AV1LrMTInfo,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub lr_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1LrMTInfo {
    pub v_start: core::ffi::c_int,
    pub v_end: core::ffi::c_int,
    pub lr_unit_row: core::ffi::c_int,
    pub plane: core::ffi::c_int,
    pub sync_mode: core::ffi::c_int,
    pub v_copy_start: core::ffi::c_int,
    pub v_copy_end: core::ffi::c_int,
}
pub type LRWorkerData = LoopRestorationWorkerData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoopRestorationWorkerData {
    pub rst_tmpbuf: *mut int32_t,
    pub rlbs: *mut core::ffi::c_void,
    pub lr_ctxt: *mut core::ffi::c_void,
    pub do_extend_border: core::ffi::c_int,
    pub error_info: aom_internal_error_info,
}
pub type sync_read_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type sync_write_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1CdefWorker {
    pub cm: *mut AV1_COMMON,
    pub xd: *mut MACROBLOCKD,
    pub colbuf: [*mut uint16_t; 3],
    pub srcbuf: *mut uint16_t,
    pub linebuf: [*mut uint16_t; 3],
    pub cdef_init_fb_row_fn: cdef_init_fb_row_t,
    pub do_extend_border: core::ffi::c_int,
    pub error_info: aom_internal_error_info,
}
pub type cdef_init_fb_row_t = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMMON,
        *const MACROBLOCKD,
        *mut CdefBlockInfo,
        *mut *mut uint16_t,
        *mut uint16_t,
        *mut AV1CdefSyncData,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1CdefSyncData {
    pub mutex_: *mut pthread_mutex_t,
    pub cdef_row_mt: *mut AV1CdefRowSync,
    pub end_of_frame: core::ffi::c_int,
    pub fbr: core::ffi::c_int,
    pub fbc: core::ffi::c_int,
    pub cdef_mt_exit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1CdefRowSync {
    pub row_mutex_: *mut pthread_mutex_t,
    pub row_cond_: *mut pthread_cond_t,
    pub is_row_done: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdefBlockInfo {
    pub src: *mut uint16_t,
    pub top_linebuf: [*mut uint16_t; 3],
    pub bot_linebuf: [*mut uint16_t; 3],
    pub dst: *mut uint8_t,
    pub dlist: [cdef_list; 256],
    pub xdec: core::ffi::c_int,
    pub ydec: core::ffi::c_int,
    pub mi_wide_l2: core::ffi::c_int,
    pub mi_high_l2: core::ffi::c_int,
    pub frame_boundary: [core::ffi::c_int; 4],
    pub damping: core::ffi::c_int,
    pub coeff_shift: core::ffi::c_int,
    pub level: core::ffi::c_int,
    pub sec_strength: core::ffi::c_int,
    pub cdef_count: core::ffi::c_int,
    pub dir: [[core::ffi::c_int; 16]; 16],
    pub var: [[core::ffi::c_int; 16]; 16],
    pub dst_stride: core::ffi::c_int,
    pub coffset: core::ffi::c_int,
    pub roffset: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdef_list {
    pub by: uint8_t,
    pub bx: uint8_t,
}
pub type AV1_COMMON = AV1Common;
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const BOUNDARIES: C2RustUnnamed_13 = 4;
pub const RIGHT: C2RustUnnamed_13 = 3;
pub const BOTTOM: C2RustUnnamed_13 = 2;
pub const LEFT: C2RustUnnamed_13 = 1;
pub const TOP: C2RustUnnamed_13 = 0;
pub type AV1LfSync = AV1LfSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
pub type copy_fun = Option<
    unsafe extern "C-unwind" fn(
        *const YV12_BUFFER_CONFIG,
        *mut YV12_BUFFER_CONFIG,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const MAX_MIB_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_MIB_SIZE_LOG2;
pub const MI_SIZE_64X64: core::ffi::c_int = 64 as core::ffi::c_int >> MI_SIZE_LOG2;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const CDEF_VBORDER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const RESTORATION_UNIT_OFFSET: core::ffi::c_int = 8 as core::ffi::c_int;
pub const RESTORATION_BORDER: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_sync_range(mut width: core::ffi::c_int) -> core::ffi::c_int {
    if width < 640 as core::ffi::c_int {
        return 1 as core::ffi::c_int
    } else if width <= 1280 as core::ffi::c_int {
        return 2 as core::ffi::c_int
    } else if width <= 4096 as core::ffi::c_int {
        return 4 as core::ffi::c_int
    } else {
        return 8 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn get_lr_sync_range(mut width: core::ffi::c_int) -> core::ffi::c_int {
    return 1 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_filter_alloc(
    mut lf_sync: *mut AV1LfSync,
    mut cm: *mut AV1_COMMON,
    mut rows: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut num_workers: core::ffi::c_int,
) {
    (*lf_sync).rows = rows;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < MAX_MB_PLANE {
        (*lf_sync).mutex_[j as usize] = aom_malloc(
            (::core::mem::size_of::<pthread_mutex_t>() as size_t)
                .wrapping_mul(rows as size_t),
        ) as *mut pthread_mutex_t;
        if ((*lf_sync).mutex_[j as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate lf_sync->mutex_[j]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if !((*lf_sync).mutex_[j as usize]).is_null() {
            i = 0 as core::ffi::c_int;
            while i < rows {
                pthread_mutex_init(
                    &raw mut *(*((*lf_sync).mutex_).as_mut_ptr().offset(j as isize))
                        .offset(i as isize),
                    0 as *const pthread_mutexattr_t,
                );
                i += 1;
            }
        }
        (*lf_sync).cond_[j as usize] = aom_malloc(
            (::core::mem::size_of::<pthread_cond_t>() as size_t)
                .wrapping_mul(rows as size_t),
        ) as *mut pthread_cond_t;
        if ((*lf_sync).cond_[j as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate lf_sync->cond_[j]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if !((*lf_sync).cond_[j as usize]).is_null() {
            i = 0 as core::ffi::c_int;
            while i < rows {
                pthread_cond_init(
                    &raw mut *(*((*lf_sync).cond_).as_mut_ptr().offset(j as isize))
                        .offset(i as isize),
                    0 as *const pthread_condattr_t,
                );
                i += 1;
            }
        }
        j += 1;
    }
    (*lf_sync).job_mutex = aom_malloc(
        ::core::mem::size_of::<pthread_mutex_t>() as size_t,
    ) as *mut pthread_mutex_t;
    if ((*lf_sync).job_mutex).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate lf_sync->job_mutex\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if !((*lf_sync).job_mutex).is_null() {
        pthread_mutex_init((*lf_sync).job_mutex, 0 as *const pthread_mutexattr_t);
    }
    (*lf_sync).lfdata = aom_malloc(
        (num_workers as size_t)
            .wrapping_mul(::core::mem::size_of::<LFWorkerData>() as size_t),
    ) as *mut LFWorkerData;
    if ((*lf_sync).lfdata).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate lf_sync->lfdata\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*lf_sync).num_workers = num_workers;
    let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while j_0 < MAX_MB_PLANE {
        (*lf_sync).cur_sb_col[j_0 as usize] = aom_malloc(
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(rows as size_t),
        ) as *mut core::ffi::c_int;
        if ((*lf_sync).cur_sb_col[j_0 as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate lf_sync->cur_sb_col[j]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        j_0 += 1;
    }
    (*lf_sync).job_queue = aom_malloc(
        (::core::mem::size_of::<AV1LfMTInfo>() as size_t)
            .wrapping_mul(rows as size_t)
            .wrapping_mul(3 as size_t)
            .wrapping_mul(2 as size_t),
    ) as *mut AV1LfMTInfo;
    if ((*lf_sync).job_queue).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate lf_sync->job_queue\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*lf_sync).sync_range = get_sync_range(width);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_filter_dealloc(mut lf_sync: *mut AV1LfSync) {
    if !lf_sync.is_null() {
        let mut j: core::ffi::c_int = 0;
        let mut i: core::ffi::c_int = 0;
        j = 0 as core::ffi::c_int;
        while j < MAX_MB_PLANE {
            if !((*lf_sync).mutex_[j as usize]).is_null() {
                i = 0 as core::ffi::c_int;
                while i < (*lf_sync).rows {
                    pthread_mutex_destroy(
                        &raw mut *(*((*lf_sync).mutex_).as_mut_ptr().offset(j as isize))
                            .offset(i as isize),
                    );
                    i += 1;
                }
                aom_free((*lf_sync).mutex_[j as usize] as *mut core::ffi::c_void);
            }
            if !((*lf_sync).cond_[j as usize]).is_null() {
                i = 0 as core::ffi::c_int;
                while i < (*lf_sync).rows {
                    pthread_cond_destroy(
                        &raw mut *(*((*lf_sync).cond_).as_mut_ptr().offset(j as isize))
                            .offset(i as isize),
                    );
                    i += 1;
                }
                aom_free((*lf_sync).cond_[j as usize] as *mut core::ffi::c_void);
            }
            j += 1;
        }
        if !((*lf_sync).job_mutex).is_null() {
            pthread_mutex_destroy((*lf_sync).job_mutex);
            aom_free((*lf_sync).job_mutex as *mut core::ffi::c_void);
        }
        aom_free((*lf_sync).lfdata as *mut core::ffi::c_void);
        j = 0 as core::ffi::c_int;
        while j < MAX_MB_PLANE {
            aom_free((*lf_sync).cur_sb_col[j as usize] as *mut core::ffi::c_void);
            j += 1;
        }
        aom_free((*lf_sync).job_queue as *mut core::ffi::c_void);
        memset(
            lf_sync as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<AV1LfSync>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_alloc_cdef_sync(
    cm: *mut AV1_COMMON,
    mut cdef_sync: *mut AV1CdefSync,
    mut num_workers: core::ffi::c_int,
) {
    if num_workers < 1 as core::ffi::c_int {
        return;
    }
    if ((*cdef_sync).mutex_).is_null() {
        (*cdef_sync).mutex_ = aom_malloc(
            ::core::mem::size_of::<pthread_mutex_t>() as size_t,
        ) as *mut pthread_mutex_t;
        if ((*cdef_sync).mutex_).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate cdef_sync->mutex_\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if !((*cdef_sync).mutex_).is_null() {
            pthread_mutex_init((*cdef_sync).mutex_, 0 as *const pthread_mutexattr_t);
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_free_cdef_sync(mut cdef_sync: *mut AV1CdefSync) {
    if cdef_sync.is_null() {
        return;
    }
    if !((*cdef_sync).mutex_).is_null() {
        pthread_mutex_destroy((*cdef_sync).mutex_);
        aom_free((*cdef_sync).mutex_ as *mut core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C-unwind" fn cdef_row_mt_sync_read(
    cdef_sync: *mut AV1CdefSync,
    mut row: core::ffi::c_int,
) {
    if row == 0 {
        return;
    }
    let cdef_row_mt: *mut AV1CdefRowSync = (*cdef_sync).cdef_row_mt;
    pthread_mutex_lock(
        (*cdef_row_mt.offset((row - 1 as core::ffi::c_int) as isize)).row_mutex_,
    );
    while (*cdef_row_mt.offset((row - 1 as core::ffi::c_int) as isize)).is_row_done
        != 1 as core::ffi::c_int
    {
        pthread_cond_wait(
            (*cdef_row_mt.offset((row - 1 as core::ffi::c_int) as isize)).row_cond_,
            (*cdef_row_mt.offset((row - 1 as core::ffi::c_int) as isize)).row_mutex_,
        );
    }
    (*cdef_row_mt.offset((row - 1 as core::ffi::c_int) as isize)).is_row_done = 0
        as core::ffi::c_int;
    pthread_mutex_unlock(
        (*cdef_row_mt.offset((row - 1 as core::ffi::c_int) as isize)).row_mutex_,
    );
}
#[inline]
unsafe extern "C-unwind" fn cdef_row_mt_sync_write(
    cdef_sync: *mut AV1CdefSync,
    mut row: core::ffi::c_int,
) {
    let cdef_row_mt: *mut AV1CdefRowSync = (*cdef_sync).cdef_row_mt;
    pthread_mutex_lock((*cdef_row_mt.offset(row as isize)).row_mutex_);
    pthread_cond_signal((*cdef_row_mt.offset(row as isize)).row_cond_);
    (*cdef_row_mt.offset(row as isize)).is_row_done = 1 as core::ffi::c_int;
    pthread_mutex_unlock((*cdef_row_mt.offset(row as isize)).row_mutex_);
}
#[inline]
unsafe extern "C-unwind" fn sync_read(
    lf_sync: *mut AV1LfSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let nsync: core::ffi::c_int = (*lf_sync).sync_range;
    if r != 0 && c & nsync - 1 as core::ffi::c_int == 0 {
        let mutex: *mut pthread_mutex_t = &raw mut *(*((*lf_sync).mutex_)
            .as_mut_ptr()
            .offset(plane as isize))
            .offset((r - 1 as core::ffi::c_int) as isize) as *mut pthread_mutex_t;
        pthread_mutex_lock(mutex);
        while c
            > *((*lf_sync).cur_sb_col[plane as usize])
                .offset((r - 1 as core::ffi::c_int) as isize) - nsync
        {
            pthread_cond_wait(
                &raw mut *(*((*lf_sync).cond_).as_mut_ptr().offset(plane as isize))
                    .offset((r - 1 as core::ffi::c_int) as isize),
                mutex,
            );
        }
        pthread_mutex_unlock(mutex);
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_write(
    lf_sync: *mut AV1LfSync,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    sb_cols: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let nsync: core::ffi::c_int = (*lf_sync).sync_range;
    let mut cur: core::ffi::c_int = 0;
    let mut sig: core::ffi::c_int = 1 as core::ffi::c_int;
    if c < sb_cols - 1 as core::ffi::c_int {
        cur = c;
        if c % nsync != 0 {
            sig = 0 as core::ffi::c_int;
        }
    } else {
        cur = sb_cols + nsync;
    }
    if sig != 0 {
        pthread_mutex_lock(
            &raw mut *(*((*lf_sync).mutex_).as_mut_ptr().offset(plane as isize))
                .offset(r as isize),
        );
        *((*lf_sync).cur_sb_col[plane as usize]).offset(r as isize) = if *((*lf_sync)
            .cur_sb_col[plane as usize])
            .offset(r as isize) > cur
        {
            *((*lf_sync).cur_sb_col[plane as usize]).offset(r as isize)
        } else {
            cur
        };
        pthread_cond_broadcast(
            &raw mut *(*((*lf_sync).cond_).as_mut_ptr().offset(plane as isize))
                .offset(r as isize),
        );
        pthread_mutex_unlock(
            &raw mut *(*((*lf_sync).mutex_).as_mut_ptr().offset(plane as isize))
                .offset(r as isize),
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_thread_loop_filter_rows(
    frame_buffer: *const YV12_BUFFER_CONFIG,
    cm: *mut AV1_COMMON,
    mut planes: *mut macroblockd_plane,
    mut xd: *mut MACROBLOCKD,
    mut mi_row: core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut lpf_opt_level: core::ffi::c_int,
    lf_sync: *mut AV1LfSync,
    mut error_info: *mut aom_internal_error_info,
    mut params_buf: *mut AV1_DEBLOCKING_PARAMETERS,
    mut tx_buf: *mut TX_SIZE,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let sb_cols: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int - 2 as core::ffi::c_int)
        - 1 as core::ffi::c_int >> 7 as core::ffi::c_int - 2 as core::ffi::c_int;
    let r: core::ffi::c_int = mi_row >> num_mis_in_lpf_unit_height_log2;
    let mut mi_col: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let joint_filter_chroma: bool = lpf_opt_level == 2 as core::ffi::c_int
        && plane > AOM_PLANE_Y;
    let num_planes: core::ffi::c_int = if joint_filter_chroma as core::ffi::c_int != 0 {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    if dir == 0 as core::ffi::c_int {
        mi_col = 0 as core::ffi::c_int;
        while mi_col < (*cm).mi_params.mi_cols {
            c = mi_col >> MAX_MIB_SIZE_LOG2;
            av1_setup_dst_planes(
                planes,
                (*(*cm).seq_params).sb_size,
                frame_buffer,
                mi_row,
                mi_col,
                plane,
                plane + num_planes,
            );
            if lpf_opt_level != 0 {
                if plane == AOM_PLANE_Y {
                    av1_filter_block_plane_vert_opt(
                        cm,
                        xd,
                        &raw mut *planes.offset(plane as isize),
                        mi_row as uint32_t,
                        mi_col as uint32_t,
                        params_buf,
                        tx_buf,
                        num_mis_in_lpf_unit_height_log2,
                    );
                } else {
                    av1_filter_block_plane_vert_opt_chroma(
                        cm,
                        xd,
                        &raw mut *planes.offset(plane as isize),
                        mi_row as uint32_t,
                        mi_col as uint32_t,
                        params_buf,
                        tx_buf,
                        plane,
                        joint_filter_chroma,
                        num_mis_in_lpf_unit_height_log2,
                    );
                }
            } else {
                av1_filter_block_plane_vert(
                    cm,
                    xd,
                    plane,
                    &raw mut *planes.offset(plane as isize),
                    mi_row as uint32_t,
                    mi_col as uint32_t,
                );
            }
            if !lf_sync.is_null() {
                sync_write(lf_sync, r, c, sb_cols, plane);
            }
            mi_col += MAX_MIB_SIZE;
        }
    } else if dir == 1 as core::ffi::c_int {
        mi_col = 0 as core::ffi::c_int;
        while mi_col < (*cm).mi_params.mi_cols {
            c = mi_col >> MAX_MIB_SIZE_LOG2;
            if !lf_sync.is_null() {
                sync_read(lf_sync, r, c, plane);
                sync_read(lf_sync, r + 1 as core::ffi::c_int, c, plane);
            }
            if !lf_sync.is_null() && (*lf_sync).num_workers > 1 as core::ffi::c_int {
                pthread_mutex_lock((*lf_sync).job_mutex);
                let lf_mt_exit: bool = (*lf_sync).lf_mt_exit;
                pthread_mutex_unlock((*lf_sync).job_mutex);
                if lf_mt_exit {
                    return;
                }
            }
            av1_setup_dst_planes(
                planes,
                (*(*cm).seq_params).sb_size,
                frame_buffer,
                mi_row,
                mi_col,
                plane,
                plane + num_planes,
            );
            if lpf_opt_level != 0 {
                if plane == AOM_PLANE_Y {
                    av1_filter_block_plane_horz_opt(
                        cm,
                        xd,
                        &raw mut *planes.offset(plane as isize),
                        mi_row as uint32_t,
                        mi_col as uint32_t,
                        params_buf,
                        tx_buf,
                        num_mis_in_lpf_unit_height_log2,
                    );
                } else {
                    av1_filter_block_plane_horz_opt_chroma(
                        cm,
                        xd,
                        &raw mut *planes.offset(plane as isize),
                        mi_row as uint32_t,
                        mi_col as uint32_t,
                        params_buf,
                        tx_buf,
                        plane,
                        joint_filter_chroma,
                        num_mis_in_lpf_unit_height_log2,
                    );
                }
            } else {
                av1_filter_block_plane_horz(
                    cm,
                    xd,
                    plane,
                    &raw mut *planes.offset(plane as isize),
                    mi_row as uint32_t,
                    mi_col as uint32_t,
                );
            }
            mi_col += MAX_MIB_SIZE;
        }
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_vert_loop_filter_done(
    mut cm: *mut AV1_COMMON,
    mut lf_sync: *mut AV1LfSync,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let mut plane: core::ffi::c_int = 0;
    let mut sb_row: core::ffi::c_int = 0;
    let sb_cols: core::ffi::c_int = (*cm).mi_params.mi_cols
        + ((1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2)
        - 1 as core::ffi::c_int >> num_mis_in_lpf_unit_height_log2;
    let sb_rows: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2)
        - 1 as core::ffi::c_int >> num_mis_in_lpf_unit_height_log2;
    sb_row = 0 as core::ffi::c_int;
    while sb_row < sb_rows {
        plane = 0 as core::ffi::c_int;
        while plane < MAX_MB_PLANE {
            sync_write(lf_sync, sb_row, sb_cols - 1 as core::ffi::c_int, sb_cols, plane);
            plane += 1;
        }
        sb_row += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_lf_workers(
    workers: *mut AVxWorker,
    cm: *mut AV1_COMMON,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut had_error: core::ffi::c_int = (*workers
        .offset(0 as core::ffi::c_int as isize))
        .had_error;
    let mut error_info: aom_internal_error_info = aom_internal_error_info {
        error_code: AOM_CODEC_OK,
        has_detail: 0,
        detail: [0; 200],
        setjmp: 0,
        jmp: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    if had_error != 0 {
        let worker: *mut AVxWorker = &raw mut *workers.offset(0 as core::ffi::c_int as isize)
            as *mut AVxWorker;
        error_info = (*((*worker).data2 as *mut LFWorkerData)).error_info;
    }
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i > 0 as core::ffi::c_int {
        let worker_0: *mut AVxWorker = &raw mut *workers.offset(i as isize)
            as *mut AVxWorker;
        if ((*winterface).sync).expect("non-null function pointer")(worker_0) == 0 {
            had_error = 1 as core::ffi::c_int;
            error_info = (*((*worker_0).data2 as *mut LFWorkerData)).error_info;
        }
        i -= 1;
    }
    if had_error != 0 {
        aom_internal_error_copy((*cm).error, &raw mut error_info);
    }
}
unsafe extern "C-unwind" fn loop_filter_row_worker(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let lf_sync: *mut AV1LfSync = arg1 as *mut AV1LfSync;
    let lf_data: *mut LFWorkerData = arg2 as *mut LFWorkerData;
    let mut cur_job_info: *mut AV1LfMTInfo = 0 as *mut AV1LfMTInfo;
    let mut job_mutex_: *mut pthread_mutex_t = (*lf_sync).job_mutex;
    let error_info: *mut aom_internal_error_info = &raw mut (*lf_data).error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    loop {
        cur_job_info = get_lf_job_info(lf_sync);
        if cur_job_info.is_null() {
            break;
        }
        let lpf_opt_level: core::ffi::c_int = (*cur_job_info).lpf_opt_level;
        av1_thread_loop_filter_rows(
            (*lf_data).frame_buffer,
            (*lf_data).cm as *mut AV1_COMMON,
            ((*lf_data).planes).as_mut_ptr(),
            (*lf_data).xd,
            (*cur_job_info).mi_row,
            (*cur_job_info).plane,
            (*cur_job_info).dir,
            lpf_opt_level,
            lf_sync,
            error_info,
            ((*lf_data).params_buf).as_mut_ptr(),
            ((*lf_data).tx_buf).as_mut_ptr(),
            MAX_MIB_SIZE_LOG2,
        );
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(job_mutex_);
        (*lf_sync).lf_mt_exit = true_0 != 0;
        pthread_mutex_unlock(job_mutex_);
        av1_set_vert_loop_filter_done(
            (*lf_data).cm as *mut AV1_COMMON,
            lf_sync,
            MAX_MIB_SIZE_LOG2,
        );
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn loop_filter_rows_mt(
    mut frame: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut start: core::ffi::c_int,
    mut stop: core::ffi::c_int,
    mut planes_to_lf: *const core::ffi::c_int,
    mut workers: *mut AVxWorker,
    mut num_workers: core::ffi::c_int,
    mut lf_sync: *mut AV1LfSync,
    mut lpf_opt_level: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut i: core::ffi::c_int = 0;
    loop_filter_frame_mt_init(
        cm,
        start,
        stop,
        planes_to_lf,
        num_workers,
        lf_sync,
        lpf_opt_level,
        MAX_MIB_SIZE_LOG2,
    );
    i = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *workers.offset(i as isize) as *mut AVxWorker;
        let lf_data: *mut LFWorkerData = &raw mut *((*lf_sync).lfdata).offset(i as isize)
            as *mut LFWorkerData;
        (*worker).hook = Some(
            loop_filter_row_worker
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ) as AVxWorkerHook;
        (*worker).data1 = lf_sync as *mut core::ffi::c_void;
        (*worker).data2 = lf_data as *mut core::ffi::c_void;
        loop_filter_data_reset(lf_data, frame, cm as *mut AV1Common, xd);
        (*worker).had_error = 0 as core::ffi::c_int;
        if i == 0 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        i -= 1;
    }
    sync_lf_workers(workers, cm, num_workers);
}
unsafe extern "C-unwind" fn loop_filter_rows(
    mut frame: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut start: core::ffi::c_int,
    mut stop: core::ffi::c_int,
    mut planes_to_lf: *const core::ffi::c_int,
    mut lpf_opt_level: core::ffi::c_int,
) {
    let mut mi_row: core::ffi::c_int = 0;
    let mut plane: core::ffi::c_int = 0;
    let mut dir: core::ffi::c_int = 0;
    let mut params_buf: [AV1_DEBLOCKING_PARAMETERS; 32] = [AV1_DEBLOCKING_PARAMETERS {
        filter_length: 0,
        lfthr: 0 as *const loop_filter_thresh,
    }; 32];
    let mut tx_buf: [TX_SIZE; 32] = [0; 32];
    mi_row = start;
    while mi_row < stop {
        plane = 0 as core::ffi::c_int;
        while plane < MAX_MB_PLANE {
            if !skip_loop_filter_plane(planes_to_lf, plane, lpf_opt_level) {
                dir = 0 as core::ffi::c_int;
                while dir < 2 as core::ffi::c_int {
                    av1_thread_loop_filter_rows(
                        frame,
                        cm,
                        ((*xd).plane).as_mut_ptr(),
                        xd,
                        mi_row,
                        plane,
                        dir,
                        lpf_opt_level,
                        0 as *mut AV1LfSync,
                        (*xd).error_info,
                        params_buf.as_mut_ptr(),
                        tx_buf.as_mut_ptr(),
                        MAX_MIB_SIZE_LOG2,
                    );
                    dir += 1;
                }
            }
            plane += 1;
        }
        mi_row += MAX_MIB_SIZE;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_filter_frame_mt(
    mut frame: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane_start: core::ffi::c_int,
    mut plane_end: core::ffi::c_int,
    mut partial_frame: core::ffi::c_int,
    mut workers: *mut AVxWorker,
    mut num_workers: core::ffi::c_int,
    mut lf_sync: *mut AV1LfSync,
    mut lpf_opt_level: core::ffi::c_int,
) {
    let mut start_mi_row: core::ffi::c_int = 0;
    let mut end_mi_row: core::ffi::c_int = 0;
    let mut mi_rows_to_filter: core::ffi::c_int = 0;
    let mut planes_to_lf: [core::ffi::c_int; 3] = [0; 3];
    if check_planes_to_loop_filter(
        &raw mut (*cm).lf,
        planes_to_lf.as_mut_ptr(),
        plane_start,
        plane_end,
    ) == 0
    {
        return;
    }
    start_mi_row = 0 as core::ffi::c_int;
    mi_rows_to_filter = (*cm).mi_params.mi_rows;
    if partial_frame != 0 && (*cm).mi_params.mi_rows > 8 as core::ffi::c_int {
        start_mi_row = (*cm).mi_params.mi_rows >> 1 as core::ffi::c_int;
        start_mi_row = (start_mi_row as core::ffi::c_uint
            & 0xfffffff8 as core::ffi::c_uint) as core::ffi::c_int;
        mi_rows_to_filter = if (*cm).mi_params.mi_rows / 8 as core::ffi::c_int
            > 8 as core::ffi::c_int
        {
            (*cm).mi_params.mi_rows / 8 as core::ffi::c_int
        } else {
            8 as core::ffi::c_int
        };
    }
    end_mi_row = start_mi_row + mi_rows_to_filter;
    av1_loop_filter_frame_init(cm as *mut AV1Common, plane_start, plane_end);
    if num_workers > 1 as core::ffi::c_int {
        loop_filter_rows_mt(
            frame,
            cm,
            xd,
            start_mi_row,
            end_mi_row,
            planes_to_lf.as_mut_ptr() as *const core::ffi::c_int,
            workers,
            num_workers,
            lf_sync,
            lpf_opt_level,
        );
    } else {
        loop_filter_rows(
            frame,
            cm,
            xd,
            start_mi_row,
            end_mi_row,
            planes_to_lf.as_mut_ptr() as *const core::ffi::c_int,
            lpf_opt_level,
        );
    };
}
#[inline]
unsafe extern "C-unwind" fn lr_sync_read(
    lr_sync: *mut core::ffi::c_void,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let loop_res_sync: *mut AV1LrSync = lr_sync as *mut AV1LrSync;
    let nsync: core::ffi::c_int = (*loop_res_sync).sync_range;
    if r != 0 && c & nsync - 1 as core::ffi::c_int == 0 {
        let mutex: *mut pthread_mutex_t = &raw mut *(*((*loop_res_sync).mutex_)
            .as_mut_ptr()
            .offset(plane as isize))
            .offset((r - 1 as core::ffi::c_int) as isize) as *mut pthread_mutex_t;
        pthread_mutex_lock(mutex);
        while c
            > *((*loop_res_sync).cur_sb_col[plane as usize])
                .offset((r - 1 as core::ffi::c_int) as isize) - nsync
        {
            pthread_cond_wait(
                &raw mut *(*((*loop_res_sync).cond_).as_mut_ptr().offset(plane as isize))
                    .offset((r - 1 as core::ffi::c_int) as isize),
                mutex,
            );
        }
        pthread_mutex_unlock(mutex);
    }
}
#[inline]
unsafe extern "C-unwind" fn lr_sync_write(
    lr_sync: *mut core::ffi::c_void,
    mut r: core::ffi::c_int,
    mut c: core::ffi::c_int,
    sb_cols: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let loop_res_sync: *mut AV1LrSync = lr_sync as *mut AV1LrSync;
    let nsync: core::ffi::c_int = (*loop_res_sync).sync_range;
    let mut cur: core::ffi::c_int = 0;
    let mut sig: core::ffi::c_int = 1 as core::ffi::c_int;
    if c < sb_cols - 1 as core::ffi::c_int {
        cur = c;
        if c % nsync != 0 {
            sig = 0 as core::ffi::c_int;
        }
    } else {
        cur = sb_cols + nsync;
    }
    if sig != 0 {
        pthread_mutex_lock(
            &raw mut *(*((*loop_res_sync).mutex_).as_mut_ptr().offset(plane as isize))
                .offset(r as isize),
        );
        *((*loop_res_sync).cur_sb_col[plane as usize]).offset(r as isize) = if *((*loop_res_sync)
            .cur_sb_col[plane as usize])
            .offset(r as isize) > cur
        {
            *((*loop_res_sync).cur_sb_col[plane as usize]).offset(r as isize)
        } else {
            cur
        };
        pthread_cond_broadcast(
            &raw mut *(*((*loop_res_sync).cond_).as_mut_ptr().offset(plane as isize))
                .offset(r as isize),
        );
        pthread_mutex_unlock(
            &raw mut *(*((*loop_res_sync).mutex_).as_mut_ptr().offset(plane as isize))
                .offset(r as isize),
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_alloc(
    mut lr_sync: *mut AV1LrSync,
    mut cm: *mut AV1_COMMON,
    mut num_workers: core::ffi::c_int,
    mut num_rows_lr: core::ffi::c_int,
    mut num_planes: core::ffi::c_int,
    mut width: core::ffi::c_int,
) {
    (*lr_sync).rows = num_rows_lr;
    (*lr_sync).num_planes = num_planes;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    j = 0 as core::ffi::c_int;
    while j < num_planes {
        (*lr_sync).mutex_[j as usize] = aom_malloc(
            (::core::mem::size_of::<pthread_mutex_t>() as size_t)
                .wrapping_mul(num_rows_lr as size_t),
        ) as *mut pthread_mutex_t;
        if ((*lr_sync).mutex_[j as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate lr_sync->mutex_[j]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if !((*lr_sync).mutex_[j as usize]).is_null() {
            i = 0 as core::ffi::c_int;
            while i < num_rows_lr {
                pthread_mutex_init(
                    &raw mut *(*((*lr_sync).mutex_).as_mut_ptr().offset(j as isize))
                        .offset(i as isize),
                    0 as *const pthread_mutexattr_t,
                );
                i += 1;
            }
        }
        (*lr_sync).cond_[j as usize] = aom_malloc(
            (::core::mem::size_of::<pthread_cond_t>() as size_t)
                .wrapping_mul(num_rows_lr as size_t),
        ) as *mut pthread_cond_t;
        if ((*lr_sync).cond_[j as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate lr_sync->cond_[j]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if !((*lr_sync).cond_[j as usize]).is_null() {
            i = 0 as core::ffi::c_int;
            while i < num_rows_lr {
                pthread_cond_init(
                    &raw mut *(*((*lr_sync).cond_).as_mut_ptr().offset(j as isize))
                        .offset(i as isize),
                    0 as *const pthread_condattr_t,
                );
                i += 1;
            }
        }
        j += 1;
    }
    (*lr_sync).job_mutex = aom_malloc(
        ::core::mem::size_of::<pthread_mutex_t>() as size_t,
    ) as *mut pthread_mutex_t;
    if ((*lr_sync).job_mutex).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate lr_sync->job_mutex\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if !((*lr_sync).job_mutex).is_null() {
        pthread_mutex_init((*lr_sync).job_mutex, 0 as *const pthread_mutexattr_t);
    }
    (*lr_sync).lrworkerdata = aom_calloc(
        num_workers as size_t,
        ::core::mem::size_of::<LRWorkerData>() as size_t,
    ) as *mut LRWorkerData;
    if ((*lr_sync).lrworkerdata).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate lr_sync->lrworkerdata\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*lr_sync).num_workers = num_workers;
    let mut worker_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while worker_idx < num_workers {
        if worker_idx < num_workers - 1 as core::ffi::c_int {
            let fresh7 = &raw mut (*((*lr_sync).lrworkerdata).offset(worker_idx as isize))
                .rst_tmpbuf;
            *fresh7 = aom_memalign(
                16 as size_t,
                (((256 as core::ffi::c_int * 3 as core::ffi::c_int
                    / 2 as core::ffi::c_int
                    + 2 as core::ffi::c_int * 3 as core::ffi::c_int
                    + 16 as core::ffi::c_int)
                    * (256 as core::ffi::c_int * 3 as core::ffi::c_int
                        / 2 as core::ffi::c_int
                        + 2 as core::ffi::c_int * 3 as core::ffi::c_int
                        + 8 as core::ffi::c_int) * 2 as core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
            ) as *mut int32_t;
            if ((*((*lr_sync).lrworkerdata).offset(worker_idx as isize)).rst_tmpbuf)
                .is_null()
            {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate lr_sync->lrworkerdata[worker_idx].rst_tmpbuf\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            let fresh8 = &raw mut (*((*lr_sync).lrworkerdata).offset(worker_idx as isize))
                .rlbs;
            *fresh8 = aom_malloc(
                ::core::mem::size_of::<RestorationLineBuffers>() as size_t,
            );
            if ((*((*lr_sync).lrworkerdata).offset(worker_idx as isize)).rlbs).is_null()
            {
                aom_internal_error(
                    (*cm).error,
                    AOM_CODEC_MEM_ERROR,
                    b"Failed to allocate lr_sync->lrworkerdata[worker_idx].rlbs\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
        } else {
            let fresh9 = &raw mut (*((*lr_sync).lrworkerdata).offset(worker_idx as isize))
                .rst_tmpbuf;
            *fresh9 = (*cm).rst_tmpbuf;
            let fresh10 = &raw mut (*((*lr_sync).lrworkerdata)
                .offset(worker_idx as isize))
                .rlbs;
            *fresh10 = (*cm).rlbs as *mut core::ffi::c_void;
        }
        worker_idx += 1;
    }
    let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while j_0 < num_planes {
        (*lr_sync).cur_sb_col[j_0 as usize] = aom_malloc(
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(num_rows_lr as size_t),
        ) as *mut core::ffi::c_int;
        if ((*lr_sync).cur_sb_col[j_0 as usize]).is_null() {
            aom_internal_error(
                (*cm).error,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate lr_sync->cur_sb_col[j]\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        j_0 += 1;
    }
    (*lr_sync).job_queue = aom_malloc(
        (::core::mem::size_of::<AV1LrMTInfo>() as size_t)
            .wrapping_mul(num_rows_lr as size_t)
            .wrapping_mul(num_planes as size_t),
    ) as *mut AV1LrMTInfo;
    if ((*lr_sync).job_queue).is_null() {
        aom_internal_error(
            (*cm).error,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate lr_sync->job_queue\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*lr_sync).sync_range = get_lr_sync_range(width);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_dealloc(mut lr_sync: *mut AV1LrSync) {
    if !lr_sync.is_null() {
        let mut j: core::ffi::c_int = 0;
        let mut i: core::ffi::c_int = 0;
        j = 0 as core::ffi::c_int;
        while j < MAX_MB_PLANE {
            if !((*lr_sync).mutex_[j as usize]).is_null() {
                i = 0 as core::ffi::c_int;
                while i < (*lr_sync).rows {
                    pthread_mutex_destroy(
                        &raw mut *(*((*lr_sync).mutex_).as_mut_ptr().offset(j as isize))
                            .offset(i as isize),
                    );
                    i += 1;
                }
                aom_free((*lr_sync).mutex_[j as usize] as *mut core::ffi::c_void);
            }
            if !((*lr_sync).cond_[j as usize]).is_null() {
                i = 0 as core::ffi::c_int;
                while i < (*lr_sync).rows {
                    pthread_cond_destroy(
                        &raw mut *(*((*lr_sync).cond_).as_mut_ptr().offset(j as isize))
                            .offset(i as isize),
                    );
                    i += 1;
                }
                aom_free((*lr_sync).cond_[j as usize] as *mut core::ffi::c_void);
            }
            j += 1;
        }
        if !((*lr_sync).job_mutex).is_null() {
            pthread_mutex_destroy((*lr_sync).job_mutex);
            aom_free((*lr_sync).job_mutex as *mut core::ffi::c_void);
        }
        j = 0 as core::ffi::c_int;
        while j < MAX_MB_PLANE {
            aom_free((*lr_sync).cur_sb_col[j as usize] as *mut core::ffi::c_void);
            j += 1;
        }
        aom_free((*lr_sync).job_queue as *mut core::ffi::c_void);
        if !((*lr_sync).lrworkerdata).is_null() {
            let mut worker_idx: core::ffi::c_int = 0 as core::ffi::c_int;
            while worker_idx < (*lr_sync).num_workers - 1 as core::ffi::c_int {
                let workerdata_data: *mut LRWorkerData = ((*lr_sync).lrworkerdata)
                    .offset(worker_idx as isize);
                aom_free((*workerdata_data).rst_tmpbuf as *mut core::ffi::c_void);
                aom_free((*workerdata_data).rlbs);
                worker_idx += 1;
            }
            aom_free((*lr_sync).lrworkerdata as *mut core::ffi::c_void);
        }
        memset(
            lr_sync as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<AV1LrSync>() as size_t,
        );
    }
}
unsafe extern "C-unwind" fn enqueue_lr_jobs(
    mut lr_sync: *mut AV1LrSync,
    mut lr_ctxt: *mut AV1LrStruct,
    mut cm: *mut AV1_COMMON,
) {
    let mut ctxt: *mut FilterFrameCtxt = ((*lr_ctxt).ctxt).as_mut_ptr();
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut lr_job_queue: *mut AV1LrMTInfo = (*lr_sync).job_queue;
    let mut lr_job_counter: [int32_t; 2] = [0; 2];
    let mut num_even_lr_jobs: int32_t = 0 as int32_t;
    (*lr_sync).jobs_enqueued = 0 as core::ffi::c_int;
    (*lr_sync).jobs_dequeued = 0 as core::ffi::c_int;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if !((*cm).rst_info[plane as usize].frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            num_even_lr_jobs = num_even_lr_jobs
                + ((*(*ctxt.offset(plane as isize)).rsi).vert_units as int32_t
                    + 1 as int32_t >> 1 as core::ffi::c_int);
        }
        plane += 1;
    }
    lr_job_counter[0 as core::ffi::c_int as usize] = 0 as core::ffi::c_int as int32_t;
    lr_job_counter[1 as core::ffi::c_int as usize] = num_even_lr_jobs;
    let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane_0 < num_planes {
        if !((*cm).rst_info[plane_0 as usize].frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            let is_uv: core::ffi::c_int = (plane_0 > 0 as core::ffi::c_int)
                as core::ffi::c_int;
            let ss_y: core::ffi::c_int = (is_uv != 0
                && (*(*cm).seq_params).subsampling_y != 0) as core::ffi::c_int;
            let unit_size: core::ffi::c_int = (*(*ctxt.offset(plane_0 as isize)).rsi)
                .restoration_unit_size;
            let plane_h: core::ffi::c_int = (*ctxt.offset(plane_0 as isize)).plane_h;
            let ext_size: core::ffi::c_int = unit_size * 3 as core::ffi::c_int
                / 2 as core::ffi::c_int;
            let mut y0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while y0 < plane_h {
                let mut remaining_h: core::ffi::c_int = plane_h - y0;
                let mut h: core::ffi::c_int = if remaining_h < ext_size {
                    remaining_h
                } else {
                    unit_size
                };
                let mut limits: RestorationTileLimits = RestorationTileLimits {
                    h_start: 0,
                    h_end: 0,
                    v_start: 0,
                    v_end: 0,
                };
                limits.v_start = y0;
                limits.v_end = y0 + h;
                let voffset: core::ffi::c_int = RESTORATION_UNIT_OFFSET >> ss_y;
                limits.v_start = if 0 as core::ffi::c_int > limits.v_start - voffset {
                    0 as core::ffi::c_int
                } else {
                    limits.v_start - voffset
                };
                if limits.v_end < plane_h {
                    limits.v_end -= voffset;
                }
                (*lr_job_queue
                    .offset(
                        lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                    ))
                    .lr_unit_row = i;
                (*lr_job_queue
                    .offset(
                        lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                    ))
                    .plane = plane_0;
                (*lr_job_queue
                    .offset(
                        lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                    ))
                    .v_start = limits.v_start;
                (*lr_job_queue
                    .offset(
                        lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                    ))
                    .v_end = limits.v_end;
                (*lr_job_queue
                    .offset(
                        lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                    ))
                    .sync_mode = i & 1 as core::ffi::c_int;
                if i & 1 as core::ffi::c_int == 0 as core::ffi::c_int {
                    (*lr_job_queue
                        .offset(
                            lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                        ))
                        .v_copy_start = limits.v_start + RESTORATION_BORDER;
                    (*lr_job_queue
                        .offset(
                            lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                        ))
                        .v_copy_end = limits.v_end - RESTORATION_BORDER;
                    if i == 0 as core::ffi::c_int {
                        (*lr_job_queue
                            .offset(
                                lr_job_counter[(i & 1 as core::ffi::c_int) as usize]
                                    as isize,
                            ))
                            .v_copy_start = 0 as core::ffi::c_int;
                    }
                    if i
                        == (*(*ctxt.offset(plane_0 as isize)).rsi).vert_units
                            - 1 as core::ffi::c_int
                    {
                        (*lr_job_queue
                            .offset(
                                lr_job_counter[(i & 1 as core::ffi::c_int) as usize]
                                    as isize,
                            ))
                            .v_copy_end = plane_h;
                    }
                } else {
                    (*lr_job_queue
                        .offset(
                            lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                        ))
                        .v_copy_start = if limits.v_start - 3 as core::ffi::c_int
                        > 0 as core::ffi::c_int
                    {
                        limits.v_start - 3 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    };
                    (*lr_job_queue
                        .offset(
                            lr_job_counter[(i & 1 as core::ffi::c_int) as usize] as isize,
                        ))
                        .v_copy_end = if (limits.v_end + 3 as core::ffi::c_int) < plane_h
                    {
                        limits.v_end + 3 as core::ffi::c_int
                    } else {
                        plane_h
                    };
                }
                lr_job_counter[(i & 1 as core::ffi::c_int) as usize] += 1;
                (*lr_sync).jobs_enqueued += 1;
                y0 += h;
                i += 1;
            }
        }
        plane_0 += 1;
    }
}
unsafe extern "C-unwind" fn get_lr_job_info(mut lr_sync: *mut AV1LrSync) -> *mut AV1LrMTInfo {
    let mut cur_job_info: *mut AV1LrMTInfo = 0 as *mut AV1LrMTInfo;
    pthread_mutex_lock((*lr_sync).job_mutex);
    if !(*lr_sync).lr_mt_exit && (*lr_sync).jobs_dequeued < (*lr_sync).jobs_enqueued {
        cur_job_info = ((*lr_sync).job_queue).offset((*lr_sync).jobs_dequeued as isize);
        (*lr_sync).jobs_dequeued += 1;
    }
    pthread_mutex_unlock((*lr_sync).job_mutex);
    return cur_job_info;
}
unsafe extern "C-unwind" fn set_loop_restoration_done(
    lr_sync: *mut AV1LrSync,
    ctxt: *mut FilterFrameCtxt,
) {
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < MAX_MB_PLANE {
        if !((*(*ctxt.offset(plane as isize)).rsi).frame_restoration_type
            as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            let mut y0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut row_number: core::ffi::c_int = 0 as core::ffi::c_int;
            let unit_size: core::ffi::c_int = (*(*ctxt.offset(plane as isize)).rsi)
                .restoration_unit_size;
            let plane_h: core::ffi::c_int = (*ctxt.offset(plane as isize)).plane_h;
            let ext_size: core::ffi::c_int = unit_size * 3 as core::ffi::c_int
                / 2 as core::ffi::c_int;
            let hnum_rest_units: core::ffi::c_int = (*(*ctxt.offset(plane as isize)).rsi)
                .horz_units;
            while y0 < plane_h {
                let remaining_h: core::ffi::c_int = plane_h - y0;
                let h: core::ffi::c_int = if remaining_h < ext_size {
                    remaining_h
                } else {
                    unit_size
                };
                lr_sync_write(
                    lr_sync as *mut core::ffi::c_void,
                    row_number,
                    hnum_rest_units - 1 as core::ffi::c_int,
                    hnum_rest_units,
                    plane,
                );
                y0 += h;
                row_number += 1;
            }
        }
        plane += 1;
    }
}
unsafe extern "C-unwind" fn loop_restoration_row_worker(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let lr_sync: *mut AV1LrSync = arg1 as *mut AV1LrSync;
    let mut lrworkerdata: *mut LRWorkerData = arg2 as *mut LRWorkerData;
    let mut lr_ctxt: *mut AV1LrStruct = (*lrworkerdata).lr_ctxt as *mut AV1LrStruct;
    let mut ctxt: *mut FilterFrameCtxt = ((*lr_ctxt).ctxt).as_mut_ptr();
    let mut lr_unit_row: core::ffi::c_int = 0;
    let mut plane: core::ffi::c_int = 0;
    let mut plane_w: core::ffi::c_int = 0;
    let mut job_mutex_: *mut pthread_mutex_t = (*lr_sync).job_mutex;
    let error_info: *mut aom_internal_error_info = &raw mut (*lrworkerdata).error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    static mut copy_funs: [copy_fun; 3] = unsafe {
        [
            Some(
                aom_yv12_partial_coloc_copy_y_c
                    as unsafe extern "C-unwind" fn(
                        *const yv12_buffer_config,
                        *mut yv12_buffer_config,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                aom_yv12_partial_coloc_copy_u_c
                    as unsafe extern "C-unwind" fn(
                        *const yv12_buffer_config,
                        *mut yv12_buffer_config,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                aom_yv12_partial_coloc_copy_v_c
                    as unsafe extern "C-unwind" fn(
                        *const yv12_buffer_config,
                        *mut yv12_buffer_config,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
        ]
    };
    loop {
        let mut cur_job_info: *mut AV1LrMTInfo = get_lr_job_info(lr_sync);
        if cur_job_info.is_null() {
            break;
        }
        let mut limits: RestorationTileLimits = RestorationTileLimits {
            h_start: 0,
            h_end: 0,
            v_start: 0,
            v_end: 0,
        };
        let mut on_sync_read: sync_read_fn_t = None;
        let mut on_sync_write: sync_write_fn_t = None;
        limits.v_start = (*cur_job_info).v_start;
        limits.v_end = (*cur_job_info).v_end;
        lr_unit_row = (*cur_job_info).lr_unit_row;
        plane = (*cur_job_info).plane;
        plane_w = (*ctxt.offset(plane as isize)).plane_w;
        on_sync_read = (if (*cur_job_info).sync_mode == 1 as core::ffi::c_int {
            Some(
                lr_sync_read
                    as unsafe extern "C-unwind" fn(
                        *mut core::ffi::c_void,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            )
        } else {
            Some(
                av1_lr_sync_read_dummy
                    as unsafe extern "C-unwind" fn(
                        *mut core::ffi::c_void,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            )
        }) as sync_read_fn_t;
        on_sync_write = (if (*cur_job_info).sync_mode == 0 as core::ffi::c_int {
            Some(
                lr_sync_write
                    as unsafe extern "C-unwind" fn(
                        *mut core::ffi::c_void,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            )
        } else {
            Some(
                av1_lr_sync_write_dummy
                    as unsafe extern "C-unwind" fn(
                        *mut core::ffi::c_void,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            )
        }) as sync_write_fn_t;
        av1_foreach_rest_unit_in_row(
            &raw mut limits,
            plane_w,
            (*lr_ctxt).on_rest_unit,
            lr_unit_row,
            (*(*ctxt.offset(plane as isize)).rsi).restoration_unit_size,
            (*(*ctxt.offset(plane as isize)).rsi).horz_units,
            (*(*ctxt.offset(plane as isize)).rsi).vert_units,
            plane,
            &raw mut *ctxt.offset(plane as isize) as *mut FilterFrameCtxt
                as *mut core::ffi::c_void,
            (*lrworkerdata).rst_tmpbuf,
            (*lrworkerdata).rlbs as *mut RestorationLineBuffers,
            on_sync_read,
            on_sync_write,
            lr_sync as *mut AV1LrSyncData,
            error_info,
        );
        (copy_funs[plane as usize])
            .expect(
                "non-null function pointer",
            )(
            (*lr_ctxt).dst,
            (*lr_ctxt).frame,
            0 as core::ffi::c_int,
            plane_w,
            (*cur_job_info).v_copy_start,
            (*cur_job_info).v_copy_end,
        );
        if (*lrworkerdata).do_extend_border != 0 {
            aom_extend_frame_borders_plane_row_c(
                (*lr_ctxt).frame,
                plane,
                (*cur_job_info).v_copy_start,
                (*cur_job_info).v_copy_end,
            );
        }
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(job_mutex_);
        (*lr_sync).lr_mt_exit = true_0 != 0;
        pthread_mutex_unlock(job_mutex_);
        set_loop_restoration_done(lr_sync, ((*lr_ctxt).ctxt).as_mut_ptr());
        return 0 as core::ffi::c_int;}
}
}}
#[inline]
unsafe extern "C-unwind" fn sync_lr_workers(
    workers: *mut AVxWorker,
    cm: *mut AV1_COMMON,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut had_error: core::ffi::c_int = (*workers
        .offset(0 as core::ffi::c_int as isize))
        .had_error;
    let mut error_info: aom_internal_error_info = aom_internal_error_info {
        error_code: AOM_CODEC_OK,
        has_detail: 0,
        detail: [0; 200],
        setjmp: 0,
        jmp: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    if had_error != 0 {
        let worker: *mut AVxWorker = &raw mut *workers.offset(0 as core::ffi::c_int as isize)
            as *mut AVxWorker;
        error_info = (*((*worker).data2 as *mut LRWorkerData)).error_info;
    }
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i > 0 as core::ffi::c_int {
        let worker_0: *mut AVxWorker = &raw mut *workers.offset(i as isize)
            as *mut AVxWorker;
        if ((*winterface).sync).expect("non-null function pointer")(worker_0) == 0 {
            had_error = 1 as core::ffi::c_int;
            error_info = (*((*worker_0).data2 as *mut LRWorkerData)).error_info;
        }
        i -= 1;
    }
    if had_error != 0 {
        aom_internal_error_copy((*cm).error, &raw mut error_info);
    }
}
unsafe extern "C-unwind" fn foreach_rest_unit_in_planes_mt(
    mut lr_ctxt: *mut AV1LrStruct,
    mut workers: *mut AVxWorker,
    mut num_workers: core::ffi::c_int,
    mut lr_sync: *mut AV1LrSync,
    mut cm: *mut AV1_COMMON,
    mut do_extend_border: core::ffi::c_int,
) {
    let mut ctxt: *mut FilterFrameCtxt = ((*lr_ctxt).ctxt).as_mut_ptr();
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut num_rows_lr: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        if !((*cm).rst_info[plane as usize].frame_restoration_type as core::ffi::c_uint
            == RESTORE_NONE as core::ffi::c_int as core::ffi::c_uint)
        {
            let plane_h: core::ffi::c_int = (*ctxt.offset(plane as isize)).plane_h;
            let unit_size: core::ffi::c_int = (*cm)
                .rst_info[plane as usize]
                .restoration_unit_size;
            num_rows_lr = if num_rows_lr > av1_lr_count_units(unit_size, plane_h) {
                num_rows_lr
            } else {
                av1_lr_count_units(unit_size, plane_h)
            };
        }
        plane += 1;
    }
    let mut i: core::ffi::c_int = 0;
    if (*lr_sync).sync_range == 0 || num_rows_lr > (*lr_sync).rows
        || num_workers > (*lr_sync).num_workers || num_planes > (*lr_sync).num_planes
    {
        av1_loop_restoration_dealloc(lr_sync);
        av1_loop_restoration_alloc(
            lr_sync,
            cm,
            num_workers,
            num_rows_lr,
            num_planes,
            (*cm).width,
        );
    }
    (*lr_sync).lr_mt_exit = false_0 != 0;
    i = 0 as core::ffi::c_int;
    while i < num_planes {
        memset(
            (*lr_sync).cur_sb_col[i as usize] as *mut core::ffi::c_void,
            -(1 as core::ffi::c_int),
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(num_rows_lr as size_t),
        );
        i += 1;
    }
    enqueue_lr_jobs(lr_sync, lr_ctxt, cm);
    i = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *workers.offset(i as isize) as *mut AVxWorker;
        let fresh6 = &raw mut (*((*lr_sync).lrworkerdata).offset(i as isize)).lr_ctxt;
        *fresh6 = lr_ctxt as *mut core::ffi::c_void;
        (*((*lr_sync).lrworkerdata).offset(i as isize)).do_extend_border = do_extend_border;
        (*worker).hook = Some(
            loop_restoration_row_worker
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ) as AVxWorkerHook;
        (*worker).data1 = lr_sync as *mut core::ffi::c_void;
        (*worker).data2 = &raw mut *((*lr_sync).lrworkerdata).offset(i as isize)
            as *mut LRWorkerData as *mut core::ffi::c_void;
        (*worker).had_error = 0 as core::ffi::c_int;
        if i == 0 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        i -= 1;
    }
    sync_lr_workers(workers, cm, num_workers);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_loop_restoration_filter_frame_mt(
    mut frame: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1_COMMON,
    mut optimized_lr: core::ffi::c_int,
    mut workers: *mut AVxWorker,
    mut num_workers: core::ffi::c_int,
    mut lr_sync: *mut AV1LrSync,
    mut lr_ctxt: *mut core::ffi::c_void,
    mut do_extend_border: core::ffi::c_int,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut loop_rest_ctxt: *mut AV1LrStruct = lr_ctxt as *mut AV1LrStruct;
    av1_loop_restoration_filter_frame_init(
        loop_rest_ctxt,
        frame,
        cm as *mut AV1Common,
        optimized_lr,
        num_planes,
    );
    foreach_rest_unit_in_planes_mt(
        loop_rest_ctxt,
        workers,
        num_workers,
        lr_sync,
        cm,
        do_extend_border,
    );
}
#[inline]
unsafe extern "C-unwind" fn reset_cdef_job_info(cdef_sync: *mut AV1CdefSync) {
    (*cdef_sync).end_of_frame = 0 as core::ffi::c_int;
    (*cdef_sync).fbr = 0 as core::ffi::c_int;
    (*cdef_sync).fbc = 0 as core::ffi::c_int;
    (*cdef_sync).cdef_mt_exit = false_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn launch_cdef_workers(
    workers: *mut AVxWorker,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *workers.offset(i as isize) as *mut AVxWorker;
        (*worker).had_error = 0 as core::ffi::c_int;
        if i == 0 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        i -= 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn sync_cdef_workers(
    workers: *mut AVxWorker,
    cm: *mut AV1_COMMON,
    mut num_workers: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let mut had_error: core::ffi::c_int = (*workers
        .offset(0 as core::ffi::c_int as isize))
        .had_error;
    let mut error_info: aom_internal_error_info = aom_internal_error_info {
        error_code: AOM_CODEC_OK,
        has_detail: 0,
        detail: [0; 200],
        setjmp: 0,
        jmp: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
    };
    if had_error != 0 {
        let worker: *mut AVxWorker = &raw mut *workers.offset(0 as core::ffi::c_int as isize)
            as *mut AVxWorker;
        error_info = (*((*worker).data2 as *mut AV1CdefWorkerData)).error_info;
    }
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i > 0 as core::ffi::c_int {
        let worker_0: *mut AVxWorker = &raw mut *workers.offset(i as isize)
            as *mut AVxWorker;
        if ((*winterface).sync).expect("non-null function pointer")(worker_0) == 0 {
            had_error = 1 as core::ffi::c_int;
            error_info = (*((*worker_0).data2 as *mut AV1CdefWorkerData)).error_info;
        }
        i -= 1;
    }
    if had_error != 0 {
        aom_internal_error_copy((*cm).error, &raw mut error_info);
    }
}
unsafe extern "C-unwind" fn update_cdef_row_next_job_info(
    cdef_sync: *mut AV1CdefSync,
    nvfb: core::ffi::c_int,
) {
    (*cdef_sync).fbr += 1;
    if (*cdef_sync).fbr == nvfb {
        (*cdef_sync).end_of_frame = 1 as core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_cdef_row_next_job(
    cdef_sync: *mut AV1CdefSync,
    mut cur_fbr: *mut core::ffi::c_int,
    nvfb: core::ffi::c_int,
) -> core::ffi::c_int {
    pthread_mutex_lock((*cdef_sync).mutex_);
    let mut do_next_row: core::ffi::c_int = 0 as core::ffi::c_int;
    if !(*cdef_sync).cdef_mt_exit && (*cdef_sync).end_of_frame == 0 as core::ffi::c_int {
        do_next_row = 1 as core::ffi::c_int;
        ::core::ptr::write_volatile(cur_fbr, (*cdef_sync).fbr);
        update_cdef_row_next_job_info(cdef_sync, nvfb);
    }
    pthread_mutex_unlock((*cdef_sync).mutex_);
    return do_next_row;
}
unsafe extern "C-unwind" fn set_cdef_init_fb_row_done(
    cdef_sync: *mut AV1CdefSync,
    mut nvfb: core::ffi::c_int,
) {
    let mut fbr: core::ffi::c_int = 0 as core::ffi::c_int;
    while fbr < nvfb {
        cdef_row_mt_sync_write(cdef_sync, fbr);
        fbr += 1;
    }
}
unsafe extern "C-unwind" fn cdef_sb_row_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let cdef_sync: *mut AV1CdefSync = arg1 as *mut AV1CdefSync;
    let cdef_worker: *mut AV1CdefWorkerData = arg2 as *mut AV1CdefWorkerData;
    let mut cm: *mut AV1_COMMON = (*cdef_worker).cm;
    let nvfb: core::ffi::c_int = ((*cm).mi_params.mi_rows + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    let mut job_mutex_: *mut pthread_mutex_t = (*cdef_sync).mutex_;
    let error_info: *mut aom_internal_error_info = &raw mut (*cdef_worker).error_info;
    loop {
match crate::jump::value_scope((((*error_info).jmp).as_mut_ptr()).cast(), || {
    (*error_info).setjmp = 1 as core::ffi::c_int;
    let mut cur_fbr: core::ffi::c_int = 0;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    while get_cdef_row_next_job(cdef_sync, &raw mut cur_fbr, nvfb) != 0 {
        let mut xd: *mut MACROBLOCKD = (*cdef_worker).xd;
        av1_cdef_fb_row(
            cm,
            xd,
            ((*cdef_worker).linebuf).as_mut_ptr(),
            ((*cdef_worker).colbuf).as_mut_ptr(),
            (*cdef_worker).srcbuf,
            cur_fbr,
            (*cdef_worker).cdef_init_fb_row_fn,
            cdef_sync as *mut AV1CdefSyncData,
            error_info,
        );
        if (*cdef_worker).do_extend_border != 0 {
            let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
            while plane < num_planes {
                let mut ybf: *const YV12_BUFFER_CONFIG = &raw mut (*(*cm).cur_frame).buf;
                let is_uv: core::ffi::c_int = (plane > 0 as core::ffi::c_int)
                    as core::ffi::c_int;
                let mi_high: core::ffi::c_int = MI_SIZE_LOG2
                    - (*xd).plane[plane as usize].subsampling_y;
                let unit_height: core::ffi::c_int = MI_SIZE_64X64 << mi_high;
                let v_start: core::ffi::c_int = cur_fbr * unit_height;
                let v_end: core::ffi::c_int = if v_start + unit_height
                    < (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize]
                {
                    v_start + unit_height
                } else {
                    (*ybf).c2rust_unnamed_2.crop_heights[is_uv as usize]
                };
                aom_extend_frame_borders_plane_row_c(
                    ybf as *const yv12_buffer_config,
                    plane,
                    v_start,
                    v_end,
                );
                plane += 1;
            }
        }
    }
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return 1 as core::ffi::c_int;}) {
Some(value) => return value,
None => {
        (*error_info).setjmp = 0 as core::ffi::c_int;
        pthread_mutex_lock(job_mutex_);
        (*cdef_sync).cdef_mt_exit = true_0 != 0;
        pthread_mutex_unlock(job_mutex_);
        set_cdef_init_fb_row_done(cdef_sync, nvfb);
        return 0 as core::ffi::c_int;}
}
}}
unsafe extern "C-unwind" fn prepare_cdef_frame_workers(
    cm: *mut AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    cdef_worker: *mut AV1CdefWorkerData,
    mut hook: AVxWorkerHook,
    workers: *mut AVxWorker,
    cdef_sync: *mut AV1CdefSync,
    mut num_workers: core::ffi::c_int,
    mut cdef_init_fb_row_fn: cdef_init_fb_row_t,
    mut do_extend_border: core::ffi::c_int,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let fresh0 = &raw mut (*cdef_worker.offset(0 as core::ffi::c_int as isize)).srcbuf;
    *fresh0 = (*cm).cdef_info.srcbuf;
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let fresh1 = &raw mut (*cdef_worker.offset(0 as core::ffi::c_int as isize))
            .colbuf[plane as usize];
        *fresh1 = (*cm).cdef_info.colbuf[plane as usize];
        plane += 1;
    }
    let mut i: core::ffi::c_int = num_workers - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let worker: *mut AVxWorker = &raw mut *workers.offset(i as isize) as *mut AVxWorker;
        let fresh2 = &raw mut (*cdef_worker.offset(i as isize)).cm;
        *fresh2 = cm;
        let fresh3 = &raw mut (*cdef_worker.offset(i as isize)).xd;
        *fresh3 = xd;
        let fresh4 = &raw mut (*cdef_worker.offset(i as isize)).cdef_init_fb_row_fn;
        *fresh4 = cdef_init_fb_row_fn;
        (*cdef_worker.offset(i as isize)).do_extend_border = do_extend_border;
        let mut plane_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while plane_0 < num_planes {
            let fresh5 = &raw mut (*cdef_worker.offset(i as isize))
                .linebuf[plane_0 as usize];
            *fresh5 = (*cm).cdef_info.linebuf[plane_0 as usize];
            plane_0 += 1;
        }
        (*worker).hook = hook;
        (*worker).data1 = cdef_sync as *mut core::ffi::c_void;
        (*worker).data2 = &raw mut *cdef_worker.offset(i as isize) as *mut AV1CdefWorkerData
            as *mut core::ffi::c_void;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_init_fb_row_mt(
    cm: *const AV1_COMMON,
    xd: *const MACROBLOCKD,
    fb_info: *mut CdefBlockInfo,
    linebuf: *mut *mut uint16_t,
    src: *mut uint16_t,
    cdef_sync: *mut AV1CdefSyncData,
    mut fbr: core::ffi::c_int,
) {
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let nvfb: core::ffi::c_int = ((*cm).mi_params.mi_rows + MI_SIZE_64X64
        - 1 as core::ffi::c_int) / MI_SIZE_64X64;
    let luma_stride: core::ffi::c_int = ((*cm).mi_params.mi_cols
        << 2 as core::ffi::c_int)
        + (((1 as core::ffi::c_int) << 4 as core::ffi::c_int) - 1 as core::ffi::c_int)
        & !(((1 as core::ffi::c_int) << 4 as core::ffi::c_int) - 1 as core::ffi::c_int);
    (*fb_info).frame_boundary[TOP as core::ffi::c_int as usize] = if MI_SIZE_64X64 * fbr
        == 0 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    if fbr != nvfb - 1 as core::ffi::c_int {
        (*fb_info).frame_boundary[BOTTOM as core::ffi::c_int as usize] = if MI_SIZE_64X64
            * (fbr + 1 as core::ffi::c_int) == (*cm).mi_params.mi_rows
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
    } else {
        (*fb_info).frame_boundary[BOTTOM as core::ffi::c_int as usize] = 1
            as core::ffi::c_int;
    }
    (*fb_info).src = src;
    (*fb_info).damping = (*cm).cdef_info.cdef_damping;
    (*fb_info).coeff_shift = (if ((*(*cm).seq_params).bit_depth as core::ffi::c_uint)
        .wrapping_sub(8 as core::ffi::c_uint) > 0 as core::ffi::c_uint
    {
        ((*(*cm).seq_params).bit_depth as core::ffi::c_uint)
            .wrapping_sub(8 as core::ffi::c_uint)
    } else {
        0 as core::ffi::c_uint
    }) as core::ffi::c_int;
    memset(
        &raw mut (*fb_info).dir as *mut [[core::ffi::c_int; 16]; 16]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[core::ffi::c_int; 16]; 16]>() as size_t,
    );
    memset(
        &raw mut (*fb_info).var as *mut [[core::ffi::c_int; 16]; 16]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[core::ffi::c_int; 16]; 16]>() as size_t,
    );
    let mut plane: core::ffi::c_int = 0 as core::ffi::c_int;
    while plane < num_planes {
        let stride: core::ffi::c_int = luma_stride
            >> (*xd).plane[plane as usize].subsampling_x;
        let mut top_linebuf: *mut uint16_t = &raw mut *(*linebuf.offset(plane as isize))
            .offset(0 as core::ffi::c_int as isize) as *mut uint16_t;
        let mut bot_linebuf: *mut uint16_t = &raw mut *(*linebuf.offset(plane as isize))
            .offset((nvfb * CDEF_VBORDER * stride) as isize) as *mut uint16_t;
        let mi_high_l2: core::ffi::c_int = MI_SIZE_LOG2
            - (*xd).plane[plane as usize].subsampling_y;
        let top_offset: core::ffi::c_int = MI_SIZE_64X64 * (fbr + 1 as core::ffi::c_int)
            << mi_high_l2;
        let bot_offset: core::ffi::c_int = MI_SIZE_64X64 * (fbr + 1 as core::ffi::c_int)
            << mi_high_l2;
        if fbr != nvfb - 1 as core::ffi::c_int {
            av1_cdef_copy_sb8_16(
                cm,
                &raw mut *top_linebuf
                    .offset(
                        ((fbr + 1 as core::ffi::c_int) * CDEF_VBORDER * stride) as isize,
                    ),
                stride,
                (*xd).plane[plane as usize].dst.buf,
                top_offset - CDEF_VBORDER,
                0 as core::ffi::c_int,
                (*xd).plane[plane as usize].dst.stride,
                CDEF_VBORDER,
                stride,
            );
        }
        if fbr != nvfb - 1 as core::ffi::c_int {
            av1_cdef_copy_sb8_16(
                cm,
                &raw mut *bot_linebuf.offset((fbr * CDEF_VBORDER * stride) as isize),
                stride,
                (*xd).plane[plane as usize].dst.buf,
                bot_offset,
                0 as core::ffi::c_int,
                (*xd).plane[plane as usize].dst.stride,
                CDEF_VBORDER,
                stride,
            );
        }
        (*fb_info).top_linebuf[plane as usize] = &raw mut *(*linebuf.offset(plane as isize))
            .offset((fbr * CDEF_VBORDER * stride) as isize) as *mut uint16_t;
        (*fb_info).bot_linebuf[plane as usize] = &raw mut *(*linebuf.offset(plane as isize))
            .offset(
                (nvfb * CDEF_VBORDER * stride + fbr * CDEF_VBORDER * stride) as isize,
            ) as *mut uint16_t;
        plane += 1;
    }
    cdef_row_mt_sync_write(cdef_sync as *mut AV1CdefSync, fbr);
    cdef_row_mt_sync_read(cdef_sync as *mut AV1CdefSync, fbr);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_frame_mt(
    cm: *mut AV1_COMMON,
    xd: *mut MACROBLOCKD,
    cdef_worker: *mut AV1CdefWorkerData,
    workers: *mut AVxWorker,
    cdef_sync: *mut AV1CdefSync,
    mut num_workers: core::ffi::c_int,
    mut cdef_init_fb_row_fn: cdef_init_fb_row_t,
    mut do_extend_border: core::ffi::c_int,
) {
    let mut frame: *mut YV12_BUFFER_CONFIG = &raw mut (*(*cm).cur_frame).buf;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    av1_setup_dst_planes(
        ((*xd).plane).as_mut_ptr(),
        (*(*cm).seq_params).sb_size,
        frame,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        num_planes,
    );
    reset_cdef_job_info(cdef_sync);
    prepare_cdef_frame_workers(
        cm,
        xd,
        cdef_worker,
        Some(
            cdef_sb_row_worker_hook
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
        workers,
        cdef_sync,
        num_workers,
        cdef_init_fb_row_fn,
        do_extend_border,
    );
    launch_cdef_workers(workers, num_workers);
    sync_cdef_workers(workers, cm, num_workers);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_intrabc_extra_top_right_sb_delay(
    mut cm: *const AV1_COMMON,
) -> core::ffi::c_int {
    if av1_allow_intrabc(cm) == 0 {
        return 0 as core::ffi::c_int;
    }
    return if (*(*cm).seq_params).sb_size as core::ffi::c_int
        == BLOCK_128X128 as core::ffi::c_int
    {
        2 as core::ffi::c_int
    } else {
        4 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_allow_intrabc(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return (frame_is_intra_only(cm) != 0
        && (*cm).features.allow_screen_content_tools as core::ffi::c_int != 0
        && (*cm).features.allow_intrabc as core::ffi::c_int != 0) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn frame_is_intra_only(cm: *const AV1_COMMON) -> core::ffi::c_int {
    return ((*cm).current_frame.frame_type as core::ffi::c_int
        == KEY_FRAME as core::ffi::c_int
        || (*cm).current_frame.frame_type as core::ffi::c_int
            == INTRA_ONLY_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
#[inline(always)]
unsafe extern "C-unwind" fn skip_loop_filter_plane(
    mut planes_to_lf: *const core::ffi::c_int,
    mut plane: core::ffi::c_int,
    mut lpf_opt_level: core::ffi::c_int,
) -> bool {
    if lpf_opt_level == 2 as core::ffi::c_int {
        if plane == AOM_PLANE_Y {
            return *planes_to_lf.offset(plane as isize) == 0;
        }
        if plane == AOM_PLANE_U {
            return *planes_to_lf.offset(1 as core::ffi::c_int as isize) == 0
                && *planes_to_lf.offset(2 as core::ffi::c_int as isize) == 0;
        }
        if plane == AOM_PLANE_V {
            return true_0 != 0;
        }
    }
    return *planes_to_lf.offset(plane as isize) == 0;
}
#[inline]
unsafe extern "C-unwind" fn enqueue_lf_jobs(
    mut lf_sync: *mut AV1LfSync,
    mut start: core::ffi::c_int,
    mut stop: core::ffi::c_int,
    mut planes_to_lf: *const core::ffi::c_int,
    mut lpf_opt_level: core::ffi::c_int,
    mut num_mis_in_lpf_unit_height: core::ffi::c_int,
) {
    let mut mi_row: core::ffi::c_int = 0;
    let mut plane: core::ffi::c_int = 0;
    let mut dir: core::ffi::c_int = 0;
    let mut lf_job_queue: *mut AV1LfMTInfo = (*lf_sync).job_queue;
    (*lf_sync).jobs_enqueued = 0 as core::ffi::c_int;
    (*lf_sync).jobs_dequeued = 0 as core::ffi::c_int;
    dir = 0 as core::ffi::c_int;
    while dir < 2 as core::ffi::c_int {
        mi_row = start;
        while mi_row < stop {
            plane = 0 as core::ffi::c_int;
            while plane < MAX_MB_PLANE {
                if !skip_loop_filter_plane(planes_to_lf, plane, lpf_opt_level) {
                    if !(*planes_to_lf.offset(plane as isize) == 0) {
                        (*lf_job_queue).mi_row = mi_row;
                        (*lf_job_queue).plane = plane;
                        (*lf_job_queue).dir = dir;
                        (*lf_job_queue).lpf_opt_level = lpf_opt_level;
                        lf_job_queue = lf_job_queue.offset(1);
                        (*lf_sync).jobs_enqueued += 1;
                    }
                }
                plane += 1;
            }
            mi_row += num_mis_in_lpf_unit_height;
        }
        dir += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn loop_filter_frame_mt_init(
    mut cm: *mut AV1_COMMON,
    mut start_mi_row: core::ffi::c_int,
    mut end_mi_row: core::ffi::c_int,
    mut planes_to_lf: *const core::ffi::c_int,
    mut num_workers: core::ffi::c_int,
    mut lf_sync: *mut AV1LfSync,
    mut lpf_opt_level: core::ffi::c_int,
    mut num_mis_in_lpf_unit_height_log2: core::ffi::c_int,
) {
    let sb_rows: core::ffi::c_int = (*cm).mi_params.mi_rows
        + ((1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2)
        - 1 as core::ffi::c_int >> num_mis_in_lpf_unit_height_log2;
    if (*lf_sync).sync_range == 0 || sb_rows != (*lf_sync).rows
        || num_workers > (*lf_sync).num_workers
    {
        av1_loop_filter_dealloc(lf_sync);
        av1_loop_filter_alloc(lf_sync, cm, sb_rows, (*cm).width, num_workers);
    }
    (*lf_sync).lf_mt_exit = false_0 != 0;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAX_MB_PLANE {
        memset(
            (*lf_sync).cur_sb_col[i as usize] as *mut core::ffi::c_void,
            -(1 as core::ffi::c_int),
            (::core::mem::size_of::<core::ffi::c_int>() as size_t)
                .wrapping_mul(sb_rows as size_t),
        );
        i += 1;
    }
    enqueue_lf_jobs(
        lf_sync,
        start_mi_row,
        end_mi_row,
        planes_to_lf,
        lpf_opt_level,
        (1 as core::ffi::c_int) << num_mis_in_lpf_unit_height_log2,
    );
}
#[inline]
unsafe extern "C-unwind" fn get_lf_job_info(mut lf_sync: *mut AV1LfSync) -> *mut AV1LfMTInfo {
    let mut cur_job_info: *mut AV1LfMTInfo = 0 as *mut AV1LfMTInfo;
    pthread_mutex_lock((*lf_sync).job_mutex);
    if !(*lf_sync).lf_mt_exit && (*lf_sync).jobs_dequeued < (*lf_sync).jobs_enqueued {
        cur_job_info = ((*lf_sync).job_queue).offset((*lf_sync).jobs_dequeued as isize);
        (*lf_sync).jobs_dequeued += 1;
    }
    pthread_mutex_unlock((*lf_sync).job_mutex);
    return cur_job_info;
}
#[inline]
unsafe extern "C-unwind" fn loop_filter_data_reset(
    mut lf_data: *mut LFWorkerData,
    mut frame_buffer: *mut YV12_BUFFER_CONFIG,
    mut cm: *mut AV1Common,
    mut xd: *mut MACROBLOCKD,
) {
    let mut pd: *mut macroblockd_plane = ((*xd).plane).as_mut_ptr();
    (*lf_data).frame_buffer = frame_buffer;
    (*lf_data).cm = cm;
    (*lf_data).xd = xd;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAX_MB_PLANE {
        (*lf_data).planes[i as usize].dst = (*pd.offset(i as isize)).dst;
        (*lf_data).planes[i as usize].subsampling_x = (*pd.offset(i as isize))
            .subsampling_x;
        (*lf_data).planes[i as usize].subsampling_y = (*pd.offset(i as isize))
            .subsampling_y;
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn set_planes_to_loop_filter(
    mut lf: *const loopfilter,
    mut planes_to_lf: *mut core::ffi::c_int,
    mut plane_start: core::ffi::c_int,
    mut plane_end: core::ffi::c_int,
) {
    *planes_to_lf.offset(0 as core::ffi::c_int as isize) = (((*lf)
        .filter_level[0 as core::ffi::c_int as usize] != 0
        || (*lf).filter_level[1 as core::ffi::c_int as usize] != 0)
        && plane_start <= 0 as core::ffi::c_int && (0 as core::ffi::c_int) < plane_end)
        as core::ffi::c_int;
    *planes_to_lf.offset(1 as core::ffi::c_int as isize) = ((*lf).filter_level_u != 0
        && plane_start <= 1 as core::ffi::c_int && (1 as core::ffi::c_int) < plane_end)
        as core::ffi::c_int;
    *planes_to_lf.offset(2 as core::ffi::c_int as isize) = ((*lf).filter_level_v != 0
        && plane_start <= 2 as core::ffi::c_int && (2 as core::ffi::c_int) < plane_end)
        as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn check_planes_to_loop_filter(
    mut lf: *const loopfilter,
    mut planes_to_lf: *mut core::ffi::c_int,
    mut plane_start: core::ffi::c_int,
    mut plane_end: core::ffi::c_int,
) -> core::ffi::c_int {
    set_planes_to_loop_filter(lf, planes_to_lf, plane_start, plane_end);
    if *planes_to_lf.offset(0 as core::ffi::c_int as isize) == 0
        && plane_start <= 0 as core::ffi::c_int && (0 as core::ffi::c_int) < plane_end
    {
        return 0 as core::ffi::c_int;
    }
    if *planes_to_lf.offset(0 as core::ffi::c_int as isize) == 0
        && *planes_to_lf.offset(1 as core::ffi::c_int as isize) == 0
        && *planes_to_lf.offset(2 as core::ffi::c_int as isize) == 0
    {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}

extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> core::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
}
