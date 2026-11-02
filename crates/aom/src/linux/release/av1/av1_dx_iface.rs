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

// Mechanically translated from upstream/aom/av1/av1_dx_iface.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    fn aom_uleb_decode(
        buffer: *const uint8_t,
        available: size_t,
        value: *mut uint64_t,
        length: *mut size_t,
    ) -> core::ffi::c_int;
    fn aom_img_free(img: *mut aom_image_t);
    fn aom_img_remove_metadata(img: *mut aom_image_t);
    fn aom_img_alloc_with_cb(
        img: *mut aom_image_t,
        fmt: aom_img_fmt_t,
        d_w: core::ffi::c_uint,
        d_h: core::ffi::c_uint,
        align: core::ffi::c_uint,
        alloc_cb: aom_alloc_img_data_cb_fn_t,
        cb_priv: *mut core::ffi::c_void,
    ) -> *mut aom_image_t;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_rb_read_bit(rb: *mut aom_read_bit_buffer) -> core::ffi::c_int;
    fn aom_rb_read_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_rb_read_uvlc(rb: *mut aom_read_bit_buffer) -> uint32_t;
    fn aom_rb_read_unsigned_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> uint32_t;
    fn av1_free_cdef_sync(cdef_sync: *mut AV1CdefSync);
    fn av1_alloc_internal_frame_buffers(
        list: *mut InternalFrameBufferList,
    ) -> core::ffi::c_int;
    fn av1_free_internal_frame_buffers(list: *mut InternalFrameBufferList);
    fn av1_get_frame_buffer(
        cb_priv: *mut core::ffi::c_void,
        min_size: size_t,
        fb: *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int;
    fn av1_release_frame_buffer(
        cb_priv: *mut core::ffi::c_void,
        fb: *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int;
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_read_obu_header_and_size(
        data: *const uint8_t,
        bytes_available: size_t,
        is_annexb: core::ffi::c_int,
        obu_header: *mut ObuHeader,
        payload_size: *mut size_t,
        bytes_read: *mut size_t,
    ) -> aom_codec_err_t;
    fn av1_get_uniform_tile_size(
        cm: *const AV1Common,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> bool;
    fn av1_receive_compressed_data(
        pbi: *mut AV1Decoder,
        size: size_t,
        psource: *mut *const uint8_t,
    ) -> core::ffi::c_int;
    fn av1_get_raw_frame(
        pbi: *mut AV1Decoder,
        index: size_t,
        sd: *mut *mut YV12_BUFFER_CONFIG,
        grain_params: *mut *mut aom_film_grain_t,
    ) -> core::ffi::c_int;
    fn av1_get_frame_to_show(
        pbi: *mut AV1Decoder,
        frame: *mut YV12_BUFFER_CONFIG,
    ) -> core::ffi::c_int;
    fn av1_copy_reference_dec(
        pbi: *mut AV1Decoder,
        idx: core::ffi::c_int,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> aom_codec_err_t;
    fn av1_set_reference_dec(
        cm: *mut AV1_COMMON,
        idx: core::ffi::c_int,
        use_external_ref: core::ffi::c_int,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> aom_codec_err_t;
    fn av1_copy_new_frame_dec(
        cm: *mut AV1_COMMON,
        new_frame: *mut YV12_BUFFER_CONFIG,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> aom_codec_err_t;
    fn av1_decoder_create(pool: *mut BufferPool) -> *mut AV1Decoder;
    fn av1_decoder_remove(pbi: *mut AV1Decoder);
    fn av1_read_profile(rb: *mut aom_read_bit_buffer) -> BITSTREAM_PROFILE;
    fn av1_add_film_grain(
        grain_params: *const aom_film_grain_t,
        src: *const aom_image_t,
        dst: *mut aom_image_t,
    ) -> core::ffi::c_int;
    fn aom_get_num_layers_from_operating_point_idc(
        operating_point_idc: core::ffi::c_int,
        number_spatial_layers: *mut core::ffi::c_uint,
        number_temporal_layers: *mut core::ffi::c_uint,
    ) -> aom_codec_err_t;
    fn av1_remove_common(cm: *mut AV1Common);
    fn av1_free_ref_frame_buffers(pool: *mut BufferPool);
    fn av1_free_cdef_buffers(
        cm: *mut AV1Common,
        cdef_worker: *mut *mut AV1CdefWorker,
        cdef_sync: *mut AV1CdefSyncData,
    );
    fn av1_free_restoration_buffers(cm: *mut AV1Common);
    fn aom_get_worker_interface() -> *const AVxWorkerInterface;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
pub type aom_codec_caps_t = core::ffi::c_long;
pub type aom_codec_flags_t = core::ffi::c_long;
pub type aom_codec_pts_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_iface {
    pub name: *const core::ffi::c_char,
    pub abi_version: core::ffi::c_int,
    pub caps: aom_codec_caps_t,
    pub init: aom_codec_init_fn_t,
    pub destroy: aom_codec_destroy_fn_t,
    pub ctrl_maps: *const aom_codec_ctrl_fn_map_t,
    pub dec: aom_codec_dec_iface,
    pub enc: aom_codec_enc_iface,
    pub set_option: aom_codec_set_option_fn_t,
}
pub type aom_codec_set_option_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const core::ffi::c_char,
        *const core::ffi::c_char,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_alg_priv_t = aom_codec_alg_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_alg_priv {
    pub base: aom_codec_priv_t,
    pub cfg: aom_codec_dec_cfg_t,
    pub si: aom_codec_stream_info_t,
    pub img: aom_image_t,
    pub img_avail: core::ffi::c_int,
    pub flushed: core::ffi::c_int,
    pub invert_tile_order: core::ffi::c_int,
    pub last_show_frame: *mut RefCntBuffer,
    pub byte_alignment: core::ffi::c_int,
    pub skip_loop_filter: core::ffi::c_int,
    pub skip_film_grain: core::ffi::c_int,
    pub decode_tile_row: core::ffi::c_int,
    pub decode_tile_col: core::ffi::c_int,
    pub tile_mode: core::ffi::c_uint,
    pub ext_tile_debug: core::ffi::c_uint,
    pub row_mt: core::ffi::c_uint,
    pub ext_refs: EXTERNAL_REFERENCES,
    pub is_annexb: core::ffi::c_uint,
    pub operating_point: core::ffi::c_int,
    pub output_all_layers: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub frame_worker: *mut AVxWorker,
    pub image_with_grain: aom_image_t,
    pub grain_image_frame_buffers: [aom_codec_frame_buffer_t; 4],
    pub num_grain_image_frame_buffers: size_t,
    pub need_resync: core::ffi::c_int,
    pub buffer_pool: *mut BufferPool,
    pub ext_priv: *mut core::ffi::c_void,
    pub get_ext_fb_cb: aom_get_frame_buffer_cb_fn_t,
    pub release_ext_fb_cb: aom_release_frame_buffer_cb_fn_t,
}
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
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
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
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
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
pub struct MV_REF {
    pub mv: int_mv,
    pub ref_frame: MV_REFERENCE_FRAME,
}
pub type MV_REFERENCE_FRAME = int8_t;
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
pub struct AVxWorker {
    pub impl_: *mut AVxWorkerImpl,
    pub status_: AVxWorkerStatus,
    pub thread_name: *const core::ffi::c_char,
    pub hook: AVxWorkerHook,
    pub data1: *mut core::ffi::c_void,
    pub data2: *mut core::ffi::c_void,
    pub had_error: core::ffi::c_int,
}
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EXTERNAL_REFERENCES {
    pub refs: [YV12_BUFFER_CONFIG; 128],
    pub num: core::ffi::c_int,
}
pub type aom_codec_stream_info_t = aom_codec_stream_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_stream_info {
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub is_kf: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
    pub number_temporal_layers: core::ffi::c_uint,
    pub is_annexb: core::ffi::c_uint,
}
pub type aom_codec_dec_cfg_t = aom_codec_dec_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_dec_cfg {
    pub threads: core::ffi::c_uint,
    pub w: core::ffi::c_uint,
    pub h: core::ffi::c_uint,
    pub allow_lowbitdepth: core::ffi::c_uint,
}
pub type aom_codec_priv_t = aom_codec_priv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_priv {
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: aom_codec_flags_t,
    pub enc: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub cx_data_dst_buf: aom_fixed_buf_t,
    pub cx_data_pad_before: core::ffi::c_uint,
    pub cx_data_pad_after: core::ffi::c_uint,
    pub cx_data_pkt: aom_codec_cx_pkt_t,
}
pub type aom_codec_cx_pkt_t = aom_codec_cx_pkt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_cx_pkt {
    pub kind: aom_codec_cx_pkt_kind,
    pub data: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub frame: C2RustUnnamed_13,
    pub twopass_stats: aom_fixed_buf_t,
    pub firstpass_mb_stats: aom_fixed_buf_t,
    pub psnr: aom_psnr_pkt,
    pub raw: aom_fixed_buf_t,
}
pub type aom_fixed_buf_t = aom_fixed_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_fixed_buf {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_psnr_pkt {
    pub samples: [core::ffi::c_uint; 4],
    pub sse: [uint64_t; 4],
    pub psnr: [core::ffi::c_double; 4],
    pub samples_hbd: [core::ffi::c_uint; 4],
    pub sse_hbd: [uint64_t; 4],
    pub psnr_hbd: [core::ffi::c_double; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub buf: *mut core::ffi::c_void,
    pub sz: size_t,
    pub pts: aom_codec_pts_t,
    pub duration: core::ffi::c_ulong,
    pub flags: aom_codec_frame_flags_t,
    pub partition_id: core::ffi::c_int,
    pub vis_frame_size: size_t,
}
pub type aom_codec_frame_flags_t = uint32_t;
pub type aom_codec_cx_pkt_kind = core::ffi::c_uint;
pub const AOM_CODEC_CUSTOM_PKT: aom_codec_cx_pkt_kind = 256;
pub const AOM_CODEC_PSNR_PKT: aom_codec_cx_pkt_kind = 3;
pub const AOM_CODEC_FPMB_STATS_PKT: aom_codec_cx_pkt_kind = 2;
pub const AOM_CODEC_STATS_PKT: aom_codec_cx_pkt_kind = 1;
pub const AOM_CODEC_CX_FRAME_PKT: aom_codec_cx_pkt_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_enc_iface {
    pub cfg_count: core::ffi::c_int,
    pub cfgs: *const aom_codec_enc_cfg_t,
    pub encode: aom_codec_encode_fn_t,
    pub get_cx_data: aom_codec_get_cx_data_fn_t,
    pub cfg_set: aom_codec_enc_config_set_fn_t,
    pub get_glob_hdrs: aom_codec_get_global_headers_fn_t,
    pub get_preview: aom_codec_get_preview_frame_fn_t,
}
pub type aom_codec_get_preview_frame_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> *mut aom_image_t,
>;
pub type aom_codec_get_global_headers_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> *mut aom_fixed_buf_t,
>;
pub type aom_codec_enc_config_set_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const aom_codec_enc_cfg_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_enc_cfg_t = aom_codec_enc_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_enc_cfg {
    pub g_usage: core::ffi::c_uint,
    pub g_threads: core::ffi::c_uint,
    pub g_profile: core::ffi::c_uint,
    pub g_w: core::ffi::c_uint,
    pub g_h: core::ffi::c_uint,
    pub g_limit: core::ffi::c_uint,
    pub g_forced_max_frame_width: core::ffi::c_uint,
    pub g_forced_max_frame_height: core::ffi::c_uint,
    pub g_bit_depth: aom_bit_depth_t,
    pub g_input_bit_depth: core::ffi::c_uint,
    pub g_timebase: aom_rational,
    pub g_error_resilient: aom_codec_er_flags_t,
    pub g_pass: aom_enc_pass,
    pub g_lag_in_frames: core::ffi::c_uint,
    pub rc_dropframe_thresh: core::ffi::c_uint,
    pub rc_resize_mode: core::ffi::c_uint,
    pub rc_resize_denominator: core::ffi::c_uint,
    pub rc_resize_kf_denominator: core::ffi::c_uint,
    pub rc_superres_mode: aom_superres_mode,
    pub rc_superres_denominator: core::ffi::c_uint,
    pub rc_superres_kf_denominator: core::ffi::c_uint,
    pub rc_superres_qthresh: core::ffi::c_uint,
    pub rc_superres_kf_qthresh: core::ffi::c_uint,
    pub rc_end_usage: aom_rc_mode,
    pub rc_twopass_stats_in: aom_fixed_buf_t,
    pub rc_firstpass_mb_stats_in: aom_fixed_buf_t,
    pub rc_target_bitrate: core::ffi::c_uint,
    pub rc_min_quantizer: core::ffi::c_uint,
    pub rc_max_quantizer: core::ffi::c_uint,
    pub rc_undershoot_pct: core::ffi::c_uint,
    pub rc_overshoot_pct: core::ffi::c_uint,
    pub rc_buf_sz: core::ffi::c_uint,
    pub rc_buf_initial_sz: core::ffi::c_uint,
    pub rc_buf_optimal_sz: core::ffi::c_uint,
    pub rc_2pass_vbr_bias_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_minsection_pct: core::ffi::c_uint,
    pub rc_2pass_vbr_maxsection_pct: core::ffi::c_uint,
    pub fwd_kf_enabled: core::ffi::c_int,
    pub kf_mode: aom_kf_mode,
    pub kf_min_dist: core::ffi::c_uint,
    pub kf_max_dist: core::ffi::c_uint,
    pub sframe_dist: core::ffi::c_uint,
    pub sframe_mode: core::ffi::c_uint,
    pub large_scale_tile: core::ffi::c_uint,
    pub monochrome: core::ffi::c_uint,
    pub full_still_picture_hdr: core::ffi::c_uint,
    pub save_as_annexb: core::ffi::c_uint,
    pub tile_width_count: core::ffi::c_int,
    pub tile_height_count: core::ffi::c_int,
    pub tile_widths: [core::ffi::c_int; 64],
    pub tile_heights: [core::ffi::c_int; 64],
    pub use_fixed_qp_offsets: core::ffi::c_uint,
    pub fixed_qp_offsets: [core::ffi::c_int; 5],
    pub encoder_cfg: cfg_options_t,
}
pub type cfg_options_t = cfg_options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_options {
    pub init_by_cfg_file: core::ffi::c_uint,
    pub super_block_size: core::ffi::c_uint,
    pub max_partition_size: core::ffi::c_uint,
    pub min_partition_size: core::ffi::c_uint,
    pub disable_ab_partition_type: core::ffi::c_uint,
    pub disable_rect_partition_type: core::ffi::c_uint,
    pub disable_1to4_partition_type: core::ffi::c_uint,
    pub disable_flip_idtx: core::ffi::c_uint,
    pub disable_cdef: core::ffi::c_uint,
    pub disable_lr: core::ffi::c_uint,
    pub disable_obmc: core::ffi::c_uint,
    pub disable_warp_motion: core::ffi::c_uint,
    pub disable_global_motion: core::ffi::c_uint,
    pub disable_dist_wtd_comp: core::ffi::c_uint,
    pub disable_diff_wtd_comp: core::ffi::c_uint,
    pub disable_inter_intra_comp: core::ffi::c_uint,
    pub disable_masked_comp: core::ffi::c_uint,
    pub disable_one_sided_comp: core::ffi::c_uint,
    pub disable_palette: core::ffi::c_uint,
    pub disable_intrabc: core::ffi::c_uint,
    pub disable_cfl: core::ffi::c_uint,
    pub disable_smooth_intra: core::ffi::c_uint,
    pub disable_filter_intra: core::ffi::c_uint,
    pub disable_dual_filter: core::ffi::c_uint,
    pub disable_intra_angle_delta: core::ffi::c_uint,
    pub disable_intra_edge_filter: core::ffi::c_uint,
    pub disable_tx_64x64: core::ffi::c_uint,
    pub disable_smooth_inter_intra: core::ffi::c_uint,
    pub disable_inter_inter_wedge: core::ffi::c_uint,
    pub disable_inter_intra_wedge: core::ffi::c_uint,
    pub disable_paeth_intra: core::ffi::c_uint,
    pub disable_trellis_quant: core::ffi::c_uint,
    pub disable_ref_frame_mv: core::ffi::c_uint,
    pub reduced_reference_set: core::ffi::c_uint,
    pub reduced_tx_type_set: core::ffi::c_uint,
}
pub type aom_kf_mode = core::ffi::c_uint;
pub const AOM_KF_DISABLED: aom_kf_mode = 0;
pub const AOM_KF_AUTO: aom_kf_mode = 1;
pub const AOM_KF_FIXED: aom_kf_mode = 0;
pub type aom_rc_mode = core::ffi::c_uint;
pub const AOM_Q: aom_rc_mode = 3;
pub const AOM_CQ: aom_rc_mode = 2;
pub const AOM_CBR: aom_rc_mode = 1;
pub const AOM_VBR: aom_rc_mode = 0;
pub type aom_superres_mode = core::ffi::c_uint;
pub const AOM_SUPERRES_AUTO: aom_superres_mode = 4;
pub const AOM_SUPERRES_QTHRESH: aom_superres_mode = 3;
pub const AOM_SUPERRES_RANDOM: aom_superres_mode = 2;
pub const AOM_SUPERRES_FIXED: aom_superres_mode = 1;
pub const AOM_SUPERRES_NONE: aom_superres_mode = 0;
pub type aom_enc_pass = core::ffi::c_uint;
pub const AOM_RC_LAST_PASS: aom_enc_pass = 2;
pub const AOM_RC_THIRD_PASS: aom_enc_pass = 3;
pub const AOM_RC_SECOND_PASS: aom_enc_pass = 2;
pub const AOM_RC_FIRST_PASS: aom_enc_pass = 1;
pub const AOM_RC_ONE_PASS: aom_enc_pass = 0;
pub type aom_codec_er_flags_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_rational {
    pub num: core::ffi::c_int,
    pub den: core::ffi::c_int,
}
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type aom_codec_get_cx_data_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_iter_t,
    ) -> *const aom_codec_cx_pkt_t,
>;
pub type aom_codec_iter_t = *const core::ffi::c_void;
pub type aom_codec_encode_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const aom_image_t,
        aom_codec_pts_t,
        core::ffi::c_ulong,
        aom_enc_frame_flags_t,
    ) -> aom_codec_err_t,
>;
pub type aom_enc_frame_flags_t = core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_dec_iface {
    pub peek_si: aom_codec_peek_si_fn_t,
    pub get_si: aom_codec_get_si_fn_t,
    pub decode: aom_codec_decode_fn_t,
    pub get_frame: aom_codec_get_frame_fn_t,
    pub set_fb_fn: aom_codec_set_fb_fn_t,
}
pub type aom_codec_set_fb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        aom_get_frame_buffer_cb_fn_t,
        aom_release_frame_buffer_cb_fn_t,
        *mut core::ffi::c_void,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_get_frame_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_iter_t,
    ) -> *mut aom_image_t,
>;
pub type aom_codec_decode_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *const uint8_t,
        size_t,
        *mut core::ffi::c_void,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_get_si_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        *mut aom_codec_stream_info_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_peek_si_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const uint8_t,
        size_t,
        *mut aom_codec_stream_info_t,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_ctrl_fn_map_t = aom_codec_ctrl_fn_map;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_ctrl_fn_map {
    pub ctrl_id: core::ffi::c_int,
    pub fn_0: aom_codec_control_fn_t,
}
pub type aom_codec_control_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        core::ffi::VaList<'_>,
    ) -> aom_codec_err_t,
>;
pub type aom_codec_destroy_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> aom_codec_err_t,
>;
pub type aom_codec_init_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_ctx_t) -> aom_codec_err_t,
>;
pub type aom_codec_ctx_t = aom_codec_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_ctx {
    pub name: *const core::ffi::c_char,
    pub iface: *const aom_codec_iface_t,
    pub err: aom_codec_err_t,
    pub err_detail: *const core::ffi::c_char,
    pub init_flags: aom_codec_flags_t,
    pub config: C2RustUnnamed_14,
    pub priv_0: *mut aom_codec_priv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub dec: *const aom_codec_dec_cfg,
    pub enc: *const aom_codec_enc_cfg,
    pub raw: *const core::ffi::c_void,
}
pub type aom_codec_iface_t = aom_codec_iface;
pub type aom_superblock_size = core::ffi::c_uint;
pub const AOM_SUPERBLOCK_SIZE_DYNAMIC: aom_superblock_size = 2;
pub const AOM_SUPERBLOCK_SIZE_128X128: aom_superblock_size = 1;
pub const AOM_SUPERBLOCK_SIZE_64X64: aom_superblock_size = 0;
pub type aom_superblock_size_t = aom_superblock_size;
pub type OBU_TYPE = core::ffi::c_uchar;
pub const OBU_PADDING: OBU_TYPE = 15;
pub const OBU_TILE_LIST: OBU_TYPE = 8;
pub const OBU_REDUNDANT_FRAME_HEADER: OBU_TYPE = 7;
pub const OBU_FRAME: OBU_TYPE = 6;
pub const OBU_METADATA: OBU_TYPE = 5;
pub const OBU_TILE_GROUP: OBU_TYPE = 4;
pub const OBU_FRAME_HEADER: OBU_TYPE = 3;
pub const OBU_TEMPORAL_DELIMITER: OBU_TYPE = 2;
pub const OBU_SEQUENCE_HEADER: OBU_TYPE = 1;
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
pub type aom_alloc_img_data_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut core::ffi::c_void, size_t) -> *mut core::ffi::c_void,
>;
pub type aom_com_control_id = core::ffi::c_uint;
pub const AOM_DECODER_CTRL_ID_START: aom_com_control_id = 256;
pub const AV1_COPY_NEW_FRAME_IMAGE: aom_com_control_id = 234;
pub const AV1_GET_NEW_FRAME_IMAGE: aom_com_control_id = 233;
pub const AV1_COPY_REFERENCE: aom_com_control_id = 232;
pub const AV1_SET_REFERENCE: aom_com_control_id = 231;
pub const AV1_GET_REFERENCE: aom_com_control_id = 230;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct av1_ref_frame {
    pub idx: core::ffi::c_int,
    pub use_external_ref: core::ffi::c_int,
    pub img: aom_image_t,
}
pub type av1_ref_frame_t = av1_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AllocCbParam {
    pub pool: *mut BufferPool,
    pub fb: *mut aom_codec_frame_buffer_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1Decoder {
    pub dcb: DecoderCodingBlock,
    pub __aom_align_common: [crate::platform::Align32; 0],
    pub common: AV1_COMMON,
    pub lf_worker: AVxWorker,
    pub lf_row_sync: AV1LfSync,
    pub lr_row_sync: AV1LrSync,
    pub lr_ctxt: AV1LrStruct,
    pub cdef_sync: AV1CdefSync,
    pub cdef_worker: *mut AV1CdefWorkerData,
    pub tile_workers: *mut AVxWorker,
    pub num_workers: core::ffi::c_int,
    pub thread_data: *mut DecWorkerData,
    pub td: ThreadData,
    pub tile_data: *mut TileDataDec,
    pub allocated_tiles: core::ffi::c_int,
    pub tile_buffers: [[TileBufferDec; 64]; 64],
    pub tile_mt_info: AV1DecTileMT,
    pub output_all_layers: core::ffi::c_int,
    pub output_frames: [*mut RefCntBuffer; 4],
    pub num_output_frames: size_t,
    pub decoding_first_frame: core::ffi::c_int,
    pub allow_lowbitdepth: core::ffi::c_int,
    pub max_threads: core::ffi::c_int,
    pub inv_tile_order: core::ffi::c_int,
    pub need_resync: core::ffi::c_int,
    pub reset_decoder_state: core::ffi::c_int,
    pub tile_size_bytes: core::ffi::c_int,
    pub tile_col_size_bytes: core::ffi::c_int,
    pub dec_tile_row: core::ffi::c_int,
    pub dec_tile_col: core::ffi::c_int,
    pub sequence_header_ready: core::ffi::c_int,
    pub sequence_header_changed: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub current_operating_point: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub seen_frame_header: core::ffi::c_int,
    pub next_start_tile: core::ffi::c_int,
    pub camera_frame_header_ready: core::ffi::c_int,
    pub frame_header_size: size_t,
    pub obu_size_hdr: DataBuffer,
    pub output_frame_width_in_tiles_minus_1: core::ffi::c_int,
    pub output_frame_height_in_tiles_minus_1: core::ffi::c_int,
    pub tile_count_minus_1: core::ffi::c_int,
    pub coded_tile_data_size: uint32_t,
    pub ext_tile_debug: core::ffi::c_uint,
    pub row_mt: core::ffi::c_uint,
    pub ext_refs: EXTERNAL_REFERENCES,
    pub tile_list_outbuf: YV12_BUFFER_CONFIG,
    pub cb_buffer_base: *mut CB_BUFFER,
    pub cb_buffer_alloc_size: core::ffi::c_int,
    pub allocated_row_mt_sync_rows: core::ffi::c_int,
    pub row_mt_mutex_: *mut pthread_mutex_t,
    pub row_mt_cond_: *mut pthread_cond_t,
    pub frame_row_mt_info: AV1DecRowMTInfo,
    pub metadata: *mut aom_metadata_array_t,
    pub context_update_tile_id: core::ffi::c_int,
    pub skip_loop_filter: core::ffi::c_int,
    pub skip_film_grain: core::ffi::c_int,
    pub is_annexb: core::ffi::c_int,
    pub valid_for_referencing: [core::ffi::c_int; 8],
    pub is_fwd_kf_present: core::ffi::c_int,
    pub is_arf_frame_present: core::ffi::c_int,
    pub num_tile_groups: core::ffi::c_int,
    pub sframe_info: aom_s_frame_info,
    pub seq_params: SequenceHeader,
    pub buffer_removal_time_present: bool,
    pub error: aom_internal_error_info,
    pub number_temporal_layers: core::ffi::c_uint,
    pub number_spatial_layers: core::ffi::c_uint,
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
pub type AV1_LEVEL = uint8_t;
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
pub type BITSTREAM_PROFILE = int8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OrderHintInfo {
    pub enable_order_hint: core::ffi::c_int,
    pub order_hint_bits_minus_1: core::ffi::c_int,
    pub enable_dist_wtd_comp: core::ffi::c_int,
    pub enable_ref_frame_mvs: core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_s_frame_info {
    pub is_s_frame: core::ffi::c_int,
    pub is_s_frame_at_altref: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTInfo {
    pub tile_rows_start: core::ffi::c_int,
    pub tile_rows_end: core::ffi::c_int,
    pub tile_cols_start: core::ffi::c_int,
    pub tile_cols_end: core::ffi::c_int,
    pub start_tile: core::ffi::c_int,
    pub end_tile: core::ffi::c_int,
    pub mi_rows_to_decode: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub row_mt_exit: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [core::ffi::c_char; 48],
    pub __align: core::ffi::c_longlong,
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
pub union __atomic_wide_counter {
    pub __value64: core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub __low: core::ffi::c_uint,
    pub __high: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CB_BUFFER {
    pub __aom_align_dqcoeff: [crate::platform::Align32; 0],

    pub dqcoeff: [[tran_low_t; 16384]; 3],
    pub eob_data: [[eob_info; 1024]; 3],
    pub __aom_align_color_index_map: [crate::platform::Align16; 0],
    pub color_index_map: [[uint8_t; 16384]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eob_info {
    pub eob: uint16_t,
    pub max_scan_line: uint16_t,
}
pub type tran_low_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DataBuffer {
    pub data: *const uint8_t,
    pub size: size_t,
}
pub type AV1DecTileMT = AV1DecTileMTData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecTileMTData {
    pub job_mutex: *mut pthread_mutex_t,
    pub job_queue: *mut TileJobsDec,
    pub jobs_enqueued: core::ffi::c_int,
    pub jobs_dequeued: core::ffi::c_int,
    pub alloc_tile_rows: core::ffi::c_int,
    pub alloc_tile_cols: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileJobsDec {
    pub tile_buffer: *mut TileBufferDec,
    pub tile_data: *mut TileDataDec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileDataDec {
    pub tile_info: TileInfo,
    pub bit_reader: aom_reader,
    pub __aom_align_tctx: [crate::platform::Align16; 0],
    pub tctx: FRAME_CONTEXT,
    pub dec_row_mt_sync: AV1DecRowMTSync,
}
pub type AV1DecRowMTSync = AV1DecRowMTSyncData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1DecRowMTSyncData {
    pub mutex_: *mut pthread_mutex_t,
    pub cond_: *mut pthread_cond_t,
    pub allocated_sb_rows: core::ffi::c_int,
    pub cur_sb_col: *mut core::ffi::c_int,
    pub sync_range: core::ffi::c_int,
    pub intrabc_extra_top_right_sb_delay: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_rows_parse_done: core::ffi::c_int,
    pub mi_rows_decode_started: core::ffi::c_int,
    pub num_threads_working: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_reader {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub ec: od_ec_dec,
    pub allow_update_cdf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_dec {
    pub buf: *const core::ffi::c_uchar,
    pub tell_offs: int32_t,
    pub end: *const core::ffi::c_uchar,
    pub bptr: *const core::ffi::c_uchar,
    pub dif: od_ec_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
}
pub type od_ec_window = uint32_t;
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
pub struct TileBufferDec {
    pub data: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadData {
    pub dcb: DecoderCodingBlock,
    pub cb_buffer_base: CB_BUFFER,
    pub bit_reader: *mut aom_reader,
    pub mc_buf: [*mut uint8_t; 2],
    pub seg_mask: *mut uint8_t,
    pub mc_buf_size: int32_t,
    pub mc_buf_use_highbd: core::ffi::c_int,
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub read_coeffs_tx_intra_block_visit: decode_block_visitor_fn_t,
    pub predict_and_recon_intra_block_visit: decode_block_visitor_fn_t,
    pub read_coeffs_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub inverse_tx_inter_block_visit: decode_block_visitor_fn_t,
    pub predict_inter_block_visit: predict_inter_block_visitor_fn_t,
    pub cfl_store_inter_block_visit: cfl_store_inter_block_visitor_fn_t,
}
pub type cfl_store_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut MACROBLOCKD) -> (),
>;
pub type MACROBLOCKD = macroblockd;
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
pub type TXFM_CONTEXT = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type ENTROPY_CONTEXT = core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
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
pub type TX_SIZE = uint8_t;
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
pub type FILTER_INTRA_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERINTER_COMPOUND_DATA {
    pub seg_mask: *mut uint8_t,
    pub wedge_index: int8_t,
    pub wedge_sign: int8_t,
    pub mask_type: DIFFWTD_MASK_TYPE,
    pub type_0: COMPOUND_TYPE,
}
pub type COMPOUND_TYPE = uint8_t;
pub type DIFFWTD_MASK_TYPE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
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
pub type UV_PREDICTION_MODE = uint8_t;
pub type PREDICTION_MODE = uint8_t;
pub type PARTITION_TYPE = uint8_t;
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
pub type qm_val_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_2d {
    pub buf: *mut uint8_t,
    pub buf0: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type PLANE_TYPE = uint8_t;
pub type AV1_COMMON = AV1Common;
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
pub struct CommonContexts {
    pub partition: *mut *mut PARTITION_CONTEXT,
    pub entropy: [*mut *mut ENTROPY_CONTEXT; 3],
    pub txfm: *mut *mut TXFM_CONTEXT,
    pub num_planes: core::ffi::c_int,
    pub num_tile_rows: core::ffi::c_int,
    pub num_mi_cols: core::ffi::c_int,
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
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
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
pub type TX_TYPE = uint8_t;
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
pub type TX_MODE = uint8_t;
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
pub type predict_inter_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut AV1_COMMON, *mut DecoderCodingBlock, BLOCK_SIZE) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecoderCodingBlock {
    pub __aom_align_xd: [crate::platform::Align32; 0],

    pub xd: MACROBLOCKD,
    pub corrupted: core::ffi::c_int,
    pub mc_buf: [*mut uint8_t; 2],
    pub dqcoeff_block: [*mut tran_low_t; 3],
    pub cb_offset: [uint16_t; 3],
    pub eob_data: [*mut eob_info; 3],
    pub txb_offset: [uint16_t; 3],
    pub ref_mv_count: [uint8_t; 29],
}
pub type decode_block_visitor_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *const AV1_COMMON,
        *mut DecoderCodingBlock,
        *mut aom_reader,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        TX_SIZE,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecWorkerData {
    pub td: *mut ThreadData,
    pub data_end: *const uint8_t,
    pub error_info: aom_internal_error_info,
}
pub type AV1CdefWorkerData = AV1CdefWorker;
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
pub type AV1CdefSync = AV1CdefSyncData;
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
pub struct RestorationTileLimits {
    pub h_start: core::ffi::c_int,
    pub h_end: core::ffi::c_int,
    pub v_start: core::ffi::c_int,
    pub v_end: core::ffi::c_int,
}
pub type AV1LrSync = AV1LrSyncData;
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
pub type AV1LfSync = AV1LfSyncData;
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
pub type LFWorkerData = LoopFilterWorkerData;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1_DEBLOCKING_PARAMETERS {
    pub filter_length: uint8_t,
    pub lfthr: *const loop_filter_thresh,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameWorkerData {
    pub pbi: *mut AV1Decoder,
    pub data: *const uint8_t,
    pub data_end: *const uint8_t,
    pub data_size: size_t,
    pub user_priv: *mut core::ffi::c_void,
    pub received_frame: core::ffi::c_int,
    pub frame_context_ready: core::ffi::c_int,
    pub frame_decoded: core::ffi::c_int,
}
pub const INTRA_ONLY_FRAME: C2RustUnnamed_17 = 2;
pub const KEY_FRAME: C2RustUnnamed_17 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObuHeader {
    pub size: size_t,
    pub type_0: OBU_TYPE,
    pub has_size_field: core::ffi::c_int,
    pub has_extension: core::ffi::c_int,
    pub temporal_layer_id: core::ffi::c_int,
    pub spatial_layer_id: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_read_bit_buffer {
    pub bit_buffer: *const uint8_t,
    pub bit_buffer_end: *const uint8_t,
    pub bit_offset: uint32_t,
    pub error_handler_data: *mut core::ffi::c_void,
    pub error_handler: aom_rb_error_handler,
}
pub type aom_rb_error_handler = Option<
    unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [core::ffi::c_char; 4],
    pub __align: core::ffi::c_int,
}
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_16 = 7;
pub const REF_FRAMES: C2RustUnnamed_16 = 8;
pub const AV1D_GET_MI_INFO: aom_dec_control_id = 295;
pub const AOMD_GET_ORDER_HINT: aom_dec_control_id = 294;
pub const AOMD_GET_BASE_Q_IDX: aom_dec_control_id = 293;
pub const AOMD_GET_SHOW_FRAME_FLAG: aom_dec_control_id = 292;
pub const AOMD_GET_S_FRAME_INFO: aom_dec_control_id = 291;
pub const AOMD_GET_SHOW_EXISTING_FRAME_FLAG: aom_dec_control_id = 290;
pub const AOMD_GET_SB_SIZE: aom_dec_control_id = 289;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_still_picture_info {
    pub is_still_picture: core::ffi::c_int,
    pub is_reduced_still_picture_hdr: core::ffi::c_int,
}
pub const AOMD_GET_STILL_PICTURE: aom_dec_control_id = 288;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_screen_content_tools_info {
    pub allow_screen_content_tools: core::ffi::c_int,
    pub allow_intrabc: core::ffi::c_int,
    pub force_integer_mv: core::ffi::c_int,
}
pub const AOMD_GET_SCREEN_CONTENT_TOOLS_INFO: aom_dec_control_id = 287;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_tile_info {
    pub tile_columns: core::ffi::c_int,
    pub tile_rows: core::ffi::c_int,
    pub tile_widths: [core::ffi::c_int; 64],
    pub tile_heights: [core::ffi::c_int; 64],
    pub num_tile_groups: core::ffi::c_int,
}
pub const AOMD_GET_TILE_INFO: aom_dec_control_id = 286;
pub const S_FRAME: C2RustUnnamed_17 = 3;
pub const AOMD_GET_FRAME_FLAGS: aom_dec_control_id = 284;
pub const AOMD_GET_ALTREF_PRESENT: aom_dec_control_id = 285;
pub const AOMD_GET_FWD_KF_PRESENT: aom_dec_control_id = 283;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_tile_data {
    pub coded_tile_data_size: size_t,
    pub coded_tile_data: *const core::ffi::c_void,
    pub extra_size: size_t,
}
pub const AV1D_GET_TILE_DATA: aom_dec_control_id = 274;
pub const AV1D_GET_FRAME_HEADER_INFO: aom_dec_control_id = 273;
pub const AV1_GET_ACCOUNTING: aom_dec_control_id = 268;
pub const AV1D_GET_FRAME_SIZE: aom_dec_control_id = 259;
pub const AV1D_GET_DISPLAY_SIZE: aom_dec_control_id = 260;
pub const AV1D_GET_TILE_COUNT: aom_dec_control_id = 264;
pub const AV1D_GET_TILE_SIZE: aom_dec_control_id = 263;
pub const AV1D_GET_IMG_FORMAT: aom_dec_control_id = 262;
pub const AV1D_GET_BIT_DEPTH: aom_dec_control_id = 261;
pub const AOMD_GET_LAST_REF_UPDATES: aom_dec_control_id = 256;
pub const AOMD_GET_LAST_QUANTIZER: aom_dec_control_id = 269;
pub const AOMD_GET_FRAME_CORRUPTED: aom_dec_control_id = 257;
pub const AOMD_SET_FRAME_SIZE_LIMIT: aom_dec_control_id = 296;
pub const AV1D_SET_SKIP_FILM_GRAIN: aom_dec_control_id = 282;
pub type av1_ext_ref_frame_t = av1_ext_ref_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct av1_ext_ref_frame {
    pub img: *mut aom_image_t,
    pub num: core::ffi::c_int,
}
pub const AV1D_SET_EXT_REF_PTR: aom_dec_control_id = 275;
pub const AV1D_SET_ROW_MT: aom_dec_control_id = 277;
pub const AV1D_EXT_TILE_DEBUG: aom_dec_control_id = 276;
pub const AV1_SET_INSPECTION_CALLBACK: aom_dec_control_id = 281;
pub const AV1D_SET_OUTPUT_ALL_LAYERS: aom_dec_control_id = 280;
pub const AV1D_SET_OPERATING_POINT: aom_dec_control_id = 279;
pub const AV1D_SET_IS_ANNEXB: aom_dec_control_id = 278;
pub const AV1_SET_TILE_MODE: aom_dec_control_id = 272;
pub const AV1_SET_DECODE_TILE_COL: aom_dec_control_id = 271;
pub const AV1_SET_DECODE_TILE_ROW: aom_dec_control_id = 270;
pub const AV1_SET_SKIP_LOOP_FILTER: aom_dec_control_id = 267;
pub const AV1_SET_BYTE_ALIGNMENT: aom_dec_control_id = 265;
pub const AV1_INVERT_TILE_DECODE_ORDER: aom_dec_control_id = 266;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1DecodeReturn {
    pub buf: *const core::ffi::c_uchar,
    pub idx: core::ffi::c_int,
    pub show_existing: core::ffi::c_int,
}
pub type aom_dec_control_id = core::ffi::c_uint;
pub const AOMD_GET_LAST_REF_USED: aom_dec_control_id = 258;
pub type C2RustUnnamed_16 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_16 = 7;
pub const BWD_REFS: C2RustUnnamed_16 = 3;
pub const FWD_REFS: C2RustUnnamed_16 = 4;
pub const EXTREF_FRAME: C2RustUnnamed_16 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_16 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_16 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_16 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_16 = 4;
pub const LAST3_FRAME: C2RustUnnamed_16 = 3;
pub const LAST2_FRAME: C2RustUnnamed_16 = 2;
pub const LAST_FRAME: C2RustUnnamed_16 = 1;
pub const INTRA_FRAME: C2RustUnnamed_16 = 0;
pub const NONE_FRAME: C2RustUnnamed_16 = -1;
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub const FRAME_TYPES: C2RustUnnamed_17 = 4;
pub const INTER_FRAME: C2RustUnnamed_17 = 1;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_CODEC_INTERNAL_ABI_VERSION: core::ffi::c_int = 7 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const AOM_IMG_FMT_HIGHBITDEPTH: core::ffi::c_int = 0x800 as core::ffi::c_int;
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AOM_PLANE_V: core::ffi::c_int = 2 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
unsafe extern "C-unwind" fn decoder_init(mut ctx: *mut aom_codec_ctx_t) -> aom_codec_err_t {
    if ((*ctx).priv_0).is_null() {
        let priv_0: *mut aom_codec_alg_priv_t = aom_calloc(
            1 as size_t,
            ::core::mem::size_of::<aom_codec_alg_priv_t>() as size_t,
        ) as *mut aom_codec_alg_priv_t;
        if priv_0.is_null() {
            return AOM_CODEC_MEM_ERROR;
        }
        (*ctx).priv_0 = priv_0 as *mut aom_codec_priv_t;
        (*(*ctx).priv_0).init_flags = (*ctx).init_flags;
        (*priv_0).flushed = 0 as core::ffi::c_int;
        (*priv_0).cfg.allow_lowbitdepth = (FORCE_HIGHBITDEPTH_DECODING == 0)
            as core::ffi::c_int as core::ffi::c_uint;
        if !((*ctx).config.dec).is_null() {
            (*priv_0).cfg = *(*ctx).config.dec as aom_codec_dec_cfg_t;
            (*ctx).config.dec = &raw mut (*priv_0).cfg;
        }
        (*priv_0).num_grain_image_frame_buffers = 0 as size_t;
        (*priv_0).row_mt = 1 as core::ffi::c_uint;
        (*priv_0).tile_mode = 0 as core::ffi::c_uint;
        (*priv_0).decode_tile_row = -(1 as core::ffi::c_int);
        (*priv_0).decode_tile_col = -(1 as core::ffi::c_int);
        (*priv_0).frame_size_limit = 0 as core::ffi::c_uint;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn decoder_destroy(
    mut ctx: *mut aom_codec_alg_priv_t,
) -> aom_codec_err_t {
    if !((*ctx).frame_worker).is_null() {
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        ((*aom_get_worker_interface()).end).expect("non-null function pointer")(worker);
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        if !frame_worker_data.is_null() && !((*frame_worker_data).pbi).is_null() {
            let pbi: *mut AV1Decoder = (*frame_worker_data).pbi as *mut AV1Decoder;
            aom_free((*pbi).common.tpl_mvs as *mut core::ffi::c_void);
            (*pbi).common.tpl_mvs = 0 as *mut TPL_MV_REF;
            av1_remove_common(&raw mut (*pbi).common);
            av1_free_cdef_buffers(
                &raw mut (*pbi).common,
                &raw mut (*pbi).cdef_worker,
                &raw mut (*pbi).cdef_sync,
            );
            av1_free_cdef_sync(&raw mut (*pbi).cdef_sync);
            av1_free_restoration_buffers(&raw mut (*pbi).common);
            av1_decoder_remove(pbi as *mut AV1Decoder);
        }
        aom_free(frame_worker_data as *mut core::ffi::c_void);
    }
    if !((*ctx).buffer_pool).is_null() {
        let mut i: size_t = 0 as size_t;
        while i < (*ctx).num_grain_image_frame_buffers {
            ((*(*ctx).buffer_pool).release_fb_cb)
                .expect(
                    "non-null function pointer",
                )(
                (*(*ctx).buffer_pool).cb_priv,
                &raw mut *((*ctx).grain_image_frame_buffers).as_mut_ptr().offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
        av1_free_ref_frame_buffers((*ctx).buffer_pool as *mut BufferPool);
        av1_free_internal_frame_buffers(&raw mut (*(*ctx).buffer_pool).int_frame_buffers);
        pthread_mutex_destroy(&raw mut (*(*ctx).buffer_pool).pool_mutex);
    }
    aom_free((*ctx).frame_worker as *mut core::ffi::c_void);
    aom_free((*ctx).buffer_pool as *mut core::ffi::c_void);
    aom_img_free(&raw mut (*ctx).img);
    aom_free(ctx as *mut core::ffi::c_void);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn parse_timing_info(
    mut rb: *mut aom_read_bit_buffer,
) -> aom_codec_err_t {
    let num_units_in_display_tick: uint32_t = aom_rb_read_unsigned_literal(
        rb,
        32 as core::ffi::c_int,
    ) as uint32_t;
    let time_scale: uint32_t = aom_rb_read_unsigned_literal(rb, 32 as core::ffi::c_int)
        as uint32_t;
    if num_units_in_display_tick == 0 as uint32_t || time_scale == 0 as uint32_t {
        return AOM_CODEC_UNSUP_BITSTREAM;
    }
    let equal_picture_interval: uint8_t = aom_rb_read_bit(rb) as uint8_t;
    if equal_picture_interval != 0 {
        let num_ticks_per_picture_minus_1: uint32_t = aom_rb_read_uvlc(rb) as uint32_t;
        if num_ticks_per_picture_minus_1 == UINT32_MAX as uint32_t {
            return AOM_CODEC_UNSUP_BITSTREAM;
        }
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn parse_decoder_model_info(
    mut rb: *mut aom_read_bit_buffer,
    mut buffer_delay_length_minus_1: *mut core::ffi::c_int,
) -> aom_codec_err_t {
    *buffer_delay_length_minus_1 = aom_rb_read_literal(rb, 5 as core::ffi::c_int);
    let num_units_in_decoding_tick: uint32_t = aom_rb_read_unsigned_literal(
        rb,
        32 as core::ffi::c_int,
    ) as uint32_t;
    let buffer_removal_time_length_minus_1: uint8_t = aom_rb_read_literal(
        rb,
        5 as core::ffi::c_int,
    ) as uint8_t;
    let frame_presentation_time_length_minus_1: uint8_t = aom_rb_read_literal(
        rb,
        5 as core::ffi::c_int,
    ) as uint8_t;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn parse_op_parameters_info(
    mut rb: *mut aom_read_bit_buffer,
    mut buffer_delay_length_minus_1: core::ffi::c_int,
) -> aom_codec_err_t {
    let n: core::ffi::c_int = buffer_delay_length_minus_1 + 1 as core::ffi::c_int;
    let decoder_buffer_delay: uint32_t = aom_rb_read_unsigned_literal(rb, n) as uint32_t;
    let encoder_buffer_delay: uint32_t = aom_rb_read_unsigned_literal(rb, n) as uint32_t;
    let low_delay_mode_flag: uint8_t = aom_rb_read_bit(rb) as uint8_t;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn parse_operating_points(
    mut rb: *mut aom_read_bit_buffer,
    mut is_reduced_header: core::ffi::c_int,
    mut si: *mut aom_codec_stream_info_t,
) -> aom_codec_err_t {
    let mut operating_point_idc0: core::ffi::c_int = 0 as core::ffi::c_int;
    if is_reduced_header != 0 {
        aom_rb_read_literal(rb, LEVEL_BITS);
    } else {
        let mut decoder_model_info_present_flag: uint8_t = 0 as uint8_t;
        let mut buffer_delay_length_minus_1: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut status: aom_codec_err_t = AOM_CODEC_OK;
        let timing_info_present_flag: uint8_t = aom_rb_read_bit(rb) as uint8_t;
        if timing_info_present_flag != 0 {
            status = parse_timing_info(rb);
            if status as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return status;
            }
            decoder_model_info_present_flag = aom_rb_read_bit(rb) as uint8_t;
            if decoder_model_info_present_flag != 0 {
                status = parse_decoder_model_info(rb, &raw mut buffer_delay_length_minus_1);
                if status as core::ffi::c_uint
                    != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                {
                    return status;
                }
            }
        }
        let initial_display_delay_present_flag: uint8_t = aom_rb_read_bit(rb) as uint8_t;
        let operating_points_cnt_minus_1: uint8_t = aom_rb_read_literal(
            rb,
            OP_POINTS_CNT_MINUS_1_BITS,
        ) as uint8_t;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i
            < operating_points_cnt_minus_1 as core::ffi::c_int + 1 as core::ffi::c_int
        {
            let mut operating_point_idc: core::ffi::c_int = 0;
            operating_point_idc = aom_rb_read_literal(rb, OP_POINTS_IDC_BITS);
            if i == 0 as core::ffi::c_int {
                operating_point_idc0 = operating_point_idc;
            }
            let mut seq_level_idx: core::ffi::c_int = aom_rb_read_literal(
                rb,
                LEVEL_BITS,
            );
            if seq_level_idx > 7 as core::ffi::c_int {
                aom_rb_read_bit(rb);
            }
            if decoder_model_info_present_flag != 0 {
                let decoder_model_present_for_this_op: uint8_t = aom_rb_read_bit(rb)
                    as uint8_t;
                if decoder_model_present_for_this_op != 0 {
                    status = parse_op_parameters_info(rb, buffer_delay_length_minus_1);
                    if status as core::ffi::c_uint
                        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return status;
                    }
                }
            }
            if initial_display_delay_present_flag != 0 {
                let initial_display_delay_present_for_this_op: uint8_t = aom_rb_read_bit(
                    rb,
                ) as uint8_t;
                if initial_display_delay_present_for_this_op != 0 {
                    aom_rb_read_literal(rb, 4 as core::ffi::c_int);
                }
            }
            i += 1;
        }
    }
    return aom_get_num_layers_from_operating_point_idc(
        operating_point_idc0,
        &raw mut (*si).number_spatial_layers,
        &raw mut (*si).number_temporal_layers,
    );
}
unsafe extern "C-unwind" fn decoder_peek_si_internal(
    mut data: *const uint8_t,
    mut data_sz: size_t,
    mut si: *mut aom_codec_stream_info_t,
    mut is_intra_only: *mut core::ffi::c_int,
) -> aom_codec_err_t {
    let mut intra_only_flag: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut got_sequence_header: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut found_keyframe: core::ffi::c_int = 0 as core::ffi::c_int;
    if data.offset(data_sz as isize) <= data || data_sz < 1 as size_t {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*si).w = 0 as core::ffi::c_uint;
    (*si).h = 0 as core::ffi::c_uint;
    (*si).is_kf = 0 as core::ffi::c_uint;
    let mut obu_header: ObuHeader = ObuHeader {
        size: 0,
        type_0: 0 as OBU_TYPE,
        has_size_field: 0,
        has_extension: 0,
        temporal_layer_id: 0,
        spatial_layer_id: 0,
    };
    memset(
        &raw mut obu_header as *mut ObuHeader as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ObuHeader>() as size_t,
    );
    let mut payload_size: size_t = 0 as size_t;
    let mut bytes_read: size_t = 0 as size_t;
    let mut reduced_still_picture_hdr: uint8_t = 0 as uint8_t;
    let mut status: aom_codec_err_t = aom_read_obu_header_and_size(
        data,
        data_sz,
        (*si).is_annexb as core::ffi::c_int,
        &raw mut obu_header,
        &raw mut payload_size,
        &raw mut bytes_read,
    );
    if status as core::ffi::c_uint
        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return status;
    }
    if obu_header.type_0 as core::ffi::c_int
        == OBU_TEMPORAL_DELIMITER as core::ffi::c_int
    {
        if data_sz < bytes_read.wrapping_add(payload_size) {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        data = data.offset(bytes_read.wrapping_add(payload_size) as isize);
        data_sz = data_sz.wrapping_sub(bytes_read.wrapping_add(payload_size));
        status = aom_read_obu_header_and_size(
            data,
            data_sz,
            (*si).is_annexb as core::ffi::c_int,
            &raw mut obu_header,
            &raw mut payload_size,
            &raw mut bytes_read,
        );
        if status as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return status;
        }
    }
    loop {
        data = data.offset(bytes_read as isize);
        data_sz = data_sz.wrapping_sub(bytes_read);
        if data_sz < payload_size {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        if obu_header.type_0 as core::ffi::c_int
            == OBU_SEQUENCE_HEADER as core::ffi::c_int
        {
            if data_sz < 2 as size_t {
                return AOM_CODEC_CORRUPT_FRAME;
            }
            let mut rb: aom_read_bit_buffer = {
                let mut init = aom_read_bit_buffer {
                    bit_buffer: data,
                    bit_buffer_end: data.offset(data_sz as isize),
                    bit_offset: 0 as uint32_t,
                    error_handler_data: NULL,
                    error_handler: None,
                };
                init
            };
            av1_read_profile(&raw mut rb);
            let still_picture: uint8_t = aom_rb_read_bit(&raw mut rb) as uint8_t;
            reduced_still_picture_hdr = aom_rb_read_bit(&raw mut rb) as uint8_t;
            if still_picture == 0 && reduced_still_picture_hdr as core::ffi::c_int != 0 {
                return AOM_CODEC_UNSUP_BITSTREAM;
            }
            status = parse_operating_points(
                &raw mut rb,
                reduced_still_picture_hdr as core::ffi::c_int,
                si,
            );
            if status as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return status;
            }
            let mut num_bits_width: core::ffi::c_int = aom_rb_read_literal(
                &raw mut rb,
                4 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
            let mut num_bits_height: core::ffi::c_int = aom_rb_read_literal(
                &raw mut rb,
                4 as core::ffi::c_int,
            ) + 1 as core::ffi::c_int;
            let mut max_frame_width: core::ffi::c_int = aom_rb_read_literal(
                &raw mut rb,
                num_bits_width,
            ) + 1 as core::ffi::c_int;
            let mut max_frame_height: core::ffi::c_int = aom_rb_read_literal(
                &raw mut rb,
                num_bits_height,
            ) + 1 as core::ffi::c_int;
            (*si).w = max_frame_width as core::ffi::c_uint;
            (*si).h = max_frame_height as core::ffi::c_uint;
            got_sequence_header = 1 as core::ffi::c_int;
        } else if obu_header.type_0 as core::ffi::c_int
            == OBU_FRAME_HEADER as core::ffi::c_int
            || obu_header.type_0 as core::ffi::c_int == OBU_FRAME as core::ffi::c_int
        {
            if got_sequence_header != 0
                && reduced_still_picture_hdr as core::ffi::c_int != 0
            {
                found_keyframe = 1 as core::ffi::c_int;
                break;
            } else {
                if data_sz < 1 as size_t {
                    return AOM_CODEC_CORRUPT_FRAME;
                }
                let mut rb_0: aom_read_bit_buffer = {
                    let mut init = aom_read_bit_buffer {
                        bit_buffer: data,
                        bit_buffer_end: data.offset(data_sz as isize),
                        bit_offset: 0 as uint32_t,
                        error_handler_data: NULL,
                        error_handler: None,
                    };
                    init
                };
                let show_existing_frame: core::ffi::c_int = aom_rb_read_bit(&raw mut rb_0)
                    as core::ffi::c_int;
                if show_existing_frame == 0 {
                    let frame_type: FRAME_TYPE = aom_rb_read_literal(
                        &raw mut rb_0,
                        2 as core::ffi::c_int,
                    ) as FRAME_TYPE;
                    if frame_type as core::ffi::c_int == KEY_FRAME as core::ffi::c_int {
                        found_keyframe = 1 as core::ffi::c_int;
                        break;
                    } else if frame_type as core::ffi::c_int
                        == INTRA_ONLY_FRAME as core::ffi::c_int
                    {
                        intra_only_flag = 1 as core::ffi::c_int;
                    }
                }
            }
        }
        data = data.offset(payload_size as isize);
        data_sz = data_sz.wrapping_sub(payload_size);
        if data_sz == 0 as size_t {
            break;
        }
        status = aom_read_obu_header_and_size(
            data,
            data_sz,
            (*si).is_annexb as core::ffi::c_int,
            &raw mut obu_header,
            &raw mut payload_size,
            &raw mut bytes_read,
        );
        if status as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return status;
        }
    }
    if got_sequence_header != 0 && found_keyframe != 0 {
        (*si).is_kf = 1 as core::ffi::c_uint;
    }
    if !is_intra_only.is_null() {
        *is_intra_only = intra_only_flag;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn decoder_peek_si(
    mut data: *const uint8_t,
    mut data_sz: size_t,
    mut si: *mut aom_codec_stream_info_t,
) -> aom_codec_err_t {
    return decoder_peek_si_internal(data, data_sz, si, 0 as *mut core::ffi::c_int);
}
unsafe extern "C-unwind" fn decoder_get_si(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut si: *mut aom_codec_stream_info_t,
) -> aom_codec_err_t {
    *si = (*ctx).si;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn set_error_detail(
    mut ctx: *mut aom_codec_alg_priv_t,
    error: *const core::ffi::c_char,
) {
    (*ctx).base.err_detail = error;
}
unsafe extern "C-unwind" fn update_error_state(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut error: *const aom_internal_error_info,
) -> aom_codec_err_t {
    if (*error).error_code as u64 != 0 {
        set_error_detail(
            ctx,
            if (*error).has_detail != 0 {
                ((*error).detail).as_ptr()
            } else {
                0 as *const core::ffi::c_char
            },
        );
    }
    return (*error).error_code;
}
unsafe extern "C-unwind" fn init_buffer_callbacks(mut ctx: *mut aom_codec_alg_priv_t) {
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    let frame_worker_data: *mut FrameWorkerData = (*worker).data1
        as *mut FrameWorkerData;
    let pbi: *mut AV1Decoder = (*frame_worker_data).pbi as *mut AV1Decoder;
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let pool: *mut BufferPool = (*cm).buffer_pool;
    (*cm).cur_frame = 0 as *mut RefCntBuffer;
    (*cm).features.byte_alignment = (*ctx).byte_alignment;
    (*pbi).skip_loop_filter = (*ctx).skip_loop_filter;
    (*pbi).skip_film_grain = (*ctx).skip_film_grain;
    if ((*ctx).get_ext_fb_cb).is_some() && ((*ctx).release_ext_fb_cb).is_some() {
        (*pool).get_fb_cb = (*ctx).get_ext_fb_cb;
        (*pool).release_fb_cb = (*ctx).release_ext_fb_cb;
        (*pool).cb_priv = (*ctx).ext_priv;
    } else {
        (*pool).get_fb_cb = Some(
            av1_get_frame_buffer
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *mut aom_codec_frame_buffer_t,
                ) -> core::ffi::c_int,
        ) as aom_get_frame_buffer_cb_fn_t;
        (*pool).release_fb_cb = Some(
            av1_release_frame_buffer
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut aom_codec_frame_buffer_t,
                ) -> core::ffi::c_int,
        ) as aom_release_frame_buffer_cb_fn_t;
        if av1_alloc_internal_frame_buffers(&raw mut (*pool).int_frame_buffers) != 0 {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to initialize internal frame buffers\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        (*pool).cb_priv = &raw mut (*pool).int_frame_buffers as *mut InternalFrameBufferList
            as *mut core::ffi::c_void;
    };
}
unsafe extern "C-unwind" fn frame_worker_hook(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let frame_worker_data: *mut FrameWorkerData = arg1 as *mut FrameWorkerData;
    let mut data: *const uint8_t = (*frame_worker_data).data;
    let mut result: core::ffi::c_int = av1_receive_compressed_data(
        (*frame_worker_data).pbi,
        (*frame_worker_data).data_size,
        &raw mut data,
    );
    (*frame_worker_data).data_end = data;
    if result != 0 as core::ffi::c_int {
        (*(*frame_worker_data).pbi).need_resync = 1 as core::ffi::c_int;
    }
    return (result == 0) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn init_decoder(
    mut ctx: *mut aom_codec_alg_priv_t,
) -> aom_codec_err_t {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    (*ctx).last_show_frame = 0 as *mut RefCntBuffer;
    (*ctx).need_resync = 1 as core::ffi::c_int;
    (*ctx).flushed = 0 as core::ffi::c_int;
    (*ctx).buffer_pool = aom_calloc(
        1 as size_t,
        ::core::mem::size_of::<BufferPool>() as size_t,
    ) as *mut BufferPool;
    if ((*ctx).buffer_pool).is_null() {
        return AOM_CODEC_MEM_ERROR;
    }
    (*(*ctx).buffer_pool).num_frame_bufs = (REF_FRAMES as core::ffi::c_int
        + 1 as core::ffi::c_int + INTER_REFS_PER_FRAME as core::ffi::c_int) as uint8_t;
    (*(*ctx).buffer_pool).frame_bufs = aom_calloc(
        (*(*ctx).buffer_pool).num_frame_bufs as size_t,
        ::core::mem::size_of::<RefCntBuffer>() as size_t,
    ) as *mut RefCntBuffer;
    if ((*(*ctx).buffer_pool).frame_bufs).is_null() {
        (*(*ctx).buffer_pool).num_frame_bufs = 0 as uint8_t;
        aom_free((*ctx).buffer_pool as *mut core::ffi::c_void);
        (*ctx).buffer_pool = 0 as *mut BufferPool;
        return AOM_CODEC_MEM_ERROR;
    }
    if pthread_mutex_init(
        &raw mut (*(*ctx).buffer_pool).pool_mutex,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        aom_free((*(*ctx).buffer_pool).frame_bufs as *mut core::ffi::c_void);
        (*(*ctx).buffer_pool).frame_bufs = 0 as *mut RefCntBuffer;
        (*(*ctx).buffer_pool).num_frame_bufs = 0 as uint8_t;
        aom_free((*ctx).buffer_pool as *mut core::ffi::c_void);
        (*ctx).buffer_pool = 0 as *mut BufferPool;
        set_error_detail(
            ctx,
            b"Failed to allocate buffer pool mutex\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AOM_CODEC_MEM_ERROR;
    }
    (*ctx).frame_worker = aom_malloc(::core::mem::size_of::<AVxWorker>() as size_t)
        as *mut AVxWorker;
    if ((*ctx).frame_worker).is_null() {
        set_error_detail(
            ctx,
            b"Failed to allocate frame_worker\0" as *const u8 as *const core::ffi::c_char,
        );
        return AOM_CODEC_MEM_ERROR;
    }
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    ((*winterface).init).expect("non-null function pointer")(worker);
    (*worker).thread_name = b"aom frameworker\0" as *const u8
        as *const core::ffi::c_char;
    (*worker).data1 = aom_memalign(
        32 as size_t,
        ::core::mem::size_of::<FrameWorkerData>() as size_t,
    );
    if ((*worker).data1).is_null() {
        ((*winterface).end).expect("non-null function pointer")(worker);
        aom_free(worker as *mut core::ffi::c_void);
        (*ctx).frame_worker = 0 as *mut AVxWorker;
        set_error_detail(
            ctx,
            b"Failed to allocate frame_worker_data\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AOM_CODEC_MEM_ERROR;
    }
    let mut frame_worker_data: *mut FrameWorkerData = (*worker).data1
        as *mut FrameWorkerData;
    (*frame_worker_data).pbi = av1_decoder_create((*ctx).buffer_pool);
    if ((*frame_worker_data).pbi).is_null() {
        ((*winterface).end).expect("non-null function pointer")(worker);
        aom_free(frame_worker_data as *mut core::ffi::c_void);
        aom_free(worker as *mut core::ffi::c_void);
        (*ctx).frame_worker = 0 as *mut AVxWorker;
        set_error_detail(
            ctx,
            b"Failed to allocate frame_worker_data->pbi\0" as *const u8
                as *const core::ffi::c_char,
        );
        return AOM_CODEC_MEM_ERROR;
    }
    (*frame_worker_data).frame_context_ready = 0 as core::ffi::c_int;
    (*frame_worker_data).received_frame = 0 as core::ffi::c_int;
    (*(*frame_worker_data).pbi).allow_lowbitdepth = (*ctx).cfg.allow_lowbitdepth
        as core::ffi::c_int;
    (*(*frame_worker_data).pbi).max_threads = (*ctx).cfg.threads as core::ffi::c_int;
    (*(*frame_worker_data).pbi).inv_tile_order = (*ctx).invert_tile_order;
    (*(*frame_worker_data).pbi).common.tiles.large_scale = (*ctx).tile_mode;
    (*(*frame_worker_data).pbi).is_annexb = (*ctx).is_annexb as core::ffi::c_int;
    (*(*frame_worker_data).pbi).dec_tile_row = (*ctx).decode_tile_row;
    (*(*frame_worker_data).pbi).dec_tile_col = (*ctx).decode_tile_col;
    (*(*frame_worker_data).pbi).operating_point = (*ctx).operating_point;
    (*(*frame_worker_data).pbi).output_all_layers = (*ctx).output_all_layers;
    (*(*frame_worker_data).pbi).frame_size_limit = (*ctx).frame_size_limit;
    (*(*frame_worker_data).pbi).ext_tile_debug = (*ctx).ext_tile_debug;
    (*(*frame_worker_data).pbi).row_mt = (*ctx).row_mt;
    (*(*frame_worker_data).pbi).is_fwd_kf_present = 0 as core::ffi::c_int;
    (*(*frame_worker_data).pbi).is_arf_frame_present = 0 as core::ffi::c_int;
    (*worker).hook = Some(
        frame_worker_hook
            as unsafe extern "C-unwind" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
            ) -> core::ffi::c_int,
    ) as AVxWorkerHook;
    init_buffer_callbacks(ctx);
    return AOM_CODEC_OK;
}
#[inline]
unsafe extern "C-unwind" fn check_resync(
    ctx: *mut aom_codec_alg_priv_t,
    pbi: *const AV1Decoder,
) {
    if (*ctx).need_resync == 1 as core::ffi::c_int
        && (*pbi).need_resync == 0 as core::ffi::c_int
        && frame_is_intra_only(&(*pbi).common) != 0
    {
        (*ctx).need_resync = 0 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn decode_one(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut data: *mut *const uint8_t,
    mut data_sz: size_t,
    mut user_priv: *mut core::ffi::c_void,
) -> aom_codec_err_t {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    if (*ctx).si.h == 0 {
        let mut is_intra_only: core::ffi::c_int = 0 as core::ffi::c_int;
        (*ctx).si.is_annexb = (*ctx).is_annexb;
        let res: aom_codec_err_t = decoder_peek_si_internal(
            *data,
            data_sz,
            &raw mut (*ctx).si,
            &raw mut is_intra_only,
        ) as aom_codec_err_t;
        if res as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return res;
        }
        if (*ctx).si.is_kf == 0 && is_intra_only == 0 {
            return AOM_CODEC_ERROR;
        }
    }
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    let frame_worker_data: *mut FrameWorkerData = (*worker).data1
        as *mut FrameWorkerData;
    (*frame_worker_data).data = *data;
    (*frame_worker_data).data_size = data_sz;
    (*frame_worker_data).user_priv = user_priv;
    (*frame_worker_data).received_frame = 1 as core::ffi::c_int;
    (*(*frame_worker_data).pbi).common.tiles.large_scale = (*ctx).tile_mode;
    (*(*frame_worker_data).pbi).dec_tile_row = (*ctx).decode_tile_row;
    (*(*frame_worker_data).pbi).dec_tile_col = (*ctx).decode_tile_col;
    (*(*frame_worker_data).pbi).ext_tile_debug = (*ctx).ext_tile_debug;
    (*(*frame_worker_data).pbi).row_mt = (*ctx).row_mt;
    (*(*frame_worker_data).pbi).ext_refs = (*ctx).ext_refs;
    (*(*frame_worker_data).pbi).is_annexb = (*ctx).is_annexb as core::ffi::c_int;
    (*worker).had_error = 0 as core::ffi::c_int;
    ((*winterface).execute).expect("non-null function pointer")(worker);
    *data = (*frame_worker_data).data_end;
    if (*worker).had_error != 0 {
        return update_error_state(ctx, &raw mut (*(*frame_worker_data).pbi).error);
    }
    check_resync(ctx, (*frame_worker_data).pbi);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn release_pending_output_frames(mut ctx: *mut aom_codec_alg_priv_t) {
    if !((*ctx).frame_worker).is_null() {
        let pool: *mut BufferPool = (*ctx).buffer_pool;
        lock_buffer_pool(pool);
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        let mut pbi: *mut AV1Decoder = (*frame_worker_data).pbi;
        let mut j: size_t = 0 as size_t;
        while j < (*pbi).num_output_frames {
            decrease_ref_count((*pbi).output_frames[j as usize], pool);
            j = j.wrapping_add(1);
        }
        (*pbi).num_output_frames = 0 as size_t;
        unlock_buffer_pool(pool);
        let mut j_0: size_t = 0 as size_t;
        while j_0 < (*ctx).num_grain_image_frame_buffers {
            ((*pool).release_fb_cb)
                .expect(
                    "non-null function pointer",
                )(
                (*pool).cb_priv,
                &raw mut *((*ctx).grain_image_frame_buffers)
                    .as_mut_ptr()
                    .offset(j_0 as isize),
            );
            (*ctx).grain_image_frame_buffers[j_0 as usize].data = 0 as *mut uint8_t;
            (*ctx).grain_image_frame_buffers[j_0 as usize].size = 0 as size_t;
            (*ctx).grain_image_frame_buffers[j_0 as usize].priv_0 = NULL;
            j_0 = j_0.wrapping_add(1);
        }
        (*ctx).num_grain_image_frame_buffers = 0 as size_t;
    }
}
unsafe extern "C-unwind" fn decoder_inspect(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut data: *const uint8_t,
    mut data_sz: size_t,
    mut user_priv: *mut core::ffi::c_void,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    release_pending_output_frames(ctx);
    if data.is_null() && data_sz == 0 as size_t {
        (*ctx).flushed = 1 as core::ffi::c_int;
        return AOM_CODEC_OK;
    }
    if data.is_null() || data_sz == 0 as size_t {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*ctx).flushed = 0 as core::ffi::c_int;
    let mut data_start: *const uint8_t = data;
    let mut data_end: *const uint8_t = data.offset(data_sz as isize);
    let mut frame_size: uint64_t = 0;
    if (*ctx).is_annexb != 0 {
        let mut length_of_size: size_t = 0;
        let mut temporal_unit_size: uint64_t = 0;
        if aom_uleb_decode(
            data_start,
            data_sz,
            &raw mut temporal_unit_size,
            &raw mut length_of_size,
        ) != 0 as core::ffi::c_int
        {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        data_start = data_start.offset(length_of_size as isize);
        if temporal_unit_size
            > data_end.offset_from(data_start) as core::ffi::c_long as uint64_t
        {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        data_end = data_start.offset(temporal_unit_size as isize);
        if aom_uleb_decode(
            data_start,
            data_end.offset_from(data_start) as core::ffi::c_long as size_t,
            &raw mut frame_size,
            &raw mut length_of_size,
        ) != 0 as core::ffi::c_int
        {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        data_start = data_start.offset(length_of_size as isize);
        if frame_size > data_end.offset_from(data_start) as core::ffi::c_long as uint64_t
        {
            return AOM_CODEC_CORRUPT_FRAME;
        }
    } else {
        frame_size = data_end.offset_from(data_start) as core::ffi::c_long as uint64_t;
    }
    if ((*ctx).frame_worker).is_null() {
        res = init_decoder(ctx);
        if res as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return res;
        }
    }
    let frame_worker_data: *mut FrameWorkerData = (*(*ctx).frame_worker).data1
        as *mut FrameWorkerData;
    let pbi: *mut AV1Decoder = (*frame_worker_data).pbi as *mut AV1Decoder;
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    res = av1_receive_compressed_data(
        (*frame_worker_data).pbi,
        frame_size as size_t,
        &raw mut data_start,
    ) as aom_codec_err_t;
    check_resync(ctx, (*frame_worker_data).pbi);
    if (*(*ctx).frame_worker).had_error != 0 {
        return update_error_state(ctx, &raw mut (*(*frame_worker_data).pbi).error);
    }
    while data_start < data_end {
        let marker: uint8_t = *data_start.offset(0 as core::ffi::c_int as isize);
        if marker != 0 {
            break;
        }
        data_start = data_start.offset(1);
    }
    let mut data2: *mut Av1DecodeReturn = user_priv as *mut Av1DecodeReturn;
    (*data2).idx = -(1 as core::ffi::c_int);
    if !((*cm).cur_frame).is_null() {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < REF_FRAMES as core::ffi::c_int {
            if (*cm).ref_frame_map[i as usize] == (*cm).cur_frame {
                (*data2).idx = i;
            }
            i += 1;
        }
    }
    (*data2).buf = data_start as *const core::ffi::c_uchar;
    (*data2).show_existing = (*cm).show_existing_frame;
    return res;
}
unsafe extern "C-unwind" fn decoder_decode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut data: *const uint8_t,
    mut data_sz: size_t,
    mut user_priv: *mut core::ffi::c_void,
) -> aom_codec_err_t {
    let mut res: aom_codec_err_t = AOM_CODEC_OK;
    release_pending_output_frames(ctx);
    if data.is_null() && data_sz == 0 as size_t {
        (*ctx).flushed = 1 as core::ffi::c_int;
        return AOM_CODEC_OK;
    }
    if data.is_null() || data_sz == 0 as size_t {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*ctx).flushed = 0 as core::ffi::c_int;
    if ((*ctx).frame_worker).is_null() {
        res = init_decoder(ctx);
        if res as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return res;
        }
    }
    let mut data_start: *const uint8_t = data;
    let mut data_end: *const uint8_t = data.offset(data_sz as isize);
    if (*ctx).is_annexb != 0 {
        let mut length_of_size: size_t = 0;
        let mut temporal_unit_size: uint64_t = 0;
        if aom_uleb_decode(
            data_start,
            data_sz,
            &raw mut temporal_unit_size,
            &raw mut length_of_size,
        ) != 0 as core::ffi::c_int
        {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        data_start = data_start.offset(length_of_size as isize);
        if temporal_unit_size
            > data_end.offset_from(data_start) as core::ffi::c_long as uint64_t
        {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        data_end = data_start.offset(temporal_unit_size as isize);
    }
    while data_start < data_end {
        let mut frame_size: uint64_t = 0;
        if (*ctx).is_annexb != 0 {
            let mut length_of_size_0: size_t = 0;
            if aom_uleb_decode(
                data_start,
                data_end.offset_from(data_start) as core::ffi::c_long as size_t,
                &raw mut frame_size,
                &raw mut length_of_size_0,
            ) != 0 as core::ffi::c_int
            {
                return AOM_CODEC_CORRUPT_FRAME;
            }
            data_start = data_start.offset(length_of_size_0 as isize);
            if frame_size
                > data_end.offset_from(data_start) as core::ffi::c_long as uint64_t
            {
                return AOM_CODEC_CORRUPT_FRAME;
            }
        } else {
            frame_size = data_end.offset_from(data_start) as core::ffi::c_long
                as uint64_t;
        }
        res = decode_one(ctx, &raw mut data_start, frame_size as size_t, user_priv);
        if res as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return res;
        }
        while data_start < data_end {
            let marker: uint8_t = *data_start.offset(0 as core::ffi::c_int as isize);
            if marker != 0 {
                break;
            }
            data_start = data_start.offset(1);
        }
    }
    return res;
}
unsafe extern "C-unwind" fn AllocWithGetFrameBufferCb(
    mut priv_0: *mut core::ffi::c_void,
    mut size: size_t,
) -> *mut core::ffi::c_void {
    let mut param: *mut AllocCbParam = priv_0 as *mut AllocCbParam;
    if ((*(*param).pool).get_fb_cb)
        .expect("non-null function pointer")((*(*param).pool).cb_priv, size, (*param).fb)
        < 0 as core::ffi::c_int
    {
        return NULL;
    }
    if ((*(*param).fb).data).is_null() || (*(*param).fb).size < size {
        return NULL;
    }
    return (*(*param).fb).data as *mut core::ffi::c_void;
}
unsafe extern "C-unwind" fn add_grain_if_needed(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut img: *mut aom_image_t,
    mut grain_img: *mut aom_image_t,
    mut grain_params: *mut aom_film_grain_t,
) -> *mut aom_image_t {
    if (*grain_params).apply_grain == 0 {
        return img;
    }
    let w_even: core::ffi::c_int = (((*img).d_w)
        .wrapping_add(
            ((1 as core::ffi::c_uint) << 1 as core::ffi::c_int)
                .wrapping_sub(1 as core::ffi::c_uint),
        )
        & !((1 as core::ffi::c_uint) << 1 as core::ffi::c_int)
            .wrapping_sub(1 as core::ffi::c_uint)) as core::ffi::c_int;
    let h_even: core::ffi::c_int = (((*img).d_h)
        .wrapping_add(
            ((1 as core::ffi::c_uint) << 1 as core::ffi::c_int)
                .wrapping_sub(1 as core::ffi::c_uint),
        )
        & !((1 as core::ffi::c_uint) << 1 as core::ffi::c_int)
            .wrapping_sub(1 as core::ffi::c_uint)) as core::ffi::c_int;
    let pool: *mut BufferPool = (*ctx).buffer_pool;
    let mut fb: *mut aom_codec_frame_buffer_t = &raw mut *((*ctx).grain_image_frame_buffers)
        .as_mut_ptr()
        .offset((*ctx).num_grain_image_frame_buffers as isize)
        as *mut aom_codec_frame_buffer_t;
    let mut param: AllocCbParam = AllocCbParam {
        pool: 0 as *mut BufferPool,
        fb: 0 as *mut aom_codec_frame_buffer_t,
    };
    param.pool = pool;
    param.fb = fb;
    if (aom_img_alloc_with_cb(
        grain_img,
        (*img).fmt,
        w_even as core::ffi::c_uint,
        h_even as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        Some(
            AllocWithGetFrameBufferCb
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    size_t,
                ) -> *mut core::ffi::c_void,
        ),
        &raw mut param as *mut AllocCbParam as *mut core::ffi::c_void,
    ))
        .is_null()
    {
        return 0 as *mut aom_image_t;
    }
    (*grain_img).user_priv = (*img).user_priv;
    (*grain_img).fb_priv = (*fb).priv_0;
    if av1_add_film_grain(grain_params, img, grain_img) != 0 {
        ((*pool).release_fb_cb).expect("non-null function pointer")((*pool).cb_priv, fb);
        return 0 as *mut aom_image_t;
    }
    (*ctx).num_grain_image_frame_buffers = ((*ctx).num_grain_image_frame_buffers)
        .wrapping_add(1);
    return grain_img;
}
unsafe extern "C-unwind" fn move_decoder_metadata_to_img(
    mut pbi: *mut AV1Decoder,
    mut img: *mut aom_image_t,
) {
    if !((*pbi).metadata).is_null() && !img.is_null() {
        (*img).metadata = (*pbi).metadata;
        (*pbi).metadata = 0 as *mut aom_metadata_array_t;
    }
}
unsafe extern "C-unwind" fn decoder_get_frame(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut iter: *mut aom_codec_iter_t,
) -> *mut aom_image_t {
    let mut img: *mut aom_image_t = 0 as *mut aom_image_t;
    if iter.is_null() {
        return 0 as *mut aom_image_t;
    }
    let mut index: *mut uintptr_t = iter as *mut uintptr_t;
    if ((*ctx).frame_worker).is_null() {
        return 0 as *mut aom_image_t;
    }
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    let frame_worker_data: *mut FrameWorkerData = (*worker).data1
        as *mut FrameWorkerData;
    let pbi: *mut AV1Decoder = (*frame_worker_data).pbi as *mut AV1Decoder;
    (*pbi).error.error_code = AOM_CODEC_OK;
    (*pbi).error.has_detail = 0 as core::ffi::c_int;
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    if ((*winterface).sync).expect("non-null function pointer")(worker) == 0 {
        (*frame_worker_data).received_frame = 0 as core::ffi::c_int;
        (*ctx).need_resync = 1 as core::ffi::c_int;
        return 0 as *mut aom_image_t;
    }
    if (*frame_worker_data).received_frame == 1 as core::ffi::c_int {
        (*frame_worker_data).received_frame = 0 as core::ffi::c_int;
        check_resync(ctx, (*frame_worker_data).pbi);
    }
    let mut sd: *mut YV12_BUFFER_CONFIG = 0 as *mut YV12_BUFFER_CONFIG;
    let mut grain_params: *mut aom_film_grain_t = 0 as *mut aom_film_grain_t;
    if av1_get_raw_frame(
        (*frame_worker_data).pbi as *mut AV1Decoder,
        *index,
        &raw mut sd,
        &raw mut grain_params,
    ) != 0 as core::ffi::c_int
    {
        return 0 as *mut aom_image_t;
    }
    let output_frame_buf: *mut RefCntBuffer = (*pbi).output_frames[*index as usize];
    (*ctx).last_show_frame = output_frame_buf;
    if (*ctx).need_resync != 0 {
        return 0 as *mut aom_image_t;
    }
    aom_img_remove_metadata(&raw mut (*ctx).img);
    yuvconfig2image(&raw mut (*ctx).img, sd, (*frame_worker_data).user_priv);
    move_decoder_metadata_to_img(pbi, &raw mut (*ctx).img);
    if (*pbi).ext_tile_debug == 0 && (*tiles).large_scale != 0 {
        *index = (*index).wrapping_add(1 as uintptr_t);
        aom_img_remove_metadata(&raw mut (*ctx).img);
        yuvconfig2image(&raw mut (*ctx).img, &raw mut (*pbi).tile_list_outbuf, NULL);
        move_decoder_metadata_to_img(pbi, &raw mut (*ctx).img);
        img = &raw mut (*ctx).img;
        return img;
    }
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    if (*pbi).ext_tile_debug != 0 && (*tiles).single_tile_decoding != 0
        && (*pbi).dec_tile_row >= 0 as core::ffi::c_int
    {
        let mut tile_width: core::ffi::c_int = 0;
        let mut tile_height: core::ffi::c_int = 0;
        if !av1_get_uniform_tile_size(cm, &raw mut tile_width, &raw mut tile_height) {
            return 0 as *mut aom_image_t;
        }
        let tile_row: core::ffi::c_int = if (*pbi).dec_tile_row
            < (*tiles).rows - 1 as core::ffi::c_int
        {
            (*pbi).dec_tile_row
        } else {
            (*tiles).rows - 1 as core::ffi::c_int
        };
        let mi_row: core::ffi::c_int = tile_row * tile_height;
        let ssy: core::ffi::c_int = (*ctx).img.y_chroma_shift as core::ffi::c_int;
        let mut plane: core::ffi::c_int = 0;
        (*ctx).img.planes[0 as core::ffi::c_int as usize] = ((*ctx)
            .img
            .planes[0 as core::ffi::c_int as usize])
            .offset(
                (mi_row * MI_SIZE * (*ctx).img.stride[0 as core::ffi::c_int as usize])
                    as isize,
            );
        if num_planes > 1 as core::ffi::c_int {
            plane = 1 as core::ffi::c_int;
            while plane < MAX_MB_PLANE {
                (*ctx).img.planes[plane as usize] = ((*ctx).img.planes[plane as usize])
                    .offset(
                        (mi_row * (MI_SIZE >> ssy) * (*ctx).img.stride[plane as usize])
                            as isize,
                    );
                plane += 1;
            }
        }
        (*ctx).img.d_h = ((if tile_height < (*cm).mi_params.mi_rows - mi_row {
            tile_height
        } else {
            (*cm).mi_params.mi_rows - mi_row
        }) * MI_SIZE) as core::ffi::c_uint;
    }
    if (*pbi).ext_tile_debug != 0 && (*tiles).single_tile_decoding != 0
        && (*pbi).dec_tile_col >= 0 as core::ffi::c_int
    {
        let mut tile_width_0: core::ffi::c_int = 0;
        let mut tile_height_0: core::ffi::c_int = 0;
        if !av1_get_uniform_tile_size(cm, &raw mut tile_width_0, &raw mut tile_height_0) {
            return 0 as *mut aom_image_t;
        }
        let tile_col: core::ffi::c_int = if (*pbi).dec_tile_col
            < (*tiles).cols - 1 as core::ffi::c_int
        {
            (*pbi).dec_tile_col
        } else {
            (*tiles).cols - 1 as core::ffi::c_int
        };
        let mi_col: core::ffi::c_int = tile_col * tile_width_0;
        let ssx: core::ffi::c_int = (*ctx).img.x_chroma_shift as core::ffi::c_int;
        let is_hbd: core::ffi::c_int = if (*ctx).img.fmt as core::ffi::c_uint
            & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint != 0
        {
            1 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        let mut plane_0: core::ffi::c_int = 0;
        (*ctx).img.planes[0 as core::ffi::c_int as usize] = ((*ctx)
            .img
            .planes[0 as core::ffi::c_int as usize])
            .offset((mi_col * MI_SIZE * (1 as core::ffi::c_int + is_hbd)) as isize);
        if num_planes > 1 as core::ffi::c_int {
            plane_0 = 1 as core::ffi::c_int;
            while plane_0 < MAX_MB_PLANE {
                (*ctx).img.planes[plane_0 as usize] = ((*ctx)
                    .img
                    .planes[plane_0 as usize])
                    .offset(
                        (mi_col * (MI_SIZE >> ssx) * (1 as core::ffi::c_int + is_hbd))
                            as isize,
                    );
                plane_0 += 1;
            }
        }
        (*ctx).img.d_w = ((if tile_width_0 < (*cm).mi_params.mi_cols - mi_col {
            tile_width_0
        } else {
            (*cm).mi_params.mi_cols - mi_col
        }) * MI_SIZE) as core::ffi::c_uint;
    }
    (*ctx).img.fb_priv = (*output_frame_buf).raw_frame_buffer.priv_0;
    img = &raw mut (*ctx).img;
    (*img).temporal_id = (*output_frame_buf).temporal_id;
    (*img).spatial_id = (*output_frame_buf).spatial_id;
    if (*pbi).skip_film_grain != 0 {
        (*grain_params).apply_grain = 0 as core::ffi::c_int;
    }
    let mut res: *mut aom_image_t = add_grain_if_needed(
        ctx,
        img,
        &raw mut (*ctx).image_with_grain,
        grain_params,
    );
    if res.is_null() {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        (*pbi).error.has_detail = 1 as core::ffi::c_int;
        snprintf(
            ((*pbi).error.detail).as_mut_ptr(),
            ::core::mem::size_of::<[core::ffi::c_char; 200]>() as size_t,
            b"Grain synthesis failed\n\0" as *const u8 as *const core::ffi::c_char,
        );
        return res;
    }
    *index = (*index).wrapping_add(1 as uintptr_t);
    return res;
}
unsafe extern "C-unwind" fn decoder_set_fb_fn(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut cb_get: aom_get_frame_buffer_cb_fn_t,
    mut cb_release: aom_release_frame_buffer_cb_fn_t,
    mut cb_priv: *mut core::ffi::c_void,
) -> aom_codec_err_t {
    if cb_get.is_none() || cb_release.is_none() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if !((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    (*ctx).get_ext_fb_cb = cb_get;
    (*ctx).release_ext_fb_cb = cb_release;
    (*ctx).ext_priv = cb_priv;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let data: *mut av1_ref_frame_t = args.next_arg::<*mut av1_ref_frame_t>();
    if !data.is_null() {
        let frame: *mut av1_ref_frame_t = data;
        let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        if worker.is_null() {
            return AOM_CODEC_ERROR;
        }
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        image2yuvconfig(&raw mut (*frame).img, &raw mut sd);
        let error: *mut aom_internal_error_info = (*(*frame_worker_data).pbi)
            .common
            .error;
        loop {
match crate::jump::value_scope((((*error).jmp).as_mut_ptr()).cast(), || {
        (*error).setjmp = 1 as core::ffi::c_int;
        av1_set_reference_dec(
            &raw mut (*(*frame_worker_data).pbi).common,
            (*frame).idx,
            (*frame).use_external_ref,
            &raw mut sd,
        );
        (*error).setjmp = 0 as core::ffi::c_int;
        return AOM_CODEC_OK;}) {
Some(value) => return value,
None => {
            (*error).setjmp = 0 as core::ffi::c_int;
            return (*error).error_code;}
}
}} else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_copy_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let frame: *const av1_ref_frame_t = args.next_arg::<*mut av1_ref_frame_t>();
    if !frame.is_null() {
        let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        if worker.is_null() {
            return AOM_CODEC_ERROR;
        }
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        image2yuvconfig(&(*frame).img, &raw mut sd);
        return av1_copy_reference_dec((*frame_worker_data).pbi, (*frame).idx, &raw mut sd);
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_reference(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let mut data: *mut av1_ref_frame_t = args.next_arg::<*mut av1_ref_frame_t>();
    if !data.is_null() {
        let mut fb: *mut YV12_BUFFER_CONFIG = 0 as *mut YV12_BUFFER_CONFIG;
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        if worker.is_null() {
            return AOM_CODEC_ERROR;
        }
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        fb = get_ref_frame(&raw mut (*(*frame_worker_data).pbi).common, (*data).idx);
        if fb.is_null() {
            return AOM_CODEC_ERROR;
        }
        yuvconfig2image(&raw mut (*data).img, fb, NULL);
        return AOM_CODEC_OK;
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_new_frame_image(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let mut new_img: *mut aom_image_t = args.next_arg::<*mut aom_image_t>();
    if !new_img.is_null() {
        let mut new_frame: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        if worker.is_null() {
            return AOM_CODEC_ERROR;
        }
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        if av1_get_frame_to_show((*frame_worker_data).pbi, &raw mut new_frame)
            == 0 as core::ffi::c_int
        {
            yuvconfig2image(new_img, &raw mut new_frame, NULL);
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_copy_new_frame_image(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let mut img: *mut aom_image_t = args.next_arg::<*mut aom_image_t>();
    if !img.is_null() {
        let mut new_frame: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        if worker.is_null() {
            return AOM_CODEC_ERROR;
        }
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        if av1_get_frame_to_show((*frame_worker_data).pbi, &raw mut new_frame)
            == 0 as core::ffi::c_int
        {
            let mut sd: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
            image2yuvconfig(img, &raw mut sd);
            return av1_copy_new_frame_dec(
                &raw mut (*(*frame_worker_data).pbi).common,
                &raw mut new_frame,
                &raw mut sd,
            );
        } else {
            return AOM_CODEC_ERROR
        }
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_last_ref_updates(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let update_info: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if !update_info.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            *update_info = (*(*frame_worker_data).pbi)
                .common
                .current_frame
                .refresh_frame_flags;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_last_quantizer(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    *arg = (*(*((*(*ctx).frame_worker).data1 as *mut FrameWorkerData)).pbi)
        .common
        .quant_params
        .base_qindex;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_fwd_kf_value(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    *arg = (*(*((*(*ctx).frame_worker).data1 as *mut FrameWorkerData)).pbi)
        .is_fwd_kf_present;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_altref_present(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    *arg = (*(*((*(*ctx).frame_worker).data1 as *mut FrameWorkerData)).pbi)
        .is_arf_frame_present;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_frame_flags(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    let mut pbi: *mut AV1Decoder = (*((*(*ctx).frame_worker).data1
        as *mut FrameWorkerData))
        .pbi as *mut AV1Decoder;
    *arg = 0 as core::ffi::c_int;
    match (*pbi).common.current_frame.frame_type as core::ffi::c_int {
        0 => {
            *arg = (*arg as core::ffi::c_uint | AOM_FRAME_IS_KEY) as core::ffi::c_int;
            *arg = (*arg as core::ffi::c_uint | AOM_FRAME_IS_INTRAONLY)
                as core::ffi::c_int;
            if (*pbi).common.show_frame == 0 {
                *arg = (*arg as core::ffi::c_uint
                    | AOM_FRAME_IS_DELAYED_RANDOM_ACCESS_POINT) as core::ffi::c_int;
            }
        }
        2 => {
            *arg = (*arg as core::ffi::c_uint | AOM_FRAME_IS_INTRAONLY)
                as core::ffi::c_int;
        }
        3 => {
            *arg = (*arg as core::ffi::c_uint | AOM_FRAME_IS_SWITCH) as core::ffi::c_int;
        }
        _ => {}
    }
    if (*pbi).common.features.error_resilient_mode {
        *arg = (*arg as core::ffi::c_uint | AOM_FRAME_IS_ERROR_RESILIENT)
            as core::ffi::c_int;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_tile_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let tile_info: *mut aom_tile_info = args.next_arg::<*mut aom_tile_info>();
    if !tile_info.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            let mut tiles: *const CommonTileParams = &(*pbi).common.tiles;
            let mut tile_rows: core::ffi::c_int = (*tiles).rows;
            let mut tile_cols: core::ffi::c_int = (*tiles).cols;
            if (*tiles).uniform_spacing != 0 {
                (*tile_info).tile_rows = (1 as core::ffi::c_int) << (*tiles).log2_rows;
                (*tile_info).tile_columns = (1 as core::ffi::c_int)
                    << (*tiles).log2_cols;
            } else {
                (*tile_info).tile_rows = tile_rows;
                (*tile_info).tile_columns = tile_cols;
            }
            let mut tile_col: core::ffi::c_int = 1 as core::ffi::c_int;
            while tile_col <= tile_cols {
                (*tile_info).tile_widths[(tile_col - 1 as core::ffi::c_int) as usize] = (*tiles)
                    .col_start_sb[tile_col as usize]
                    - (*tiles).col_start_sb[(tile_col - 1 as core::ffi::c_int) as usize];
                tile_col += 1;
            }
            let mut tile_row: core::ffi::c_int = 1 as core::ffi::c_int;
            while tile_row <= tile_rows {
                (*tile_info).tile_heights[(tile_row - 1 as core::ffi::c_int) as usize] = (*tiles)
                    .row_start_sb[tile_row as usize]
                    - (*tiles).row_start_sb[(tile_row - 1 as core::ffi::c_int) as usize];
                tile_row += 1;
            }
            (*tile_info).num_tile_groups = (*pbi).num_tile_groups;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_screen_content_tools_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let sc_info: *mut aom_screen_content_tools_info = args
        .next_arg::<*mut aom_screen_content_tools_info>();
    if !sc_info.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            (*sc_info).allow_screen_content_tools = (*pbi)
                .common
                .features
                .allow_screen_content_tools as core::ffi::c_int;
            (*sc_info).allow_intrabc = (*pbi).common.features.allow_intrabc
                as core::ffi::c_int;
            (*sc_info).force_integer_mv = (*pbi)
                .common
                .features
                .cur_frame_force_integer_mv as core::ffi::c_int;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_still_picture(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let still_picture_info: *mut aom_still_picture_info = args
        .next_arg::<*mut aom_still_picture_info>();
    if !still_picture_info.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            (*still_picture_info).is_still_picture = (*pbi).seq_params.still_picture
                as core::ffi::c_int;
            (*still_picture_info).is_reduced_still_picture_hdr = (*pbi)
                .seq_params
                .reduced_still_picture_hdr as core::ffi::c_int;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_sb_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let sb_size: *mut aom_superblock_size_t = args.next_arg::<*mut aom_superblock_size_t>();
    if !sb_size.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            if (*pbi).seq_params.sb_size as core::ffi::c_int
                == BLOCK_128X128 as core::ffi::c_int
            {
                *sb_size = AOM_SUPERBLOCK_SIZE_128X128;
            } else {
                *sb_size = AOM_SUPERBLOCK_SIZE_64X64;
            }
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_show_existing_frame_flag(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    *arg = (*(*((*(*ctx).frame_worker).data1 as *mut FrameWorkerData)).pbi)
        .common
        .show_existing_frame;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_s_frame_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let s_frame_info: *mut aom_s_frame_info = args.next_arg::<*mut aom_s_frame_info>();
    if !s_frame_info.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            (*s_frame_info).is_s_frame = (*pbi).sframe_info.is_s_frame;
            (*s_frame_info).is_s_frame_at_altref = (*pbi)
                .sframe_info
                .is_s_frame_at_altref;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_frame_corrupted(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let mut corrupted: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if !corrupted.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let pbi: *mut AV1Decoder = (*frame_worker_data).pbi as *mut AV1Decoder;
            if (*pbi).seen_frame_header != 0 && (*pbi).num_output_frames == 0 as size_t {
                return AOM_CODEC_ERROR;
            }
            if !((*ctx).last_show_frame).is_null() {
                *corrupted = (*(*ctx).last_show_frame).buf.corrupted;
            }
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_frame_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let frame_size: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if !frame_size.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let cm: *const AV1_COMMON = &raw mut (*(*frame_worker_data).pbi).common;
            *frame_size.offset(0 as core::ffi::c_int as isize) = (*cm).width;
            *frame_size.offset(1 as core::ffi::c_int as isize) = (*cm).height;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_frame_header_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let frame_header_info: *mut aom_tile_data = args.next_arg::<*mut aom_tile_data>();
    if !frame_header_info.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            (*frame_header_info).coded_tile_data_size = (*pbi).obu_size_hdr.size;
            (*frame_header_info).coded_tile_data = (*pbi).obu_size_hdr.data
                as *const core::ffi::c_void;
            (*frame_header_info).extra_size = (*pbi).frame_header_size;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_tile_data(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let tile_data: *mut aom_tile_data = args.next_arg::<*mut aom_tile_data>();
    if !tile_data.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let mut pbi: *const AV1Decoder = (*frame_worker_data).pbi;
            if (*pbi).dec_tile_row < 0 as core::ffi::c_int
                || (*pbi).dec_tile_row >= MAX_TILE_ROWS
                || (*pbi).dec_tile_col < 0 as core::ffi::c_int
                || (*pbi).dec_tile_col >= MAX_TILE_COLS
            {
                return AOM_CODEC_ERROR;
            }
            (*tile_data).coded_tile_data_size = (*pbi)
                .tile_buffers[(*pbi).dec_tile_row as usize][(*pbi).dec_tile_col as usize]
                .size;
            (*tile_data).coded_tile_data = (*pbi)
                .tile_buffers[(*pbi).dec_tile_row as usize][(*pbi).dec_tile_col as usize]
                .data as *const core::ffi::c_void;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_set_ext_ref_ptr(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let data: *mut av1_ext_ref_frame_t = args.next_arg::<*mut av1_ext_ref_frame_t>();
    if !data.is_null() {
        let ext_frames: *mut av1_ext_ref_frame_t = data;
        (*ctx).ext_refs.num = (*ext_frames).num;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*ctx).ext_refs.num {
            let fresh0 = (*ext_frames).img;
            (*ext_frames).img = ((*ext_frames).img).offset(1);
            image2yuvconfig(
                fresh0,
                &raw mut *((*ctx).ext_refs.refs).as_mut_ptr().offset(i as isize),
            );
            i += 1;
        }
        return AOM_CODEC_OK;
    } else {
        return AOM_CODEC_INVALID_PARAM
    };
}
unsafe extern "C-unwind" fn ctrl_get_render_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let render_size: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if !render_size.is_null() {
        if !((*ctx).frame_worker).is_null() {
            let worker: *mut AVxWorker = (*ctx).frame_worker;
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let cm: *const AV1_COMMON = &raw mut (*(*frame_worker_data).pbi).common;
            *render_size.offset(0 as core::ffi::c_int as isize) = (*cm).render_width;
            *render_size.offset(1 as core::ffi::c_int as isize) = (*cm).render_height;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_bit_depth(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let bit_depth: *mut core::ffi::c_uint = args.next_arg::<*mut core::ffi::c_uint>();
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    if !bit_depth.is_null() {
        if !worker.is_null() {
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let cm: *const AV1_COMMON = &raw mut (*(*frame_worker_data).pbi).common;
            *bit_depth = (*(*cm).seq_params).bit_depth as core::ffi::c_uint;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn get_img_format(
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut use_highbitdepth: core::ffi::c_int,
) -> aom_img_fmt_t {
    let mut fmt: aom_img_fmt_t = AOM_IMG_FMT_NONE;
    if subsampling_x == 0 as core::ffi::c_int && subsampling_y == 0 as core::ffi::c_int {
        fmt = AOM_IMG_FMT_I444;
    } else if subsampling_x == 1 as core::ffi::c_int
        && subsampling_y == 0 as core::ffi::c_int
    {
        fmt = AOM_IMG_FMT_I422;
    } else if subsampling_x == 1 as core::ffi::c_int
        && subsampling_y == 1 as core::ffi::c_int
    {
        fmt = AOM_IMG_FMT_I420;
    }
    if use_highbitdepth != 0 {
        fmt = ::core::mem::transmute::<
            core::ffi::c_uint,
            aom_img_fmt_t,
        >(fmt as core::ffi::c_uint | AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint);
    }
    return fmt;
}
unsafe extern "C-unwind" fn ctrl_get_img_format(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let img_fmt: *mut aom_img_fmt_t = args.next_arg::<*mut aom_img_fmt_t>();
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    if !img_fmt.is_null() {
        if !worker.is_null() {
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let cm: *const AV1_COMMON = &raw mut (*(*frame_worker_data).pbi).common;
            *img_fmt = get_img_format(
                (*(*cm).seq_params).subsampling_x,
                (*(*cm).seq_params).subsampling_y,
                (*(*cm).seq_params).use_highbitdepth as core::ffi::c_int,
            );
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_tile_size(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let tile_size: *mut core::ffi::c_uint = args.next_arg::<*mut core::ffi::c_uint>();
    let worker: *mut AVxWorker = (*ctx).frame_worker;
    if !tile_size.is_null() {
        if !worker.is_null() {
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            let cm: *const AV1_COMMON = &raw mut (*(*frame_worker_data).pbi).common;
            let mut tile_width: core::ffi::c_int = 0;
            let mut tile_height: core::ffi::c_int = 0;
            if !av1_get_uniform_tile_size(
                cm as *const AV1Common,
                &raw mut tile_width,
                &raw mut tile_height,
            ) {
                return AOM_CODEC_CORRUPT_FRAME;
            }
            *tile_size = ((tile_width * MI_SIZE << 16 as core::ffi::c_int)
                + tile_height * MI_SIZE) as core::ffi::c_uint;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_tile_count(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let tile_count: *mut core::ffi::c_uint = args.next_arg::<*mut core::ffi::c_uint>();
    if !tile_count.is_null() {
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        if !worker.is_null() {
            let frame_worker_data: *mut FrameWorkerData = (*worker).data1
                as *mut FrameWorkerData;
            *tile_count = ((*(*frame_worker_data).pbi).tile_count_minus_1
                + 1 as core::ffi::c_int) as core::ffi::c_uint;
            return AOM_CODEC_OK;
        } else {
            return AOM_CODEC_ERROR
        }
    }
    return AOM_CODEC_INVALID_PARAM;
}
unsafe extern "C-unwind" fn ctrl_get_base_q_idx(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    let frame_worker_data: *mut FrameWorkerData = (*(*ctx).frame_worker).data1
        as *mut FrameWorkerData;
    *arg = (*(*frame_worker_data).pbi).common.quant_params.base_qindex;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_show_frame_flag(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_int = args.next_arg::<*mut core::ffi::c_int>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    let frame_worker_data: *mut FrameWorkerData = (*(*ctx).frame_worker).data1
        as *mut FrameWorkerData;
    *arg = (*(*frame_worker_data).pbi).common.show_frame;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_order_hint(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let arg: *mut core::ffi::c_uint = args.next_arg::<*mut core::ffi::c_uint>();
    if arg.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    let frame_worker_data: *mut FrameWorkerData = (*(*ctx).frame_worker).data1
        as *mut FrameWorkerData;
    *arg = (*(*frame_worker_data).pbi).common.current_frame.order_hint;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_mi_info(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let mut mi_row: core::ffi::c_int = args.next_arg::<core::ffi::c_int>();
    let mut mi_col: core::ffi::c_int = args.next_arg::<core::ffi::c_int>();
    let mut mi: *mut MB_MODE_INFO = args.next_arg::<*mut MB_MODE_INFO>();
    if mi.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if ((*ctx).frame_worker).is_null() {
        return AOM_CODEC_ERROR;
    }
    let frame_worker_data: *mut FrameWorkerData = (*(*ctx).frame_worker).data1
        as *mut FrameWorkerData;
    if frame_worker_data.is_null() {
        return AOM_CODEC_ERROR;
    }
    let mut cm: *mut AV1_COMMON = &raw mut (*(*frame_worker_data).pbi).common;
    let mi_rows: core::ffi::c_int = (*cm).mi_params.mi_rows;
    let mi_cols: core::ffi::c_int = (*cm).mi_params.mi_cols;
    let mi_stride: core::ffi::c_int = (*cm).mi_params.mi_stride;
    let offset: core::ffi::c_int = mi_row * mi_stride + mi_col;
    if mi_row < 0 as core::ffi::c_int || mi_row >= mi_rows
        || mi_col < 0 as core::ffi::c_int || mi_col >= mi_cols
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    *mi = **((*cm).mi_params.mi_grid_base).offset(offset as isize);
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_invert_tile_order(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).invert_tile_order = args.next_arg::<core::ffi::c_int>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_byte_alignment(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    let legacy_byte_alignment: core::ffi::c_int = 0 as core::ffi::c_int;
    let min_byte_alignment: core::ffi::c_int = 32 as core::ffi::c_int;
    let max_byte_alignment: core::ffi::c_int = 1024 as core::ffi::c_int;
    let byte_alignment: core::ffi::c_int = args.next_arg::<core::ffi::c_int>();
    if byte_alignment != legacy_byte_alignment
        && (byte_alignment < min_byte_alignment || byte_alignment > max_byte_alignment
            || byte_alignment & byte_alignment - 1 as core::ffi::c_int
                != 0 as core::ffi::c_int)
    {
        return AOM_CODEC_INVALID_PARAM;
    }
    (*ctx).byte_alignment = byte_alignment;
    if !((*ctx).frame_worker).is_null() {
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        (*(*frame_worker_data).pbi).common.features.byte_alignment = byte_alignment;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_skip_loop_filter(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).skip_loop_filter = args.next_arg::<core::ffi::c_int>();
    if !((*ctx).frame_worker).is_null() {
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        (*(*frame_worker_data).pbi).skip_loop_filter = (*ctx).skip_loop_filter;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_skip_film_grain(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).skip_film_grain = args.next_arg::<core::ffi::c_int>();
    if !((*ctx).frame_worker).is_null() {
        let worker: *mut AVxWorker = (*ctx).frame_worker;
        let frame_worker_data: *mut FrameWorkerData = (*worker).data1
            as *mut FrameWorkerData;
        (*(*frame_worker_data).pbi).skip_film_grain = (*ctx).skip_film_grain;
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_frame_size_limit(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).frame_size_limit = args.next_arg::<core::ffi::c_uint>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_get_accounting(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    return AOM_CODEC_INCAPABLE;
}
unsafe extern "C-unwind" fn ctrl_set_decode_tile_row(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).decode_tile_row = args.next_arg::<core::ffi::c_int>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_decode_tile_col(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).decode_tile_col = args.next_arg::<core::ffi::c_int>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_tile_mode(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).tile_mode = args.next_arg::<core::ffi::c_uint>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_is_annexb(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).is_annexb = args.next_arg::<core::ffi::c_uint>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_operating_point(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).operating_point = args.next_arg::<core::ffi::c_int>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_output_all_layers(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).output_all_layers = args.next_arg::<core::ffi::c_int>();
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_inspection_callback(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    return AOM_CODEC_INCAPABLE;
}
unsafe extern "C-unwind" fn ctrl_ext_tile_debug(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).ext_tile_debug = args.next_arg::<core::ffi::c_int>() as core::ffi::c_uint;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn ctrl_set_row_mt(
    mut ctx: *mut aom_codec_alg_priv_t,
    mut args: ::core::ffi::VaList,
) -> aom_codec_err_t {
    (*ctx).row_mt = args.next_arg::<core::ffi::c_uint>();
    return AOM_CODEC_OK;
}
static mut decoder_ctrl_maps: [aom_codec_ctrl_fn_map_t; 46] = unsafe {
    [
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_COPY_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_copy_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_INVERT_TILE_DECODE_ORDER as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_invert_tile_order
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_BYTE_ALIGNMENT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_byte_alignment
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_SKIP_LOOP_FILTER as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_skip_loop_filter
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_DECODE_TILE_ROW as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_decode_tile_row
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_DECODE_TILE_COL as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_decode_tile_col
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_TILE_MODE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_tile_mode
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_SET_IS_ANNEXB as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_is_annexb
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_SET_OPERATING_POINT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_operating_point
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_SET_OUTPUT_ALL_LAYERS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_output_all_layers
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_SET_INSPECTION_CALLBACK as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_inspection_callback
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_EXT_TILE_DEBUG as core::ffi::c_int,
                fn_0: Some(
                    ctrl_ext_tile_debug
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_SET_ROW_MT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_row_mt
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_SET_EXT_REF_PTR as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_ext_ref_ptr
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_SET_SKIP_FILM_GRAIN as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_skip_film_grain
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_SET_FRAME_SIZE_LIMIT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_set_frame_size_limit
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_FRAME_CORRUPTED as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_frame_corrupted
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_LAST_QUANTIZER as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_last_quantizer
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_LAST_REF_UPDATES as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_last_ref_updates
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_BIT_DEPTH as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_bit_depth
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_IMG_FORMAT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_img_format
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_TILE_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_tile_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_TILE_COUNT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_tile_count
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_DISPLAY_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_render_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_FRAME_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_frame_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_GET_ACCOUNTING as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_accounting
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_GET_NEW_FRAME_IMAGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_new_frame_image
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_COPY_NEW_FRAME_IMAGE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_copy_new_frame_image
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1_GET_REFERENCE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_reference
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_FRAME_HEADER_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_frame_header_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_TILE_DATA as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_tile_data
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_FWD_KF_PRESENT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_fwd_kf_value
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_ALTREF_PRESENT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_altref_present
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_FRAME_FLAGS as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_frame_flags
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_TILE_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_tile_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_SCREEN_CONTENT_TOOLS_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_screen_content_tools_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_STILL_PICTURE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_still_picture
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_SB_SIZE as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_sb_size
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_SHOW_EXISTING_FRAME_FLAG as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_show_existing_frame_flag
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_S_FRAME_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_s_frame_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_SHOW_FRAME_FLAG as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_show_frame_flag
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_BASE_Q_IDX as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_base_q_idx
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AOMD_GET_ORDER_HINT as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_order_hint
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: AV1D_GET_MI_INFO as core::ffi::c_int,
                fn_0: Some(
                    ctrl_get_mi_info
                        as unsafe extern "C-unwind" fn(
                            *mut aom_codec_alg_priv_t,
                            ::core::ffi::VaList,
                        ) -> aom_codec_err_t,
                ),
            };
            init
        },
        {
            let mut init = aom_codec_ctrl_fn_map {
                ctrl_id: 0 as core::ffi::c_int,
                fn_0: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut aom_codec_av1_dx_algo: aom_codec_iface_t = unsafe {
    {
        let mut init = aom_codec_iface {
            name: b"AOMedia Project AV1 Decoder v3.15.2\0" as *const u8
                as *const core::ffi::c_char,
            abi_version: AOM_CODEC_INTERNAL_ABI_VERSION,
            caps: (AOM_CODEC_CAP_DECODER | AOM_CODEC_CAP_EXTERNAL_FRAME_BUFFER)
                as aom_codec_caps_t,
            init: Some(
                decoder_init
                    as unsafe extern "C-unwind" fn(*mut aom_codec_ctx_t) -> aom_codec_err_t,
            ),
            destroy: Some(
                decoder_destroy
                    as unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> aom_codec_err_t,
            ),
            ctrl_maps: core::ptr::addr_of!(decoder_ctrl_maps).cast::<aom_codec_ctrl_fn_map_t>(),
            dec: {
                let mut init = aom_codec_dec_iface {
                    peek_si: Some(
                        decoder_peek_si
                            as unsafe extern "C-unwind" fn(
                                *const uint8_t,
                                size_t,
                                *mut aom_codec_stream_info_t,
                            ) -> aom_codec_err_t,
                    ),
                    get_si: Some(
                        decoder_get_si
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *mut aom_codec_stream_info_t,
                            ) -> aom_codec_err_t,
                    ),
                    decode: Some(
                        decoder_decode
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *const uint8_t,
                                size_t,
                                *mut core::ffi::c_void,
                            ) -> aom_codec_err_t,
                    ),
                    get_frame: Some(
                        decoder_get_frame
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *mut aom_codec_iter_t,
                            ) -> *mut aom_image_t,
                    ),
                    set_fb_fn: Some(
                        decoder_set_fb_fn
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                aom_get_frame_buffer_cb_fn_t,
                                aom_release_frame_buffer_cb_fn_t,
                                *mut core::ffi::c_void,
                            ) -> aom_codec_err_t,
                    ),
                };
                init
            },
            enc: {
                let mut init = aom_codec_enc_iface {
                    cfg_count: 0 as core::ffi::c_int,
                    cfgs: 0 as *const aom_codec_enc_cfg_t,
                    encode: None,
                    get_cx_data: None,
                    cfg_set: None,
                    get_glob_hdrs: None,
                    get_preview: None,
                };
                init
            },
            set_option: None,
        };
        init
    }
};
#[no_mangle]
pub static mut aom_codec_av1_inspect_algo: aom_codec_iface_t = unsafe {
    {
        let mut init = aom_codec_iface {
            name: b"AOMedia Project AV1 Decoder Inspector v3.15.2\0" as *const u8
                as *const core::ffi::c_char,
            abi_version: AOM_CODEC_INTERNAL_ABI_VERSION,
            caps: (AOM_CODEC_CAP_DECODER | AOM_CODEC_CAP_EXTERNAL_FRAME_BUFFER)
                as aom_codec_caps_t,
            init: Some(
                decoder_init
                    as unsafe extern "C-unwind" fn(*mut aom_codec_ctx_t) -> aom_codec_err_t,
            ),
            destroy: Some(
                decoder_destroy
                    as unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t) -> aom_codec_err_t,
            ),
            ctrl_maps: core::ptr::addr_of!(decoder_ctrl_maps).cast::<aom_codec_ctrl_fn_map_t>(),
            dec: {
                let mut init = aom_codec_dec_iface {
                    peek_si: Some(
                        decoder_peek_si
                            as unsafe extern "C-unwind" fn(
                                *const uint8_t,
                                size_t,
                                *mut aom_codec_stream_info_t,
                            ) -> aom_codec_err_t,
                    ),
                    get_si: Some(
                        decoder_get_si
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *mut aom_codec_stream_info_t,
                            ) -> aom_codec_err_t,
                    ),
                    decode: Some(
                        decoder_inspect
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *const uint8_t,
                                size_t,
                                *mut core::ffi::c_void,
                            ) -> aom_codec_err_t,
                    ),
                    get_frame: Some(
                        decoder_get_frame
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                *mut aom_codec_iter_t,
                            ) -> *mut aom_image_t,
                    ),
                    set_fb_fn: Some(
                        decoder_set_fb_fn
                            as unsafe extern "C-unwind" fn(
                                *mut aom_codec_alg_priv_t,
                                aom_get_frame_buffer_cb_fn_t,
                                aom_release_frame_buffer_cb_fn_t,
                                *mut core::ffi::c_void,
                            ) -> aom_codec_err_t,
                    ),
                };
                init
            },
            enc: {
                let mut init = aom_codec_enc_iface {
                    cfg_count: 0 as core::ffi::c_int,
                    cfgs: 0 as *const aom_codec_enc_cfg_t,
                    encode: None,
                    get_cx_data: None,
                    cfg_set: None,
                    get_glob_hdrs: None,
                    get_preview: None,
                };
                init
            },
            set_option: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_codec_av1_dx() -> *const aom_codec_iface_t {
    return &raw const aom_codec_av1_dx_algo;
}
pub const AOM_CODEC_CAP_EXTERNAL_FRAME_BUFFER: core::ffi::c_int = 0x200000
    as core::ffi::c_int;
pub const AOM_CODEC_CAP_DECODER: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const AOM_FRAME_IS_KEY: core::ffi::c_uint = 0x1 as core::ffi::c_uint;
pub const AOM_FRAME_IS_INTRAONLY: core::ffi::c_uint = 0x10 as core::ffi::c_uint;
pub const AOM_FRAME_IS_SWITCH: core::ffi::c_uint = 0x20 as core::ffi::c_uint;
pub const AOM_FRAME_IS_ERROR_RESILIENT: core::ffi::c_uint = 0x40 as core::ffi::c_uint;
pub const AOM_FRAME_IS_DELAYED_RANDOM_ACCESS_POINT: core::ffi::c_uint = 0x80
    as core::ffi::c_uint;
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn decrease_ref_count(buf: *mut RefCntBuffer, pool: *mut BufferPool) {
    if !buf.is_null() {
        (*buf).ref_count -= 1;
        if (*buf).ref_count == 0 as core::ffi::c_int
            && !((*buf).raw_frame_buffer.data).is_null()
        {
            ((*pool).release_fb_cb)
                .expect(
                    "non-null function pointer",
                )((*pool).cb_priv, &raw mut (*buf).raw_frame_buffer);
            (*buf).raw_frame_buffer.data = 0 as *mut uint8_t;
            (*buf).raw_frame_buffer.size = 0 as size_t;
            (*buf).raw_frame_buffer.priv_0 = NULL;
        }
    }
}
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const MAX_TILE_ROWS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const MAX_TILE_COLS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const OP_POINTS_CNT_MINUS_1_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const OP_POINTS_IDC_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const LEVEL_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn yuvconfig2image(
    mut img: *mut aom_image_t,
    mut yv12: *const YV12_BUFFER_CONFIG,
    mut user_priv: *mut core::ffi::c_void,
) {
    let mut bps: core::ffi::c_int = 0;
    if (*yv12).subsampling_y == 0 {
        if (*yv12).subsampling_x == 0 {
            (*img).fmt = AOM_IMG_FMT_I444;
            bps = 24 as core::ffi::c_int;
        } else {
            (*img).fmt = AOM_IMG_FMT_I422;
            bps = 16 as core::ffi::c_int;
        }
    } else {
        (*img).fmt = AOM_IMG_FMT_I420;
        bps = 12 as core::ffi::c_int;
    }
    (*img).cp = (*yv12).color_primaries;
    (*img).tc = (*yv12).transfer_characteristics;
    (*img).mc = (*yv12).matrix_coefficients;
    (*img).monochrome = (*yv12).monochrome as core::ffi::c_int;
    (*img).csp = (*yv12).chroma_sample_position;
    (*img).range = (*yv12).color_range;
    (*img).bit_depth = 8 as core::ffi::c_uint;
    (*img).w = (*yv12).c2rust_unnamed.c2rust_unnamed.y_width as core::ffi::c_uint;
    (*img).h = (*yv12).c2rust_unnamed_0.c2rust_unnamed.y_height as core::ffi::c_uint;
    (*img).d_w = (*yv12).c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        as core::ffi::c_uint;
    (*img).d_h = (*yv12).c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        as core::ffi::c_uint;
    (*img).r_w = (*yv12).render_width as core::ffi::c_uint;
    (*img).r_h = (*yv12).render_height as core::ffi::c_uint;
    (*img).x_chroma_shift = (*yv12).subsampling_x as core::ffi::c_uint;
    (*img).y_chroma_shift = (*yv12).subsampling_y as core::ffi::c_uint;
    (*img).planes[AOM_PLANE_Y as usize] = (*yv12)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .y_buffer as *mut core::ffi::c_uchar;
    (*img).planes[AOM_PLANE_U as usize] = (*yv12)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .u_buffer as *mut core::ffi::c_uchar;
    (*img).planes[AOM_PLANE_V as usize] = (*yv12)
        .c2rust_unnamed_4
        .c2rust_unnamed
        .v_buffer as *mut core::ffi::c_uchar;
    (*img).stride[AOM_PLANE_Y as usize] = (*yv12)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .y_stride;
    (*img).stride[AOM_PLANE_U as usize] = (*yv12)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .uv_stride;
    (*img).stride[AOM_PLANE_V as usize] = (*yv12)
        .c2rust_unnamed_3
        .c2rust_unnamed
        .uv_stride;
    if (*yv12).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        bps *= 2 as core::ffi::c_int;
        (*img).fmt = ((*img).fmt as core::ffi::c_uint
            | AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint) as aom_img_fmt_t;
        (*img).bit_depth = (*yv12).bit_depth;
        (*img).planes[AOM_PLANE_Y as usize] = (((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
            as *mut uint8_t as *mut core::ffi::c_uchar;
        (*img).planes[AOM_PLANE_U as usize] = (((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .u_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
            as *mut uint8_t as *mut core::ffi::c_uchar;
        (*img).planes[AOM_PLANE_V as usize] = (((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer as uintptr_t) << 1 as core::ffi::c_int) as *mut uint16_t
            as *mut uint8_t as *mut core::ffi::c_uchar;
        (*img).stride[AOM_PLANE_Y as usize] = 2 as core::ffi::c_int
            * (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride;
        (*img).stride[AOM_PLANE_U as usize] = 2 as core::ffi::c_int
            * (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride;
        (*img).stride[AOM_PLANE_V as usize] = 2 as core::ffi::c_int
            * (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride;
    }
    (*img).bps = bps;
    (*img).user_priv = user_priv;
    (*img).img_data = (*yv12).buffer_alloc as *mut core::ffi::c_uchar;
    (*img).img_data_owner = 0 as core::ffi::c_int;
    (*img).self_allocd = 0 as core::ffi::c_int;
    (*img).sz = (*yv12).frame_size;
    (*img).metadata = 0 as *mut aom_metadata_array_t;
}
#[inline]
unsafe extern "C-unwind" fn image2yuvconfig(
    mut img: *const aom_image_t,
    mut yv12: *mut YV12_BUFFER_CONFIG,
) -> aom_codec_err_t {
    (*yv12).c2rust_unnamed_4.c2rust_unnamed.y_buffer = (*img)
        .planes[AOM_PLANE_Y as usize] as *mut uint8_t;
    (*yv12).c2rust_unnamed_4.c2rust_unnamed.u_buffer = (*img)
        .planes[AOM_PLANE_U as usize] as *mut uint8_t;
    (*yv12).c2rust_unnamed_4.c2rust_unnamed.v_buffer = (*img)
        .planes[AOM_PLANE_V as usize] as *mut uint8_t;
    (*yv12).c2rust_unnamed_1.c2rust_unnamed.y_crop_width = (*img).d_w
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_2.c2rust_unnamed.y_crop_height = (*img).d_h
        as core::ffi::c_int;
    (*yv12).render_width = (*img).r_w as core::ffi::c_int;
    (*yv12).render_height = (*img).r_h as core::ffi::c_int;
    (*yv12).c2rust_unnamed.c2rust_unnamed.y_width = (*img).w as core::ffi::c_int;
    (*yv12).c2rust_unnamed_0.c2rust_unnamed.y_height = (*img).h as core::ffi::c_int;
    (*yv12).c2rust_unnamed.c2rust_unnamed.uv_width = (((*yv12)
        .c2rust_unnamed
        .c2rust_unnamed
        .y_width as core::ffi::c_uint)
        .wrapping_add((*img).x_chroma_shift) >> (*img).x_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_0.c2rust_unnamed.uv_height = (((*yv12)
        .c2rust_unnamed_0
        .c2rust_unnamed
        .y_height as core::ffi::c_uint)
        .wrapping_add((*img).y_chroma_shift) >> (*img).y_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_1.c2rust_unnamed.uv_crop_width = (((*yv12)
        .c2rust_unnamed_1
        .c2rust_unnamed
        .y_crop_width as core::ffi::c_uint)
        .wrapping_add((*img).x_chroma_shift) >> (*img).x_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_2.c2rust_unnamed.uv_crop_height = (((*yv12)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .y_crop_height as core::ffi::c_uint)
        .wrapping_add((*img).y_chroma_shift) >> (*img).y_chroma_shift)
        as core::ffi::c_int;
    (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride = (*img)
        .stride[AOM_PLANE_Y as usize];
    (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride = (*img)
        .stride[AOM_PLANE_U as usize];
    (*yv12).color_primaries = (*img).cp;
    (*yv12).transfer_characteristics = (*img).tc;
    (*yv12).matrix_coefficients = (*img).mc;
    (*yv12).monochrome = (*img).monochrome as uint8_t;
    (*yv12).chroma_sample_position = (*img).csp;
    (*yv12).color_range = (*img).range;
    if (*img).fmt as core::ffi::c_uint & AOM_IMG_FMT_HIGHBITDEPTH as core::ffi::c_uint
        != 0
    {
        (*yv12).c2rust_unnamed_4.c2rust_unnamed.y_buffer = ((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .y_buffer as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
        (*yv12).c2rust_unnamed_4.c2rust_unnamed.u_buffer = ((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .u_buffer as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
        (*yv12).c2rust_unnamed_4.c2rust_unnamed.v_buffer = ((*yv12)
            .c2rust_unnamed_4
            .c2rust_unnamed
            .v_buffer as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t;
        (*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride >>= 1 as core::ffi::c_int;
        (*yv12).c2rust_unnamed_3.c2rust_unnamed.uv_stride >>= 1 as core::ffi::c_int;
        (*yv12).flags = YV12_FLAG_HIGHBITDEPTH;
    } else {
        (*yv12).flags = 0 as core::ffi::c_int;
    }
    let mut border: core::ffi::c_int = ((*yv12).c2rust_unnamed_3.c2rust_unnamed.y_stride
        - (((*img).w).wrapping_add(31 as core::ffi::c_uint) & !(31 as core::ffi::c_uint))
            as core::ffi::c_int) / 2 as core::ffi::c_int;
    (*yv12).border = if border < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        border
    };
    (*yv12).subsampling_x = (*img).x_chroma_shift as core::ffi::c_int;
    (*yv12).subsampling_y = (*img).y_chroma_shift as core::ffi::c_int;
    (*yv12).metadata = (*img).metadata;
    return AOM_CODEC_OK;
}
pub const FORCE_HIGHBITDEPTH_DECODING: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn lock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_lock(&raw mut (*pool).pool_mutex);
}
unsafe extern "C-unwind" fn unlock_buffer_pool(pool: *mut BufferPool) {
    pthread_mutex_unlock(&raw mut (*pool).pool_mutex);
}
#[inline]
unsafe extern "C-unwind" fn get_ref_frame(
    mut cm: *mut AV1_COMMON,
    mut index: core::ffi::c_int,
) -> *mut YV12_BUFFER_CONFIG {
    if index < 0 as core::ffi::c_int || index >= REF_FRAMES as core::ffi::c_int {
        return 0 as *mut YV12_BUFFER_CONFIG;
    }
    if ((*cm).ref_frame_map[index as usize]).is_null() {
        return 0 as *mut YV12_BUFFER_CONFIG;
    }
    return &raw mut (**((*cm).ref_frame_map).as_mut_ptr().offset(index as isize)).buf;
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

extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn snprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
}
