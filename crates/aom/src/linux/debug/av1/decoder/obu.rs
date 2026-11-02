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

// Mechanically translated from upstream/aom/av1/decoder/obu.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    pub type AVxWorkerImpl;
    fn aom_yv12_partial_copy_u_c(
        src_bc: *const yv12_buffer_config,
        hstart1: core::ffi::c_int,
        hend1: core::ffi::c_int,
        vstart1: core::ffi::c_int,
        vend1: core::ffi::c_int,
        dst_bc: *mut yv12_buffer_config,
        hstart2: core::ffi::c_int,
        vstart2: core::ffi::c_int,
    );
    fn aom_yv12_partial_copy_v_c(
        src_bc: *const yv12_buffer_config,
        hstart1: core::ffi::c_int,
        hend1: core::ffi::c_int,
        vstart1: core::ffi::c_int,
        vend1: core::ffi::c_int,
        dst_bc: *mut yv12_buffer_config,
        hstart2: core::ffi::c_int,
        vstart2: core::ffi::c_int,
    );
    fn aom_yv12_partial_copy_y_c(
        src_ybc: *const yv12_buffer_config,
        hstart1: core::ffi::c_int,
        hend1: core::ffi::c_int,
        vstart1: core::ffi::c_int,
        vend1: core::ffi::c_int,
        dst_ybc: *mut yv12_buffer_config,
        hstart2: core::ffi::c_int,
        vstart2: core::ffi::c_int,
    );
    fn aom_uleb_decode(
        buffer: *const uint8_t,
        available: size_t,
        value: *mut uint64_t,
        length: *mut size_t,
    ) -> core::ffi::c_int;
    fn aom_img_metadata_alloc(
        type_0: uint32_t,
        data: *const uint8_t,
        sz: size_t,
        insert_flag: aom_metadata_insert_flags_t,
    ) -> *mut aom_metadata_t;
    fn aom_img_metadata_free(metadata: *mut aom_metadata_t);
    fn aom_rb_bytes_read(rb: *const aom_read_bit_buffer) -> size_t;
    fn aom_rb_read_bit(rb: *mut aom_read_bit_buffer) -> core::ffi::c_int;
    fn aom_rb_read_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_read_obu_header_and_size(
        data: *const uint8_t,
        bytes_available: size_t,
        is_annexb: core::ffi::c_int,
        obu_header: *mut ObuHeader,
        payload_size: *mut size_t,
        bytes_read: *mut size_t,
    ) -> aom_codec_err_t;
    fn av1_max_level_bitrate(
        seq_profile: BITSTREAM_PROFILE,
        seq_level_idx: core::ffi::c_int,
        seq_tier: core::ffi::c_int,
    ) -> int64_t;
    fn aom_img_metadata_array_alloc(sz: size_t) -> *mut aom_metadata_array_t;
    fn aom_alloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        border: core::ffi::c_int,
        byte_alignment_0: core::ffi::c_int,
        alloc_pyramid: bool,
        alloc_y_plane_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn av1_get_uniform_tile_size(
        cm: *const AV1Common,
        w: *mut core::ffi::c_int,
        h: *mut core::ffi::c_int,
    ) -> bool;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn av1_set_reference_dec(
        cm: *mut AV1_COMMON,
        idx: core::ffi::c_int,
        use_external_ref: core::ffi::c_int,
        sd: *mut YV12_BUFFER_CONFIG,
    ) -> aom_codec_err_t;
    fn av1_read_sequence_header(
        cm: *mut AV1_COMMON,
        rb: *mut aom_read_bit_buffer,
        seq_params: *mut SequenceHeader,
    );
    fn av1_read_profile(rb: *mut aom_read_bit_buffer) -> BITSTREAM_PROFILE;
    fn av1_check_trailing_bits(
        pbi: *mut AV1Decoder,
        rb: *mut aom_read_bit_buffer,
    ) -> core::ffi::c_int;
    fn av1_decode_frame_headers_and_setup(
        pbi: *mut AV1Decoder,
        rb: *mut aom_read_bit_buffer,
        trailing_bits_present: core::ffi::c_int,
    ) -> uint32_t;
    fn av1_decode_tg_tiles_and_wrapup(
        pbi: *mut AV1Decoder,
        data: *const uint8_t,
        data_end: *const uint8_t,
        p_data_end: *mut *const uint8_t,
        start_tile: core::ffi::c_int,
        end_tile: core::ffi::c_int,
        initialize_flag: core::ffi::c_int,
    );
    fn av1_read_color_config(
        rb: *mut aom_read_bit_buffer,
        allow_lowbitdepth: core::ffi::c_int,
        seq_params: *mut SequenceHeader,
        error_info: *mut aom_internal_error_info,
    );
    fn av1_read_timing_info_header(
        timing_info: *mut aom_timing_info_t,
        error: *mut aom_internal_error_info,
        rb: *mut aom_read_bit_buffer,
    );
    fn av1_read_decoder_model_info(
        decoder_model_info: *mut aom_dec_model_info_t,
        rb: *mut aom_read_bit_buffer,
    );
    fn av1_read_op_parameters_info(
        op_params: *mut aom_dec_model_op_parameters_t,
        buffer_delay_length: core::ffi::c_int,
        rb: *mut aom_read_bit_buffer,
    );
    fn av1_init_read_bit_buffer(
        pbi: *mut AV1Decoder,
        rb: *mut aom_read_bit_buffer,
        data: *const uint8_t,
        data_end: *const uint8_t,
    ) -> *mut aom_read_bit_buffer;
    fn av1_set_single_tile_decoding_mode(cm: *mut AV1_COMMON);
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
pub type aom_metadata_array_t = aom_metadata_array;
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
pub type aom_metadata_insert_flags_t = aom_metadata_insert_flags;
pub type aom_metadata_insert_flags = core::ffi::c_uint;
pub const AOM_MIF_ANY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 18;
pub const AOM_MIF_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 17;
pub const AOM_MIF_NON_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 16;
pub const AOM_MIF_ANY_FRAME: aom_metadata_insert_flags = 2;
pub const AOM_MIF_KEY_FRAME: aom_metadata_insert_flags = 1;
pub const AOM_MIF_NON_KEY_FRAME: aom_metadata_insert_flags = 0;
pub type size_t = usize;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type aom_color_range_t = aom_color_range;
pub type aom_color_range = core::ffi::c_uint;
pub const AOM_CR_FULL_RANGE: aom_color_range = 1;
pub const AOM_CR_STUDIO_RANGE: aom_color_range = 0;
pub type aom_chroma_sample_position_t = aom_chroma_sample_position;
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_matrix_coefficients_t = aom_matrix_coefficients;
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
pub type aom_transfer_characteristics_t = aom_transfer_characteristics;
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
pub type aom_color_primaries_t = aom_color_primaries;
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
pub type __int8_t = i8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
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
pub type OBU_METADATA_TYPE = core::ffi::c_uint;
pub const OBU_METADATA_TYPE_TIMECODE: OBU_METADATA_TYPE = 5;
pub const OBU_METADATA_TYPE_ITUT_T35: OBU_METADATA_TYPE = 4;
pub const OBU_METADATA_TYPE_SCALABILITY: OBU_METADATA_TYPE = 3;
pub const OBU_METADATA_TYPE_HDR_MDCV: OBU_METADATA_TYPE = 2;
pub const OBU_METADATA_TYPE_HDR_CLL: OBU_METADATA_TYPE = 1;
pub const OBU_METADATA_TYPE_AOM_RESERVED_0: OBU_METADATA_TYPE = 0;
pub type aom_rb_error_handler = Option<
    unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_read_bit_buffer {
    pub bit_buffer: *const uint8_t,
    pub bit_buffer_end: *const uint8_t,
    pub bit_offset: uint32_t,
    pub error_handler_data: *mut core::ffi::c_void,
    pub error_handler: aom_rb_error_handler,
}
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
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
pub type TX_SIZE = uint8_t;
pub type TX_TYPE = uint8_t;
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
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const SEQ_LEVEL_KEEP_STATS: C2RustUnnamed_11 = 32;
pub const SEQ_LEVEL_MAX: C2RustUnnamed_11 = 31;
pub const SEQ_LEVELS: C2RustUnnamed_11 = 28;
pub const SEQ_LEVEL_8_3: C2RustUnnamed_11 = 27;
pub const SEQ_LEVEL_8_2: C2RustUnnamed_11 = 26;
pub const SEQ_LEVEL_8_1: C2RustUnnamed_11 = 25;
pub const SEQ_LEVEL_8_0: C2RustUnnamed_11 = 24;
pub const SEQ_LEVEL_7_3: C2RustUnnamed_11 = 23;
pub const SEQ_LEVEL_7_2: C2RustUnnamed_11 = 22;
pub const SEQ_LEVEL_7_1: C2RustUnnamed_11 = 21;
pub const SEQ_LEVEL_7_0: C2RustUnnamed_11 = 20;
pub const SEQ_LEVEL_6_3: C2RustUnnamed_11 = 19;
pub const SEQ_LEVEL_6_2: C2RustUnnamed_11 = 18;
pub const SEQ_LEVEL_6_1: C2RustUnnamed_11 = 17;
pub const SEQ_LEVEL_6_0: C2RustUnnamed_11 = 16;
pub const SEQ_LEVEL_5_3: C2RustUnnamed_11 = 15;
pub const SEQ_LEVEL_5_2: C2RustUnnamed_11 = 14;
pub const SEQ_LEVEL_5_1: C2RustUnnamed_11 = 13;
pub const SEQ_LEVEL_5_0: C2RustUnnamed_11 = 12;
pub const SEQ_LEVEL_4_3: C2RustUnnamed_11 = 11;
pub const SEQ_LEVEL_4_2: C2RustUnnamed_11 = 10;
pub const SEQ_LEVEL_4_1: C2RustUnnamed_11 = 9;
pub const SEQ_LEVEL_4_0: C2RustUnnamed_11 = 8;
pub const SEQ_LEVEL_3_3: C2RustUnnamed_11 = 7;
pub const SEQ_LEVEL_3_2: C2RustUnnamed_11 = 6;
pub const SEQ_LEVEL_3_1: C2RustUnnamed_11 = 5;
pub const SEQ_LEVEL_3_0: C2RustUnnamed_11 = 4;
pub const SEQ_LEVEL_2_3: C2RustUnnamed_11 = 3;
pub const SEQ_LEVEL_2_2: C2RustUnnamed_11 = 2;
pub const SEQ_LEVEL_2_1: C2RustUnnamed_11 = 1;
pub const SEQ_LEVEL_2_0: C2RustUnnamed_11 = 0;
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
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const SCALABILITY_SS: C2RustUnnamed_12 = 14;
pub const SCALABILITY_S2T3h: C2RustUnnamed_12 = 13;
pub const SCALABILITY_S2T2h: C2RustUnnamed_12 = 12;
pub const SCALABILITY_S2T1h: C2RustUnnamed_12 = 11;
pub const SCALABILITY_L2T3h: C2RustUnnamed_12 = 10;
pub const SCALABILITY_L2T2h: C2RustUnnamed_12 = 9;
pub const SCALABILITY_L2T1h: C2RustUnnamed_12 = 8;
pub const SCALABILITY_S2T3: C2RustUnnamed_12 = 7;
pub const SCALABILITY_S2T2: C2RustUnnamed_12 = 6;
pub const SCALABILITY_S2T1: C2RustUnnamed_12 = 5;
pub const SCALABILITY_L2T3: C2RustUnnamed_12 = 4;
pub const SCALABILITY_L2T2: C2RustUnnamed_12 = 3;
pub const SCALABILITY_L2T1: C2RustUnnamed_12 = 2;
pub const SCALABILITY_L1T3: C2RustUnnamed_12 = 1;
pub const SCALABILITY_L1T2: C2RustUnnamed_12 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_timing {
    pub num_units_in_display_tick: uint32_t,
    pub time_scale: uint32_t,
    pub equal_picture_interval: core::ffi::c_int,
    pub num_ticks_per_picture: uint32_t,
}
pub type aom_timing_info_t = aom_timing;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_info {
    pub num_units_in_decoding_tick: uint32_t,
    pub encoder_decoder_buffer_delay_length: core::ffi::c_int,
    pub buffer_removal_time_length: core::ffi::c_int,
    pub frame_presentation_time_length: core::ffi::c_int,
}
pub type aom_dec_model_info_t = aom_dec_model_info;
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
pub type aom_dec_model_op_parameters_t = aom_dec_model_op_parameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_s_frame_info {
    pub is_s_frame: core::ffi::c_int,
    pub is_s_frame_at_altref: core::ffi::c_int,
}
pub type aom_cdf_prob = uint16_t;
pub type od_ec_window = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_reader {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub ec: od_ec_dec,
    pub allow_update_cdf: uint8_t,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
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
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
pub type __jmp_buf = [core::ffi::c_long; 8];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [core::ffi::c_char; 48],
    pub __align: core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eob_info {
    pub eob: uint16_t,
    pub max_scan_line: uint16_t,
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
pub type MACROBLOCKD = macroblockd;
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
pub type AV1LfSync = AV1LfSyncData;
pub type AV1LrSync = AV1LrSyncData;
pub type AV1CdefWorkerData = AV1CdefWorker;
pub type AV1CdefSync = AV1CdefSyncData;
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
pub struct EXTERNAL_REFERENCES {
    pub refs: [YV12_BUFFER_CONFIG; 128],
    pub num: core::ffi::c_int,
}
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
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MI_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MI_SIZE_LOG2;
pub const OP_POINTS_CNT_MINUS_1_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const OP_POINTS_IDC_BITS: core::ffi::c_int = 12 as core::ffi::c_int;
pub const LEVEL_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_EXTERNAL_REFERENCES: core::ffi::c_int = 128 as core::ffi::c_int;
pub const MAX_TILES: core::ffi::c_int = 512 as core::ffi::c_int;
pub const MAX_NUM_TEMPORAL_LAYERS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_NUM_SPATIAL_LAYERS: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn av1_num_planes(mut cm: *const AV1_COMMON) -> core::ffi::c_int {
    return if (*(*cm).seq_params).monochrome as core::ffi::c_int != 0 {
        1 as core::ffi::c_int
    } else {
        MAX_MB_PLANE
    };
}
#[inline]
unsafe extern "C-unwind" fn is_valid_seq_level_idx(
    mut seq_level_idx: AV1_LEVEL,
) -> core::ffi::c_int {
    return (seq_level_idx as core::ffi::c_int == SEQ_LEVEL_MAX as core::ffi::c_int
        || (seq_level_idx as core::ffi::c_int) < SEQ_LEVELS as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_2_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_2_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_3_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_3_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_4_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_4_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_0 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_1 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_7_3 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_0 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_1 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_2 as core::ffi::c_int
            && seq_level_idx as core::ffi::c_int != SEQ_LEVEL_8_3 as core::ffi::c_int)
        as core::ffi::c_int;
}
pub const MAX_MB_PLANE: core::ffi::c_int = 3 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_num_layers_from_operating_point_idc(
    mut operating_point_idc: core::ffi::c_int,
    mut number_spatial_layers: *mut core::ffi::c_uint,
    mut number_temporal_layers: *mut core::ffi::c_uint,
) -> aom_codec_err_t {
    if number_spatial_layers.is_null() || number_temporal_layers.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    if operating_point_idc == 0 as core::ffi::c_int {
        *number_temporal_layers = 1 as core::ffi::c_uint;
        *number_spatial_layers = 1 as core::ffi::c_uint;
    } else {
        *number_spatial_layers = 0 as core::ffi::c_uint;
        *number_temporal_layers = 0 as core::ffi::c_uint;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < MAX_NUM_SPATIAL_LAYERS {
            *number_spatial_layers = (*number_spatial_layers)
                .wrapping_add(
                    (operating_point_idc >> j + MAX_NUM_TEMPORAL_LAYERS
                        & 0x1 as core::ffi::c_int) as core::ffi::c_uint,
                );
            j += 1;
        }
        let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while j_0 < MAX_NUM_TEMPORAL_LAYERS {
            *number_temporal_layers = (*number_temporal_layers)
                .wrapping_add(
                    (operating_point_idc >> j_0 & 0x1 as core::ffi::c_int)
                        as core::ffi::c_uint,
                );
            j_0 += 1;
        }
    }
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn is_obu_in_current_operating_point(
    mut pbi: *mut AV1Decoder,
    mut obu_header: *const ObuHeader,
) -> core::ffi::c_int {
    if (*pbi).current_operating_point == 0 || (*obu_header).has_extension == 0 {
        return 1 as core::ffi::c_int;
    }
    if (*pbi).current_operating_point >> (*obu_header).temporal_layer_id
        & 0x1 as core::ffi::c_int != 0
        && (*pbi).current_operating_point
            >> (*obu_header).spatial_layer_id + 8 as core::ffi::c_int
            & 0x1 as core::ffi::c_int != 0
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn byte_alignment(
    mut error_info: *mut aom_internal_error_info,
    rb: *mut aom_read_bit_buffer,
) {
    while (*rb).bit_offset & 7 as uint32_t != 0 {
        if aom_rb_read_bit(rb) != 0 {
            aom_internal_error(
                error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"byte_alignment() is not all 0 bits\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C-unwind" fn read_temporal_delimiter_obu() -> uint32_t {
    return 0 as uint32_t;
}
unsafe extern "C-unwind" fn read_bitstream_level(
    mut seq_level_idx: *mut AV1_LEVEL,
    mut rb: *mut aom_read_bit_buffer,
) -> core::ffi::c_int {
    *seq_level_idx = aom_rb_read_literal(rb, LEVEL_BITS) as AV1_LEVEL;
    if is_valid_seq_level_idx(*seq_level_idx) == 0 {
        return 0 as core::ffi::c_int;
    }
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn are_seq_headers_consistent(
    mut seq_params_old: *const SequenceHeader,
    mut seq_params_new: *const SequenceHeader,
) -> core::ffi::c_int {
    return (memcmp(
        seq_params_old as *const core::ffi::c_void,
        seq_params_new as *const core::ffi::c_void,
        360 as size_t,
    ) == 0) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_sequence_header_obu(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let saved_bit_offset: uint32_t = (*rb).bit_offset;
    if ((*rb).error_handler).is_some() {} else {
        __assert_fail(
            b"rb->error_handler\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            110 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 78],
                [core::ffi::c_char; 78],
            >(
                *b"uint32_t read_sequence_header_obu(AV1Decoder *, struct aom_read_bit_buffer *)\0",
            ))
                .as_ptr(),
        );
    };
    let mut sh: SequenceHeader = *(*cm).seq_params;
    let seq_params: *mut SequenceHeader = &raw mut sh;
    (*seq_params).profile = av1_read_profile(rb);
    if (*seq_params).profile as core::ffi::c_int > CONFIG_MAX_DECODE_PROFILE {
        (*pbi).error.error_code = AOM_CODEC_UNSUP_BITSTREAM;
        return 0 as uint32_t;
    }
    (*seq_params).still_picture = aom_rb_read_bit(rb) as uint8_t;
    (*seq_params).reduced_still_picture_hdr = aom_rb_read_bit(rb) as uint8_t;
    if (*seq_params).still_picture == 0
        && (*seq_params).reduced_still_picture_hdr as core::ffi::c_int != 0
    {
        (*pbi).error.error_code = AOM_CODEC_UNSUP_BITSTREAM;
        return 0 as uint32_t;
    }
    if (*seq_params).reduced_still_picture_hdr != 0 {
        (*seq_params).timing_info_present = 0 as core::ffi::c_int;
        (*seq_params).decoder_model_info_present_flag = 0 as uint8_t;
        (*seq_params).display_model_info_present_flag = 0 as uint8_t;
        (*seq_params).operating_points_cnt_minus_1 = 0 as core::ffi::c_int;
        (*seq_params).operating_point_idc[0 as core::ffi::c_int as usize] = 0
            as core::ffi::c_int;
        (*seq_params).has_nonzero_operating_point_idc = false_0 != 0;
        if read_bitstream_level(
            &raw mut *((*seq_params).seq_level_idx)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize),
            rb,
        ) == 0
        {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"Value %d of seq_level_idx[%d] is not yet defined\0" as *const u8
                    as *const core::ffi::c_char,
                (*seq_params).seq_level_idx[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
        }
        (*seq_params).tier[0 as core::ffi::c_int as usize] = 0 as uint8_t;
        (*seq_params)
            .op_params[0 as core::ffi::c_int as usize]
            .decoder_model_param_present_flag = 0 as core::ffi::c_int;
        (*seq_params)
            .op_params[0 as core::ffi::c_int as usize]
            .display_model_param_present_flag = 0 as core::ffi::c_int;
    } else {
        (*seq_params).timing_info_present = aom_rb_read_bit(rb);
        if (*seq_params).timing_info_present != 0 {
            av1_read_timing_info_header(
                &raw mut (*seq_params).timing_info,
                &raw mut (*pbi).error,
                rb,
            );
            (*seq_params).decoder_model_info_present_flag = aom_rb_read_bit(rb)
                as uint8_t;
            if (*seq_params).decoder_model_info_present_flag != 0 {
                av1_read_decoder_model_info(&raw mut (*seq_params).decoder_model_info, rb);
            }
        } else {
            (*seq_params).decoder_model_info_present_flag = 0 as uint8_t;
        }
        (*seq_params).display_model_info_present_flag = aom_rb_read_bit(rb) as uint8_t;
        (*seq_params).operating_points_cnt_minus_1 = aom_rb_read_literal(
            rb,
            OP_POINTS_CNT_MINUS_1_BITS,
        );
        (*seq_params).has_nonzero_operating_point_idc = false_0 != 0;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*seq_params).operating_points_cnt_minus_1 + 1 as core::ffi::c_int {
            (*seq_params).operating_point_idc[i as usize] = aom_rb_read_literal(
                rb,
                OP_POINTS_IDC_BITS,
            );
            if (*seq_params).operating_point_idc[i as usize] != 0 as core::ffi::c_int {
                (*seq_params).has_nonzero_operating_point_idc = true_0 != 0;
            }
            if read_bitstream_level(
                &raw mut *((*seq_params).seq_level_idx).as_mut_ptr().offset(i as isize),
                rb,
            ) == 0
            {
                aom_internal_error(
                    &raw mut (*pbi).error as *mut aom_internal_error_info,
                    AOM_CODEC_UNSUP_BITSTREAM,
                    b"Value %d of seq_level_idx[%d] is not yet defined\0" as *const u8
                        as *const core::ffi::c_char,
                    (*seq_params).seq_level_idx[i as usize] as core::ffi::c_int,
                    i,
                );
            }
            if (*seq_params).seq_level_idx[i as usize] as core::ffi::c_int
                >= SEQ_LEVEL_4_0 as core::ffi::c_int
            {
                (*seq_params).tier[i as usize] = aom_rb_read_bit(rb) as uint8_t;
            } else {
                (*seq_params).tier[i as usize] = 0 as uint8_t;
            }
            if (*seq_params).decoder_model_info_present_flag != 0 {
                (*seq_params).op_params[i as usize].decoder_model_param_present_flag = aom_rb_read_bit(
                    rb,
                );
                if (*seq_params).op_params[i as usize].decoder_model_param_present_flag
                    != 0
                {
                    av1_read_op_parameters_info(
                        &raw mut *((*seq_params).op_params).as_mut_ptr().offset(i as isize),
                        (*seq_params)
                            .decoder_model_info
                            .encoder_decoder_buffer_delay_length,
                        rb,
                    );
                }
            } else {
                (*seq_params).op_params[i as usize].decoder_model_param_present_flag = 0
                    as core::ffi::c_int;
            }
            if (*seq_params).timing_info_present != 0
                && ((*seq_params).timing_info.equal_picture_interval != 0
                    || (*seq_params)
                        .op_params[i as usize]
                        .decoder_model_param_present_flag != 0)
            {
                (*seq_params).op_params[i as usize].bitrate = av1_max_level_bitrate(
                    (*seq_params).profile,
                    (*seq_params).seq_level_idx[i as usize] as core::ffi::c_int,
                    (*seq_params).tier[i as usize] as core::ffi::c_int,
                );
                if (*seq_params).op_params[i as usize].bitrate == 0 as int64_t {
                    aom_internal_error(
                        &raw mut (*pbi).error as *mut aom_internal_error_info,
                        AOM_CODEC_UNSUP_BITSTREAM,
                        b"AV1 does not support this combination of profile, level, and tier.\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                (*seq_params).op_params[i as usize].buffer_size = (*seq_params)
                    .op_params[i as usize]
                    .bitrate;
            }
            if (*seq_params).timing_info_present != 0
                && (*seq_params).timing_info.equal_picture_interval != 0
                && (*seq_params).op_params[i as usize].decoder_model_param_present_flag
                    == 0
            {
                (*seq_params).op_params[i as usize].decoder_buffer_delay = 70000
                    as uint32_t;
                (*seq_params).op_params[i as usize].encoder_buffer_delay = 20000
                    as uint32_t;
                (*seq_params).op_params[i as usize].low_delay_mode_flag = 0
                    as core::ffi::c_int;
            }
            if (*seq_params).display_model_info_present_flag != 0 {
                (*seq_params).op_params[i as usize].display_model_param_present_flag = aom_rb_read_bit(
                    rb,
                );
                if (*seq_params).op_params[i as usize].display_model_param_present_flag
                    != 0
                {
                    (*seq_params).op_params[i as usize].initial_display_delay = aom_rb_read_literal(
                        rb,
                        4 as core::ffi::c_int,
                    ) + 1 as core::ffi::c_int;
                    if (*seq_params).op_params[i as usize].initial_display_delay
                        > 10 as core::ffi::c_int
                    {
                        aom_internal_error(
                            &raw mut (*pbi).error as *mut aom_internal_error_info,
                            AOM_CODEC_UNSUP_BITSTREAM,
                            b"AV1 does not support more than 10 decoded frames delay\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    }
                } else {
                    (*seq_params).op_params[i as usize].initial_display_delay = 10
                        as core::ffi::c_int;
                }
            } else {
                (*seq_params).op_params[i as usize].display_model_param_present_flag = 0
                    as core::ffi::c_int;
                (*seq_params).op_params[i as usize].initial_display_delay = 10
                    as core::ffi::c_int;
            }
            i += 1;
        }
    }
    let mut operating_point: core::ffi::c_int = (*pbi).operating_point;
    if operating_point < 0 as core::ffi::c_int
        || operating_point > (*seq_params).operating_points_cnt_minus_1
    {
        operating_point = 0 as core::ffi::c_int;
    }
    (*pbi).current_operating_point = (*seq_params)
        .operating_point_idc[operating_point as usize];
    let mut status: aom_codec_err_t = aom_get_num_layers_from_operating_point_idc(
        (*pbi).current_operating_point,
        &raw mut (*pbi).number_spatial_layers,
        &raw mut (*pbi).number_temporal_layers,
    );
    if status as core::ffi::c_uint
        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        (*pbi).error.error_code = status;
        return 0 as uint32_t;
    }
    av1_read_sequence_header(cm, rb, seq_params);
    av1_read_color_config(rb, (*pbi).allow_lowbitdepth, seq_params, &raw mut (*pbi).error);
    if !((*seq_params).subsampling_x == 0 as core::ffi::c_int
        && (*seq_params).subsampling_y == 0 as core::ffi::c_int)
        && !((*seq_params).subsampling_x == 1 as core::ffi::c_int
            && (*seq_params).subsampling_y == 1 as core::ffi::c_int)
        && !((*seq_params).subsampling_x == 1 as core::ffi::c_int
            && (*seq_params).subsampling_y == 0 as core::ffi::c_int)
    {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_UNSUP_BITSTREAM,
            b"Only 4:4:4, 4:2:2 and 4:2:0 are currently supported, %d %d subsampling is not supported.\n\0"
                as *const u8 as *const core::ffi::c_char,
            (*seq_params).subsampling_x,
            (*seq_params).subsampling_y,
        );
    }
    (*seq_params).film_grain_params_present = aom_rb_read_bit(rb) as uint8_t;
    if av1_check_trailing_bits(pbi as *mut AV1Decoder, rb) != 0 as core::ffi::c_int {
        return 0 as uint32_t;
    }
    if (*pbi).sequence_header_ready != 0 {
        if are_seq_headers_consistent((*cm).seq_params, seq_params) == 0 {
            (*pbi).sequence_header_changed = 1 as core::ffi::c_int;
        }
    }
    *(*cm).seq_params = *seq_params;
    (*pbi).sequence_header_ready = 1 as core::ffi::c_int;
    return ((*rb).bit_offset).wrapping_sub(saved_bit_offset).wrapping_add(7 as uint32_t)
        >> 3 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_frame_header_obu(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
    mut data: *const uint8_t,
    mut p_data_end: *mut *const uint8_t,
    mut trailing_bits_present: core::ffi::c_int,
) -> uint32_t {
    let hdr_size: uint32_t = av1_decode_frame_headers_and_setup(
        pbi as *mut AV1Decoder,
        rb,
        trailing_bits_present,
    ) as uint32_t;
    let mut cm: *const AV1_COMMON = &raw mut (*pbi).common;
    if (*cm).show_existing_frame != 0 {
        *p_data_end = data.offset(hdr_size as isize);
    }
    return hdr_size;
}
unsafe extern "C-unwind" fn read_tile_group_header(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
    mut start_tile: *mut core::ffi::c_int,
    mut end_tile: *mut core::ffi::c_int,
    mut tile_start_implicit: core::ffi::c_int,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let tiles: *mut CommonTileParams = &raw mut (*cm).tiles;
    let mut saved_bit_offset: uint32_t = (*rb).bit_offset;
    let mut tile_start_and_end_present_flag: core::ffi::c_int = 0 as core::ffi::c_int;
    let num_tiles: core::ffi::c_int = (*tiles).rows * (*tiles).cols;
    if (*tiles).large_scale == 0 && num_tiles > 1 as core::ffi::c_int {
        tile_start_and_end_present_flag = aom_rb_read_bit(rb);
        if tile_start_implicit != 0 && tile_start_and_end_present_flag != 0 {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_UNSUP_BITSTREAM,
                b"For OBU_FRAME type obu tile_start_and_end_present_flag must be 0\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
    }
    if (*tiles).large_scale != 0 || num_tiles == 1 as core::ffi::c_int
        || tile_start_and_end_present_flag == 0
    {
        *start_tile = 0 as core::ffi::c_int;
        *end_tile = num_tiles - 1 as core::ffi::c_int;
    } else {
        let mut tile_bits: core::ffi::c_int = (*tiles).log2_rows + (*tiles).log2_cols;
        *start_tile = aom_rb_read_literal(rb, tile_bits);
        *end_tile = aom_rb_read_literal(rb, tile_bits);
    }
    if *start_tile != (*pbi).next_start_tile {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"tg_start (%d) must be equal to %d\0" as *const u8
                as *const core::ffi::c_char,
            *start_tile,
            (*pbi).next_start_tile,
        );
    }
    if *start_tile > *end_tile {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"tg_end (%d) must be greater than or equal to tg_start (%d)\0" as *const u8
                as *const core::ffi::c_char,
            *end_tile,
            *start_tile,
        );
    }
    if *end_tile >= num_tiles {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"tg_end (%d) must be less than NumTiles (%d)\0" as *const u8
                as *const core::ffi::c_char,
            *end_tile,
            num_tiles,
        );
    }
    (*pbi).next_start_tile = if *end_tile == num_tiles - 1 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        *end_tile + 1 as core::ffi::c_int
    };
    return ((*rb).bit_offset).wrapping_sub(saved_bit_offset).wrapping_add(7 as uint32_t)
        >> 3 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_one_tile_group_obu(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
    mut is_first_tg: core::ffi::c_int,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut p_data_end: *mut *const uint8_t,
    mut is_last_tg: *mut core::ffi::c_int,
    mut tile_start_implicit: core::ffi::c_int,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut start_tile: core::ffi::c_int = 0;
    let mut end_tile: core::ffi::c_int = 0;
    if (*rb).bit_offset & 7 as uint32_t == 0 as uint32_t {} else {
        __assert_fail(
            b"(rb->bit_offset & 7) == 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            357 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 146],
                [core::ffi::c_char; 146],
            >(
                *b"uint32_t read_one_tile_group_obu(AV1Decoder *, struct aom_read_bit_buffer *, int, const uint8_t *, const uint8_t *, const uint8_t **, int *, int)\0",
            ))
                .as_ptr(),
        );
    };
    if ((*rb).bit_buffer).offset(aom_rb_bytes_read(rb) as isize) == data {} else {
        __assert_fail(
            b"rb->bit_buffer + aom_rb_bytes_read(rb) == data\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            358 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 146],
                [core::ffi::c_char; 146],
            >(
                *b"uint32_t read_one_tile_group_obu(AV1Decoder *, struct aom_read_bit_buffer *, int, const uint8_t *, const uint8_t *, const uint8_t **, int *, int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut header_size: uint32_t = read_tile_group_header(
        pbi,
        rb,
        &raw mut start_tile,
        &raw mut end_tile,
        tile_start_implicit,
    );
    byte_alignment(&raw mut (*pbi).error, rb);
    data = data.offset(header_size as isize);
    av1_decode_tg_tiles_and_wrapup(
        pbi as *mut AV1Decoder,
        data,
        data_end,
        p_data_end,
        start_tile,
        end_tile,
        is_first_tg,
    );
    let mut tg_payload_size: uint32_t = (*p_data_end).offset_from(data)
        as core::ffi::c_long as uint32_t;
    *is_last_tg = (end_tile
        == (*cm).tiles.rows * (*cm).tiles.cols - 1 as core::ffi::c_int)
        as core::ffi::c_int;
    return header_size.wrapping_add(tg_payload_size);
}
unsafe extern "C-unwind" fn alloc_tile_list_buffer(
    mut pbi: *mut AV1Decoder,
    mut tile_width_in_pixels: core::ffi::c_int,
    mut tile_height_in_pixels: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let output_frame_width: core::ffi::c_int = ((*pbi)
        .output_frame_width_in_tiles_minus_1 + 1 as core::ffi::c_int)
        * tile_width_in_pixels;
    let output_frame_height: core::ffi::c_int = ((*pbi)
        .output_frame_height_in_tiles_minus_1 + 1 as core::ffi::c_int)
        * tile_height_in_pixels;
    if (*pbi).tile_count_minus_1 + 1 as core::ffi::c_int
        <= ((*pbi).output_frame_width_in_tiles_minus_1 + 1 as core::ffi::c_int)
            * ((*pbi).output_frame_height_in_tiles_minus_1 + 1 as core::ffi::c_int)
    {} else {
        __assert_fail(
            b"(pbi->tile_count_minus_1 + 1) <= (pbi->output_frame_width_in_tiles_minus_1 + 1) * (pbi->output_frame_height_in_tiles_minus_1 + 1)\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            388 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 52],
                [core::ffi::c_char; 52],
            >(*b"void alloc_tile_list_buffer(AV1Decoder *, int, int)\0"))
                .as_ptr(),
        );
    };
    if aom_alloc_frame_buffer(
        &raw mut (*pbi).tile_list_outbuf,
        output_frame_width,
        output_frame_height,
        (*(*cm).seq_params).subsampling_x,
        (*(*cm).seq_params).subsampling_y,
        ((*(*cm).seq_params).use_highbitdepth as core::ffi::c_int != 0
            && (*(*cm).seq_params).bit_depth as core::ffi::c_uint
                > AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int,
        0 as core::ffi::c_int,
        (*cm).features.byte_alignment,
        false_0 != 0,
        0 as core::ffi::c_int,
    ) != 0
    {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_MEM_ERROR,
            b"Failed to allocate the tile list output buffer\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
}
unsafe extern "C-unwind" fn yv12_tile_copy(
    mut src: *const YV12_BUFFER_CONFIG,
    mut hstart1: core::ffi::c_int,
    mut hend1: core::ffi::c_int,
    mut vstart1: core::ffi::c_int,
    mut vend1: core::ffi::c_int,
    mut dst: *mut YV12_BUFFER_CONFIG,
    mut hstart2: core::ffi::c_int,
    mut vstart2: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let src_stride: core::ffi::c_int = if plane > 0 as core::ffi::c_int {
        (*src).c2rust_unnamed_3.strides[1 as core::ffi::c_int as usize]
    } else {
        (*src).c2rust_unnamed_3.strides[0 as core::ffi::c_int as usize]
    };
    let dst_stride: core::ffi::c_int = if plane > 0 as core::ffi::c_int {
        (*dst).c2rust_unnamed_3.strides[1 as core::ffi::c_int as usize]
    } else {
        (*dst).c2rust_unnamed_3.strides[0 as core::ffi::c_int as usize]
    };
    let mut row: core::ffi::c_int = 0;
    let mut col: core::ffi::c_int = 0;
    if (*src).flags & 8 as core::ffi::c_int != 0 {} else {
        __assert_fail(
            b"src->flags & YV12_FLAG_HIGHBITDEPTH\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            412 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"void yv12_tile_copy(const YV12_BUFFER_CONFIG *, int, int, int, int, YV12_BUFFER_CONFIG *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if (*dst).flags & 8 as core::ffi::c_int == 0 {} else {
        __assert_fail(
            b"!(dst->flags & YV12_FLAG_HIGHBITDEPTH)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            413 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 105],
                [core::ffi::c_char; 105],
            >(
                *b"void yv12_tile_copy(const YV12_BUFFER_CONFIG *, int, int, int, int, YV12_BUFFER_CONFIG *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut src16: *const uint16_t = ((((*src).c2rust_unnamed_4.buffers[plane as usize])
        .offset((vstart1 * src_stride) as isize)
        .offset(hstart1 as isize) as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut dst8: *mut uint8_t = ((*dst).c2rust_unnamed_4.buffers[plane as usize])
        .offset((vstart2 * dst_stride) as isize)
        .offset(hstart2 as isize);
    row = vstart1;
    while row < vend1 {
        col = 0 as core::ffi::c_int;
        while col < hend1 - hstart1 {
            let fresh1 = src16;
            src16 = src16.offset(1);
            let fresh2 = dst8;
            dst8 = dst8.offset(1);
            *fresh2 = *fresh1 as uint8_t;
            col += 1;
        }
        src16 = src16.offset((src_stride - (hend1 - hstart1)) as isize);
        dst8 = dst8.offset((dst_stride - (hend1 - hstart1)) as isize);
        row += 1;
    }
}
unsafe extern "C-unwind" fn copy_decoded_tile_to_tile_list_buffer(
    mut pbi: *mut AV1Decoder,
    mut tile_idx: core::ffi::c_int,
    mut tile_width_in_pixels: core::ffi::c_int,
    mut tile_height_in_pixels: core::ffi::c_int,
) {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let ssy: core::ffi::c_int = (*(*cm).seq_params).subsampling_y;
    let ssx: core::ffi::c_int = (*(*cm).seq_params).subsampling_x;
    let num_planes: core::ffi::c_int = av1_num_planes(cm) as core::ffi::c_int;
    let mut cur_frame: *mut YV12_BUFFER_CONFIG = &raw mut (*(*cm).cur_frame).buf;
    let tr: core::ffi::c_int = tile_idx
        / ((*pbi).output_frame_width_in_tiles_minus_1 + 1 as core::ffi::c_int);
    let tc: core::ffi::c_int = tile_idx
        % ((*pbi).output_frame_width_in_tiles_minus_1 + 1 as core::ffi::c_int);
    let mut plane: core::ffi::c_int = 0;
    plane = 0 as core::ffi::c_int;
    while plane < num_planes {
        let shift_x: core::ffi::c_int = if plane > 0 as core::ffi::c_int {
            ssx
        } else {
            0 as core::ffi::c_int
        };
        let shift_y: core::ffi::c_int = if plane > 0 as core::ffi::c_int {
            ssy
        } else {
            0 as core::ffi::c_int
        };
        let h: core::ffi::c_int = tile_height_in_pixels >> shift_y;
        let w: core::ffi::c_int = tile_width_in_pixels >> shift_x;
        let mut vstart1: core::ffi::c_int = (*pbi).dec_tile_row * h;
        let mut vend1: core::ffi::c_int = vstart1 + h;
        let mut hstart1: core::ffi::c_int = (*pbi).dec_tile_col * w;
        let mut hend1: core::ffi::c_int = hstart1 + w;
        let mut vstart2: core::ffi::c_int = tr * h;
        let mut hstart2: core::ffi::c_int = tc * w;
        if (*(*cm).seq_params).use_highbitdepth as core::ffi::c_int != 0
            && (*(*cm).seq_params).bit_depth as core::ffi::c_uint
                == AOM_BITS_8 as core::ffi::c_int as core::ffi::c_uint
        {
            yv12_tile_copy(
                cur_frame,
                hstart1,
                hend1,
                vstart1,
                vend1,
                &raw mut (*pbi).tile_list_outbuf,
                hstart2,
                vstart2,
                plane,
            );
        } else {
            match plane {
                0 => {
                    aom_yv12_partial_copy_y_c(
                        cur_frame,
                        hstart1,
                        hend1,
                        vstart1,
                        vend1,
                        &raw mut (*pbi).tile_list_outbuf,
                        hstart2,
                        vstart2,
                    );
                }
                1 => {
                    aom_yv12_partial_copy_u_c(
                        cur_frame,
                        hstart1,
                        hend1,
                        vstart1,
                        vend1,
                        &raw mut (*pbi).tile_list_outbuf,
                        hstart2,
                        vstart2,
                    );
                }
                2 => {
                    aom_yv12_partial_copy_v_c(
                        cur_frame,
                        hstart1,
                        hend1,
                        vstart1,
                        vend1,
                        &raw mut (*pbi).tile_list_outbuf,
                        hstart2,
                        vstart2,
                    );
                }
                _ => {
                    if 0 as core::ffi::c_int != 0 {} else {
                        __assert_fail(
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/decoder/obu.c\0" as *const u8
                                as *const core::ffi::c_char,
                            474 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 72],
                                [core::ffi::c_char; 72],
                            >(
                                *b"void copy_decoded_tile_to_tile_list_buffer(AV1Decoder *, int, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                }
            }
        }
        plane += 1;
    }
}
unsafe extern "C-unwind" fn read_and_decode_one_tile_list(
    mut pbi: *mut AV1Decoder,
    mut rb: *mut aom_read_bit_buffer,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut p_data_end: *mut *const uint8_t,
    mut frame_decoding_finished: *mut core::ffi::c_int,
) -> uint32_t {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut tile_list_payload_size: uint32_t = 0 as uint32_t;
    let num_tiles: core::ffi::c_int = (*cm).tiles.cols * (*cm).tiles.rows;
    let start_tile: core::ffi::c_int = 0 as core::ffi::c_int;
    let end_tile: core::ffi::c_int = num_tiles - 1 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    (*pbi).output_frame_width_in_tiles_minus_1 = aom_rb_read_literal(
        rb,
        8 as core::ffi::c_int,
    );
    (*pbi).output_frame_height_in_tiles_minus_1 = aom_rb_read_literal(
        rb,
        8 as core::ffi::c_int,
    );
    (*pbi).tile_count_minus_1 = aom_rb_read_literal(rb, 16 as core::ffi::c_int);
    if (*pbi).tile_count_minus_1 + 1 as core::ffi::c_int
        > ((*pbi).output_frame_width_in_tiles_minus_1 + 1 as core::ffi::c_int)
            * ((*pbi).output_frame_height_in_tiles_minus_1 + 1 as core::ffi::c_int)
    {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        return 0 as uint32_t;
    }
    if (*pbi).tile_count_minus_1 > MAX_TILES - 1 as core::ffi::c_int {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        return 0 as uint32_t;
    }
    let mut tile_width: core::ffi::c_int = 0;
    let mut tile_height: core::ffi::c_int = 0;
    if !av1_get_uniform_tile_size(cm, &raw mut tile_width, &raw mut tile_height) {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        return 0 as uint32_t;
    }
    let tile_width_in_pixels: core::ffi::c_int = tile_width * MI_SIZE;
    let tile_height_in_pixels: core::ffi::c_int = tile_height * MI_SIZE;
    alloc_tile_list_buffer(pbi, tile_width_in_pixels, tile_height_in_pixels);
    let mut tile_list_info_bytes: uint32_t = 4 as uint32_t;
    tile_list_payload_size = tile_list_payload_size.wrapping_add(tile_list_info_bytes);
    data = data.offset(tile_list_info_bytes as isize);
    let mut tile_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i <= (*pbi).tile_count_minus_1 {
        (*rb).bit_offset = 0 as uint32_t;
        (*rb).bit_buffer = data;
        let mut tile_info_bytes: uint32_t = 5 as uint32_t;
        let mut ref_idx: core::ffi::c_int = aom_rb_read_literal(
            rb,
            8 as core::ffi::c_int,
        );
        if ref_idx >= MAX_EXTERNAL_REFERENCES {
            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as uint32_t;
        }
        av1_set_reference_dec(
            cm,
            (*cm).remapped_ref_idx[0 as core::ffi::c_int as usize],
            1 as core::ffi::c_int,
            &raw mut *((*pbi).ext_refs.refs).as_mut_ptr().offset(ref_idx as isize),
        );
        (*pbi).dec_tile_row = aom_rb_read_literal(rb, 8 as core::ffi::c_int);
        (*pbi).dec_tile_col = aom_rb_read_literal(rb, 8 as core::ffi::c_int);
        if (*pbi).dec_tile_row < 0 as core::ffi::c_int
            || (*pbi).dec_tile_col < 0 as core::ffi::c_int
            || (*pbi).dec_tile_row >= (*cm).tiles.rows
            || (*pbi).dec_tile_col >= (*cm).tiles.cols
        {
            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as uint32_t;
        }
        (*pbi).coded_tile_data_size = (aom_rb_read_literal(rb, 16 as core::ffi::c_int)
            + 1 as core::ffi::c_int) as uint32_t;
        data = data.offset(tile_info_bytes as isize);
        if (data_end.offset_from(data) as core::ffi::c_long as size_t)
            < (*pbi).coded_tile_data_size as size_t
        {
            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as uint32_t;
        }
        av1_decode_tg_tiles_and_wrapup(
            pbi as *mut AV1Decoder,
            data,
            data.offset((*pbi).coded_tile_data_size as isize),
            p_data_end,
            start_tile,
            end_tile,
            0 as core::ffi::c_int,
        );
        let mut tile_payload_size: uint32_t = (*p_data_end).offset_from(data)
            as core::ffi::c_long as uint32_t;
        tile_list_payload_size = tile_list_payload_size
            .wrapping_add(tile_info_bytes.wrapping_add(tile_payload_size));
        data = *p_data_end;
        if data <= data_end {} else {
            __assert_fail(
                b"data <= data_end\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/obu.c\0" as *const u8
                    as *const core::ffi::c_char,
                573 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 142],
                    [core::ffi::c_char; 142],
                >(
                    *b"uint32_t read_and_decode_one_tile_list(AV1Decoder *, struct aom_read_bit_buffer *, const uint8_t *, const uint8_t *, const uint8_t **, int *)\0",
                ))
                    .as_ptr(),
            );
        };
        copy_decoded_tile_to_tile_list_buffer(
            pbi,
            tile_idx,
            tile_width_in_pixels,
            tile_height_in_pixels,
        );
        tile_idx += 1;
        i += 1;
    }
    *frame_decoding_finished = 1 as core::ffi::c_int;
    return tile_list_payload_size;
}
unsafe extern "C-unwind" fn get_last_nonzero_byte_index(
    mut data: *const uint8_t,
    mut sz: size_t,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = sz as core::ffi::c_int - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int
        && *data.offset(i as isize) as core::ffi::c_int == 0 as core::ffi::c_int
    {
        i -= 1;
    }
    return i;
}
unsafe extern "C-unwind" fn alloc_read_metadata(
    pbi: *mut AV1Decoder,
    mut metadata_type: OBU_METADATA_TYPE,
    mut data: *const uint8_t,
    mut sz: size_t,
    mut insert_flag: aom_metadata_insert_flags_t,
) {
    if ((*pbi).metadata).is_null() {
        (*pbi).metadata = aom_img_metadata_array_alloc(0 as size_t);
        if ((*pbi).metadata).is_null() {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_MEM_ERROR,
                b"Failed to allocate metadata array\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
    let mut metadata: *mut aom_metadata_t = aom_img_metadata_alloc(
        metadata_type as uint32_t,
        data,
        sz,
        insert_flag,
    );
    if metadata.is_null() {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_MEM_ERROR,
            b"Error allocating metadata\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    let mut metadata_array: *mut *mut aom_metadata_t = realloc(
        (*(*pbi).metadata).metadata_array as *mut core::ffi::c_void,
        ((*(*pbi).metadata).sz)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut aom_metadata_t>() as size_t),
    ) as *mut *mut aom_metadata_t;
    if metadata_array.is_null() {
        aom_img_metadata_free(metadata);
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_MEM_ERROR,
            b"Error growing metadata array\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    (*(*pbi).metadata).metadata_array = metadata_array;
    let fresh3 = &raw mut *((*(*pbi).metadata).metadata_array)
        .offset((*(*pbi).metadata).sz as isize);
    *fresh3 = metadata;
    (*(*pbi).metadata).sz = ((*(*pbi).metadata).sz).wrapping_add(1);
}
unsafe extern "C-unwind" fn read_metadata_itut_t35(
    pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut sz: size_t,
    mut has_obu_extension_header: bool,
) {
    if sz == 0 as size_t {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"itu_t_t35_country_code is missing\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    let mut country_code_size: core::ffi::c_int = 1 as core::ffi::c_int;
    if *data as core::ffi::c_int == 0xff as core::ffi::c_int {
        if sz == 1 as size_t {
            aom_internal_error(
                &raw mut (*pbi).error as *mut aom_internal_error_info,
                AOM_CODEC_CORRUPT_FRAME,
                b"itu_t_t35_country_code_extension_byte is missing\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        country_code_size += 1;
    }
    let mut end_index: core::ffi::c_int = get_last_nonzero_byte_index(data, sz);
    if end_index < country_code_size {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"No trailing bits found in ITU-T T.35 metadata OBU\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    if *data.offset(end_index as isize) as core::ffi::c_int != 0x80 as core::ffi::c_int {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"The last nonzero byte of the ITU-T T.35 metadata OBU is 0x%02x, should be 0x80.\0"
                as *const u8 as *const core::ffi::c_char,
            *data.offset(end_index as isize) as core::ffi::c_int,
        );
    }
    alloc_read_metadata(
        pbi,
        OBU_METADATA_TYPE_ITUT_T35,
        data,
        end_index as size_t,
        (if has_obu_extension_header as core::ffi::c_int != 0 {
            AOM_MIF_ANY_FRAME_LAYER_SPECIFIC as core::ffi::c_int
        } else {
            AOM_MIF_ANY_FRAME as core::ffi::c_int
        }) as aom_metadata_insert_flags_t,
    );
}
unsafe extern "C-unwind" fn read_metadata_hdr_cll(
    pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut sz: size_t,
) -> size_t {
    let kHdrCllPayloadSize: size_t = 4 as size_t;
    if sz < kHdrCllPayloadSize {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Incorrect HDR CLL metadata payload size\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    alloc_read_metadata(
        pbi,
        OBU_METADATA_TYPE_HDR_CLL,
        data,
        kHdrCllPayloadSize,
        AOM_MIF_ANY_FRAME,
    );
    return kHdrCllPayloadSize;
}
unsafe extern "C-unwind" fn read_metadata_hdr_mdcv(
    pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut sz: size_t,
) -> size_t {
    let kMdcvPayloadSize: size_t = 24 as size_t;
    if sz < kMdcvPayloadSize {
        aom_internal_error(
            &raw mut (*pbi).error as *mut aom_internal_error_info,
            AOM_CODEC_CORRUPT_FRAME,
            b"Incorrect HDR MDCV metadata payload size\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    alloc_read_metadata(
        pbi,
        OBU_METADATA_TYPE_HDR_MDCV,
        data,
        kMdcvPayloadSize,
        AOM_MIF_ANY_FRAME,
    );
    return kMdcvPayloadSize;
}
unsafe extern "C-unwind" fn scalability_structure(mut rb: *mut aom_read_bit_buffer) {
    let spatial_layers_cnt_minus_1: core::ffi::c_int = aom_rb_read_literal(
        rb,
        2 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let spatial_layer_dimensions_present_flag: core::ffi::c_int = aom_rb_read_bit(rb)
        as core::ffi::c_int;
    let spatial_layer_description_present_flag: core::ffi::c_int = aom_rb_read_bit(rb)
        as core::ffi::c_int;
    let temporal_group_description_present_flag: core::ffi::c_int = aom_rb_read_bit(rb)
        as core::ffi::c_int;
    aom_rb_read_literal(rb, 3 as core::ffi::c_int);
    if spatial_layer_dimensions_present_flag != 0 {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i <= spatial_layers_cnt_minus_1 {
            aom_rb_read_literal(rb, 16 as core::ffi::c_int);
            aom_rb_read_literal(rb, 16 as core::ffi::c_int);
            i += 1;
        }
    }
    if spatial_layer_description_present_flag != 0 {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 <= spatial_layers_cnt_minus_1 {
            aom_rb_read_literal(rb, 8 as core::ffi::c_int);
            i_0 += 1;
        }
    }
    if temporal_group_description_present_flag != 0 {
        let temporal_group_size: core::ffi::c_int = aom_rb_read_literal(
            rb,
            8 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < temporal_group_size {
            aom_rb_read_literal(rb, 3 as core::ffi::c_int);
            aom_rb_read_bit(rb);
            aom_rb_read_bit(rb);
            let temporal_group_ref_cnt: core::ffi::c_int = aom_rb_read_literal(
                rb,
                3 as core::ffi::c_int,
            ) as core::ffi::c_int;
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < temporal_group_ref_cnt {
                aom_rb_read_literal(rb, 8 as core::ffi::c_int);
                j += 1;
            }
            i_1 += 1;
        }
    }
}
unsafe extern "C-unwind" fn read_metadata_scalability(mut rb: *mut aom_read_bit_buffer) {
    let scalability_mode_idc: core::ffi::c_int = aom_rb_read_literal(
        rb,
        8 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if scalability_mode_idc == SCALABILITY_SS as core::ffi::c_int {
        scalability_structure(rb);
    }
}
unsafe extern "C-unwind" fn read_metadata_timecode(mut rb: *mut aom_read_bit_buffer) {
    aom_rb_read_literal(rb, 5 as core::ffi::c_int);
    let full_timestamp_flag: core::ffi::c_int = aom_rb_read_bit(rb) as core::ffi::c_int;
    aom_rb_read_bit(rb);
    aom_rb_read_bit(rb);
    aom_rb_read_literal(rb, 9 as core::ffi::c_int);
    if full_timestamp_flag != 0 {
        aom_rb_read_literal(rb, 6 as core::ffi::c_int);
        aom_rb_read_literal(rb, 6 as core::ffi::c_int);
        aom_rb_read_literal(rb, 5 as core::ffi::c_int);
    } else {
        let seconds_flag: core::ffi::c_int = aom_rb_read_bit(rb) as core::ffi::c_int;
        if seconds_flag != 0 {
            aom_rb_read_literal(rb, 6 as core::ffi::c_int);
            let minutes_flag: core::ffi::c_int = aom_rb_read_bit(rb) as core::ffi::c_int;
            if minutes_flag != 0 {
                aom_rb_read_literal(rb, 6 as core::ffi::c_int);
                let hours_flag: core::ffi::c_int = aom_rb_read_bit(rb)
                    as core::ffi::c_int;
                if hours_flag != 0 {
                    aom_rb_read_literal(rb, 5 as core::ffi::c_int);
                }
            }
        }
    }
    let time_offset_length: core::ffi::c_int = aom_rb_read_literal(
        rb,
        5 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if time_offset_length != 0 {
        aom_rb_read_literal(rb, time_offset_length);
    }
}
unsafe extern "C-unwind" fn get_last_nonzero_byte(
    mut data: *const uint8_t,
    mut sz: size_t,
) -> uint8_t {
    let mut i: size_t = sz;
    while i != 0 as size_t {
        i = i.wrapping_sub(1);
        if *data.offset(i as isize) as core::ffi::c_int != 0 as core::ffi::c_int {
            return *data.offset(i as isize);
        }
    }
    return 0 as uint8_t;
}
unsafe extern "C-unwind" fn read_metadata(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut sz: size_t,
    mut has_obu_extension_header: bool,
) -> size_t {
    let mut type_length: size_t = 0;
    let mut type_value: uint64_t = 0;
    if aom_uleb_decode(data, sz, &raw mut type_value, &raw mut type_length)
        < 0 as core::ffi::c_int
    {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        return 0 as size_t;
    }
    let metadata_type: OBU_METADATA_TYPE = type_value as OBU_METADATA_TYPE;
    if metadata_type as core::ffi::c_uint == 0 as core::ffi::c_uint
        || metadata_type as core::ffi::c_uint >= 6 as core::ffi::c_uint
    {
        if get_last_nonzero_byte(
            data.offset(type_length as isize),
            sz.wrapping_sub(type_length),
        ) as core::ffi::c_int == 0 as core::ffi::c_int
        {
            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as size_t;
        }
        return sz;
    }
    if metadata_type as core::ffi::c_uint
        == OBU_METADATA_TYPE_ITUT_T35 as core::ffi::c_int as core::ffi::c_uint
    {
        read_metadata_itut_t35(
            pbi,
            data.offset(type_length as isize),
            sz.wrapping_sub(type_length),
            has_obu_extension_header,
        );
        return sz;
    } else if metadata_type as core::ffi::c_uint
        == OBU_METADATA_TYPE_HDR_CLL as core::ffi::c_int as core::ffi::c_uint
    {
        let mut bytes_read: size_t = type_length
            .wrapping_add(
                read_metadata_hdr_cll(
                    pbi,
                    data.offset(type_length as isize),
                    sz.wrapping_sub(type_length),
                ),
            );
        if get_last_nonzero_byte(
            data.offset(bytes_read as isize),
            sz.wrapping_sub(bytes_read),
        ) as core::ffi::c_int != 0x80 as core::ffi::c_int
        {
            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as size_t;
        }
        return sz;
    } else if metadata_type as core::ffi::c_uint
        == OBU_METADATA_TYPE_HDR_MDCV as core::ffi::c_int as core::ffi::c_uint
    {
        let mut bytes_read_0: size_t = type_length
            .wrapping_add(
                read_metadata_hdr_mdcv(
                    pbi,
                    data.offset(type_length as isize),
                    sz.wrapping_sub(type_length),
                ),
            );
        if get_last_nonzero_byte(
            data.offset(bytes_read_0 as isize),
            sz.wrapping_sub(bytes_read_0),
        ) as core::ffi::c_int != 0x80 as core::ffi::c_int
        {
            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as size_t;
        }
        return sz;
    }
    let mut rb: aom_read_bit_buffer = aom_read_bit_buffer {
        bit_buffer: 0 as *const uint8_t,
        bit_buffer_end: 0 as *const uint8_t,
        bit_offset: 0,
        error_handler_data: 0 as *mut core::ffi::c_void,
        error_handler: None,
    };
    av1_init_read_bit_buffer(
        pbi as *mut AV1Decoder,
        &raw mut rb,
        data.offset(type_length as isize),
        data.offset(sz as isize),
    );
    if metadata_type as core::ffi::c_uint
        == OBU_METADATA_TYPE_SCALABILITY as core::ffi::c_int as core::ffi::c_uint
    {
        read_metadata_scalability(&raw mut rb);
    } else {
        if metadata_type as core::ffi::c_uint
            == OBU_METADATA_TYPE_TIMECODE as core::ffi::c_int as core::ffi::c_uint
        {} else {
            __assert_fail(
                b"metadata_type == OBU_METADATA_TYPE_TIMECODE\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/decoder/obu.c\0" as *const u8
                    as *const core::ffi::c_char,
                835 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 67],
                    [core::ffi::c_char; 67],
                >(
                    *b"size_t read_metadata(AV1Decoder *, const uint8_t *, size_t, _Bool)\0",
                ))
                    .as_ptr(),
            );
        };
        read_metadata_timecode(&raw mut rb);
    }
    if av1_check_trailing_bits(pbi as *mut AV1Decoder, &raw mut rb) != 0 as core::ffi::c_int
    {
        return 0 as size_t;
    }
    if rb.bit_offset & 7 as uint32_t == 0 as uint32_t {} else {
        __assert_fail(
            b"(rb.bit_offset & 7) == 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            842 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"size_t read_metadata(AV1Decoder *, const uint8_t *, size_t, _Bool)\0"))
                .as_ptr(),
        );
    };
    return type_length.wrapping_add((rb.bit_offset >> 3 as core::ffi::c_int) as size_t);
}
unsafe extern "C-unwind" fn read_padding(
    mut error_info: *mut aom_internal_error_info,
    mut data: *const uint8_t,
    mut sz: size_t,
) -> size_t {
    if sz > 0 as size_t {
        let last_nonzero_byte: uint8_t = get_last_nonzero_byte(data, sz) as uint8_t;
        if last_nonzero_byte as core::ffi::c_int != 0x80 as core::ffi::c_int {
            (*error_info).error_code = AOM_CODEC_CORRUPT_FRAME;
            return 0 as size_t;
        }
    }
    return sz;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_decode_frame_from_obus(
    mut pbi: *mut AV1Decoder,
    mut data: *const uint8_t,
    mut data_end: *const uint8_t,
    mut p_data_end: *mut *const uint8_t,
) -> core::ffi::c_int {
    let cm: *mut AV1_COMMON = &raw mut (*pbi).common;
    let mut frame_decoding_finished: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut is_first_tg_obu_received: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut frame_header: *const uint8_t = data;
    let mut frame_header_size: uint32_t = 0 as uint32_t;
    let mut obu_header: ObuHeader = ObuHeader {
        size: 0,
        type_0: 0 as OBU_TYPE,
        has_size_field: 0,
        has_extension: 0,
        temporal_layer_id: 0,
        spatial_layer_id: 0,
    };
    if (*pbi).error.setjmp != 0 {} else {
        __assert_fail(
            b"pbi->error.setjmp\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/decoder/obu.c\0" as *const u8 as *const core::ffi::c_char,
            889 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 104],
                [core::ffi::c_char; 104],
            >(
                *b"int aom_decode_frame_from_obus(struct AV1Decoder *, const uint8_t *, const uint8_t *, const uint8_t **)\0",
            ))
                .as_ptr(),
        );
    };
    memset(
        &raw mut obu_header as *mut ObuHeader as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<ObuHeader>() as size_t,
    );
    (*pbi).seen_frame_header = 0 as core::ffi::c_int;
    (*pbi).next_start_tile = 0 as core::ffi::c_int;
    (*pbi).num_tile_groups = 0 as core::ffi::c_int;
    if data_end < data {
        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
        return -(1 as core::ffi::c_int);
    }
    if (*cm).tiles.large_scale == 0 {
        (*pbi).camera_frame_header_ready = 0 as core::ffi::c_int;
    }
    while frame_decoding_finished == 0
        && (*pbi).error.error_code as core::ffi::c_uint
            == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        let mut rb: aom_read_bit_buffer = aom_read_bit_buffer {
            bit_buffer: 0 as *const uint8_t,
            bit_buffer_end: 0 as *const uint8_t,
            bit_offset: 0,
            error_handler_data: 0 as *mut core::ffi::c_void,
            error_handler: None,
        };
        let mut payload_size: size_t = 0 as size_t;
        let mut decoded_payload_size: size_t = 0 as size_t;
        let mut obu_payload_offset: size_t = 0 as size_t;
        let mut bytes_read: size_t = 0 as size_t;
        let bytes_available: size_t = data_end.offset_from(data) as core::ffi::c_long
            as size_t;
        if bytes_available == 0 as size_t && (*pbi).seen_frame_header == 0 {
            *p_data_end = data;
            (*pbi).error.error_code = AOM_CODEC_OK;
            break;
        } else {
            let mut status: aom_codec_err_t = aom_read_obu_header_and_size(
                data,
                bytes_available,
                (*pbi).is_annexb,
                &raw mut obu_header,
                &raw mut payload_size,
                &raw mut bytes_read,
            );
            if status as core::ffi::c_uint
                != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
            {
                (*pbi).error.error_code = status;
                return -(1 as core::ffi::c_int);
            }
            (*pbi).obu_size_hdr.data = data.offset(obu_header.size as isize);
            (*pbi).obu_size_hdr.size = bytes_read.wrapping_sub(obu_header.size);
            data = data.offset(bytes_read as isize);
            if (data_end.offset_from(data) as core::ffi::c_long as size_t) < payload_size
            {
                (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                return -(1 as core::ffi::c_int);
            }
            (*cm).temporal_layer_id = obu_header.temporal_layer_id;
            (*cm).spatial_layer_id = obu_header.spatial_layer_id;
            if obu_header.type_0 as core::ffi::c_int
                != OBU_TEMPORAL_DELIMITER as core::ffi::c_int
                && obu_header.type_0 as core::ffi::c_int
                    != OBU_SEQUENCE_HEADER as core::ffi::c_int
            {
                if is_obu_in_current_operating_point(
                    pbi as *mut AV1Decoder,
                    &raw mut obu_header,
                ) == 0
                {
                    data = data.offset(payload_size as isize);
                    continue;
                }
            }
            av1_init_read_bit_buffer(
                pbi,
                &raw mut rb,
                data,
                data.offset(payload_size as isize),
            );
            let mut current_block_123: u64;
            match obu_header.type_0 as core::ffi::c_int {
                2 => {
                    decoded_payload_size = read_temporal_delimiter_obu() as size_t;
                    if (*pbi).seen_frame_header != 0 {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    current_block_123 = 0;
                }
                1 => {
                    decoded_payload_size = read_sequence_header_obu(
                        pbi as *mut AV1Decoder,
                        &raw mut rb,
                    ) as size_t;
                    if (*pbi).error.error_code as core::ffi::c_uint
                        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    if (*pbi).sequence_header_changed != 0
                        && (*pbi).seen_frame_header != 0
                    {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    current_block_123 = 0;
                }
                3 | 7 | 6 => {
                    if obu_header.type_0 as core::ffi::c_int
                        == OBU_REDUNDANT_FRAME_HEADER as core::ffi::c_int
                    {
                        if (*pbi).seen_frame_header == 0 {
                            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                            return -(1 as core::ffi::c_int);
                        }
                    } else if (*pbi).seen_frame_header != 0 {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    if (*pbi).seen_frame_header == 0
                        || (*cm).tiles.large_scale != 0
                            && (*pbi).camera_frame_header_ready == 0
                            && obu_header.type_0 as core::ffi::c_int
                                != OBU_REDUNDANT_FRAME_HEADER as core::ffi::c_int
                    {
                        frame_header_size = read_frame_header_obu(
                            pbi as *mut AV1Decoder,
                            &raw mut rb,
                            data,
                            p_data_end,
                            (obu_header.type_0 as core::ffi::c_int
                                != OBU_FRAME as core::ffi::c_int) as core::ffi::c_int,
                        );
                        frame_header = data;
                        (*pbi).seen_frame_header = 1 as core::ffi::c_int;
                        if (*pbi).ext_tile_debug == 0 && (*cm).tiles.large_scale != 0 {
                            (*pbi).camera_frame_header_ready = 1 as core::ffi::c_int;
                        }
                    } else {
                        if frame_header_size as size_t > payload_size
                            || memcmp(
                                data as *const core::ffi::c_void,
                                frame_header as *const core::ffi::c_void,
                                frame_header_size as size_t,
                            ) != 0 as core::ffi::c_int
                        {
                            (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                            return -(1 as core::ffi::c_int);
                        }
                        if rb.bit_offset == 0 as uint32_t {} else {
                            __assert_fail(
                                b"rb.bit_offset == 0\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"upstream/aom/av1/decoder/obu.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                1007 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 104],
                                    [core::ffi::c_char; 104],
                                >(
                                    *b"int aom_decode_frame_from_obus(struct AV1Decoder *, const uint8_t *, const uint8_t *, const uint8_t **)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        rb.bit_offset = (8 as uint32_t).wrapping_mul(frame_header_size);
                    }
                    decoded_payload_size = frame_header_size as size_t;
                    (*pbi).frame_header_size = frame_header_size as size_t;
                    (*(*cm).cur_frame).temporal_id = obu_header.temporal_layer_id;
                    (*(*cm).cur_frame).spatial_id = obu_header.spatial_layer_id;
                    if (*cm).show_existing_frame != 0 {
                        if obu_header.type_0 as core::ffi::c_int
                            == OBU_FRAME as core::ffi::c_int
                        {
                            (*pbi).error.error_code = AOM_CODEC_UNSUP_BITSTREAM;
                            return -(1 as core::ffi::c_int);
                        }
                        frame_decoding_finished = 1 as core::ffi::c_int;
                        (*pbi).seen_frame_header = 0 as core::ffi::c_int;
                        if (*cm).show_frame != 0
                            && (*(*cm).seq_params).order_hint_info.enable_order_hint == 0
                        {
                            (*cm).current_frame.frame_number = ((*cm)
                                .current_frame
                                .frame_number)
                                .wrapping_add(1);
                        }
                        current_block_123 = 0;
                    } else if (*pbi).ext_tile_debug == 0
                        && (*pbi).camera_frame_header_ready != 0
                    {
                        frame_decoding_finished = 1 as core::ffi::c_int;
                        decoded_payload_size = payload_size;
                        *p_data_end = data_end;
                        current_block_123 = 0;
                    } else if obu_header.type_0 as core::ffi::c_int
                        != OBU_FRAME as core::ffi::c_int
                    {
                        current_block_123 = 0;
                    } else {
                        obu_payload_offset = frame_header_size as size_t;
                        byte_alignment(&raw mut (*pbi).error, &raw mut rb);
                        current_block_123 = 1;
                    }
                }
                4 => {
                    current_block_123 = 1;
                }
                5 => {
                    decoded_payload_size = read_metadata(
                        pbi as *mut AV1Decoder,
                        data,
                        payload_size,
                        obu_header.has_extension != 0,
                    );
                    if (*pbi).error.error_code as core::ffi::c_uint
                        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    current_block_123 = 0;
                }
                8 => {
                    if (*pbi).camera_frame_header_ready == 0 {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    (*cm).tiles.large_scale = 1 as core::ffi::c_uint;
                    av1_set_single_tile_decoding_mode(cm);
                    decoded_payload_size = read_and_decode_one_tile_list(
                        pbi as *mut AV1Decoder,
                        &raw mut rb,
                        data,
                        data.offset(payload_size as isize),
                        p_data_end,
                        &raw mut frame_decoding_finished,
                    ) as size_t;
                    if (*pbi).error.error_code as core::ffi::c_uint
                        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    current_block_123 = 0;
                }
                15 => {
                    decoded_payload_size = read_padding(
                        &raw mut (*pbi).error,
                        data,
                        payload_size,
                    );
                    if (*pbi).error.error_code as core::ffi::c_uint
                        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    current_block_123 = 0;
                }
                _ => {
                    if payload_size > 0 as size_t
                        && get_last_nonzero_byte(data, payload_size) as core::ffi::c_int
                            == 0 as core::ffi::c_int
                    {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    decoded_payload_size = payload_size;
                    current_block_123 = 0;
                }
            }
            match current_block_123 {
                1 => {
                    if (*pbi).seen_frame_header == 0 {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    if obu_payload_offset > payload_size {
                        (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                        return -(1 as core::ffi::c_int);
                    }
                    decoded_payload_size = decoded_payload_size
                        .wrapping_add(
                            read_one_tile_group_obu(
                                pbi as *mut AV1Decoder,
                                &raw mut rb,
                                is_first_tg_obu_received,
                                data.offset(obu_payload_offset as isize),
                                data.offset(payload_size as isize),
                                p_data_end,
                                &raw mut frame_decoding_finished,
                                (obu_header.type_0 as core::ffi::c_int
                                    == OBU_FRAME as core::ffi::c_int) as core::ffi::c_int,
                            ) as size_t,
                        );
                    is_first_tg_obu_received = 0 as core::ffi::c_int;
                    if frame_decoding_finished != 0 {
                        (*pbi).seen_frame_header = 0 as core::ffi::c_int;
                        (*pbi).next_start_tile = 0 as core::ffi::c_int;
                    }
                    (*pbi).num_tile_groups += 1;
                }
                _ => {}
            }
            if decoded_payload_size > payload_size {
                (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                return -(1 as core::ffi::c_int);
            }
            while decoded_payload_size < payload_size {
                let fresh0 = decoded_payload_size;
                decoded_payload_size = decoded_payload_size.wrapping_add(1);
                let mut padding_byte: uint8_t = *data.offset(fresh0 as isize);
                if padding_byte as core::ffi::c_int != 0 as core::ffi::c_int {
                    (*pbi).error.error_code = AOM_CODEC_CORRUPT_FRAME;
                    return -(1 as core::ffi::c_int);
                }
            }
            data = data.offset(payload_size as isize);
        }
    }
    if (*pbi).error.error_code as core::ffi::c_uint
        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return -(1 as core::ffi::c_int);
    }
    return frame_decoding_finished;
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;
pub const CONFIG_MAX_DECODE_PROFILE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const CONFIG_NORMAL_TILE_MODE: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
}
