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

// Mechanically translated from upstream/aom/av1/encoder/lookahead.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_alloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        border: core::ffi::c_int,
        byte_alignment: core::ffi::c_int,
        alloc_pyramid: bool,
        alloc_y_plane_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_realloc_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        ss_x: core::ffi::c_int,
        ss_y: core::ffi::c_int,
        use_highbitdepth: core::ffi::c_int,
        border: core::ffi::c_int,
        byte_alignment: core::ffi::c_int,
        fb: *mut aom_codec_frame_buffer_t,
        cb: aom_get_frame_buffer_cb_fn_t,
        cb_priv: *mut core::ffi::c_void,
        alloc_pyramid: bool,
        alloc_y_plane_only: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn aom_free_frame_buffer(ybf: *mut YV12_BUFFER_CONFIG) -> core::ffi::c_int;
    fn aom_remove_metadata_from_frame_buffer(ybf: *mut YV12_BUFFER_CONFIG);
    fn aom_copy_metadata_to_frame_buffer(
        ybf: *mut YV12_BUFFER_CONFIG,
        arr: *const aom_metadata_array_t,
    ) -> core::ffi::c_int;
    fn av1_copy_and_extend_frame(
        src: *const YV12_BUFFER_CONFIG,
        dst: *mut YV12_BUFFER_CONFIG,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
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
pub type aom_enc_frame_flags_t = core::ffi::c_long;
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
pub type COMPRESSOR_STAGE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookahead_ctx {
    pub max_sz: core::ffi::c_int,
    pub write_idx: core::ffi::c_int,
    pub read_ctxs: [read_ctx; 2],
    pub buf: *mut lookahead_entry,
    pub push_frame_count: core::ffi::c_int,
    pub max_pre_frames: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookahead_entry {
    pub img: YV12_BUFFER_CONFIG,
    pub ts_start: int64_t,
    pub ts_end: int64_t,
    pub display_idx: core::ffi::c_int,
    pub flags: aom_enc_frame_flags_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_ctx {
    pub sz: core::ffi::c_int,
    pub read_idx: core::ffi::c_int,
    pub pop_sz: core::ffi::c_int,
    pub valid: core::ffi::c_int,
}
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const MAX_STAGES: C2RustUnnamed_11 = 2;
pub const LAP_STAGE: C2RustUnnamed_11 = 1;
pub const ENCODE_STAGE: C2RustUnnamed_11 = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const AOM_BORDER_IN_PIXELS: core::ffi::c_int = 288 as core::ffi::c_int;
pub const MAX_LAG_BUFFERS: core::ffi::c_int = 48 as core::ffi::c_int;
pub const MAX_LAP_BUFFERS: core::ffi::c_int = 48 as core::ffi::c_int;
pub const MAX_TOTAL_BUFFERS: core::ffi::c_int = MAX_LAG_BUFFERS + MAX_LAP_BUFFERS;
pub const MAX_PRE_FRAMES: core::ffi::c_int = 1 as core::ffi::c_int;
unsafe extern "C-unwind" fn pop(
    mut ctx: *mut lookahead_ctx,
    mut idx: *mut core::ffi::c_int,
) -> *mut lookahead_entry {
    let mut index: core::ffi::c_int = *idx;
    let mut buf: *mut lookahead_entry = ((*ctx).buf).offset(index as isize);
    if index < (*ctx).max_sz {} else {
        __assert_fail(
            b"index < ctx->max_sz\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            27 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 59],
                [core::ffi::c_char; 59],
            >(*b"struct lookahead_entry *pop(struct lookahead_ctx *, int *)\0"))
                .as_ptr(),
        );
    };
    index += 1;
    if index >= (*ctx).max_sz {
        index -= (*ctx).max_sz;
    }
    *idx = index;
    return buf;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_destroy(mut ctx: *mut lookahead_ctx) {
    if !ctx.is_null() {
        if !((*ctx).buf).is_null() {
            let mut i: core::ffi::c_int = 0;
            i = 0 as core::ffi::c_int;
            while i < (*ctx).max_sz {
                aom_free_frame_buffer(&raw mut (*((*ctx).buf).offset(i as isize)).img);
                i += 1;
            }
            free((*ctx).buf as *mut core::ffi::c_void);
        }
        free(ctx as *mut core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_init(
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
    mut use_highbitdepth: core::ffi::c_int,
    mut depth: core::ffi::c_int,
    mut border_in_pixels: core::ffi::c_int,
    mut byte_alignment: core::ffi::c_int,
    mut num_lap_buffers: core::ffi::c_int,
    mut is_all_intra: bool,
    mut alloc_pyramid: bool,
) -> *mut lookahead_ctx {
    let mut current_block: u64;
    let mut lag_in_frames: core::ffi::c_int = if 1 as core::ffi::c_int > depth {
        1 as core::ffi::c_int
    } else {
        depth
    };
    let max_pre_frames: uint8_t = (if is_all_intra as core::ffi::c_int != 0 {
        0 as core::ffi::c_int
    } else {
        MAX_PRE_FRAMES
    }) as uint8_t;
    depth += num_lap_buffers;
    depth = clamp(depth, 1 as core::ffi::c_int, MAX_TOTAL_BUFFERS);
    depth += max_pre_frames as core::ffi::c_int;
    let mut ctx: *mut lookahead_ctx = calloc(
        1 as size_t,
        ::core::mem::size_of::<lookahead_ctx>() as size_t,
    ) as *mut lookahead_ctx;
    if !ctx.is_null() {
        (*ctx).max_sz = depth;
        (*ctx).push_frame_count = 0 as core::ffi::c_int;
        (*ctx).max_pre_frames = max_pre_frames;
        (*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].pop_sz = (*ctx)
            .max_sz - (*ctx).max_pre_frames as core::ffi::c_int;
        (*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].valid = 1
            as core::ffi::c_int;
        if num_lap_buffers != 0 {
            (*ctx).read_ctxs[LAP_STAGE as core::ffi::c_int as usize].pop_sz = lag_in_frames;
            (*ctx).read_ctxs[LAP_STAGE as core::ffi::c_int as usize].valid = 1
                as core::ffi::c_int;
        }
        (*ctx).buf = calloc(
            depth as size_t,
            ::core::mem::size_of::<lookahead_entry>() as size_t,
        ) as *mut lookahead_entry;
        if ((*ctx).buf).is_null() {
            current_block = 0;
        } else {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            loop {
                if !(i < depth) {
                    current_block = 1;
                    break;
                }
                if aom_realloc_frame_buffer(
                    &raw mut (*((*ctx).buf).offset(i as isize)).img,
                    width,
                    height,
                    subsampling_x,
                    subsampling_y,
                    use_highbitdepth,
                    border_in_pixels,
                    byte_alignment,
                    0 as *mut aom_codec_frame_buffer_t,
                    None,
                    NULL,
                    alloc_pyramid,
                    0 as core::ffi::c_int,
                ) != 0
                {
                    current_block = 0;
                    break;
                }
                i += 1;
            }
        }
        match current_block {
            1 => {}
            _ => {
                av1_lookahead_destroy(ctx);
                return 0 as *mut lookahead_ctx;
            }
        }
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_full(
    mut ctx: *const lookahead_ctx,
) -> core::ffi::c_int {
    return ((*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].sz
        >= (*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].pop_sz)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_push(
    mut ctx: *mut lookahead_ctx,
    mut src: *const YV12_BUFFER_CONFIG,
    mut ts_start: int64_t,
    mut ts_end: int64_t,
    mut use_highbitdepth: core::ffi::c_int,
    mut alloc_pyramid: bool,
    mut flags: aom_enc_frame_flags_t,
) -> core::ffi::c_int {
    let mut width: core::ffi::c_int = (*src)
        .c2rust_unnamed_1
        .c2rust_unnamed
        .y_crop_width;
    let mut height: core::ffi::c_int = (*src)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .y_crop_height;
    let mut uv_width: core::ffi::c_int = (*src)
        .c2rust_unnamed_1
        .c2rust_unnamed
        .uv_crop_width;
    let mut uv_height: core::ffi::c_int = (*src)
        .c2rust_unnamed_2
        .c2rust_unnamed
        .uv_crop_height;
    let mut subsampling_x: core::ffi::c_int = (*src).subsampling_x;
    let mut subsampling_y: core::ffi::c_int = (*src).subsampling_y;
    let mut larger_dimensions: core::ffi::c_int = 0;
    let mut new_dimensions: core::ffi::c_int = 0;
    if (*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].valid
        == 1 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"ctx->read_ctxs[ENCODE_STAGE].valid == 1\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            110 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 128],
                [core::ffi::c_char; 128],
            >(
                *b"int av1_lookahead_push(struct lookahead_ctx *, const YV12_BUFFER_CONFIG *, int64_t, int64_t, int, _Bool, aom_enc_frame_flags_t)\0",
            ))
                .as_ptr(),
        );
    };
    if (*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].sz
        + (*ctx).max_pre_frames as core::ffi::c_int > (*ctx).max_sz
    {
        return 1 as core::ffi::c_int;
    }
    (*ctx).read_ctxs[ENCODE_STAGE as core::ffi::c_int as usize].sz += 1;
    if (*ctx).read_ctxs[LAP_STAGE as core::ffi::c_int as usize].valid != 0 {
        (*ctx).read_ctxs[LAP_STAGE as core::ffi::c_int as usize].sz += 1;
    }
    let mut buf: *mut lookahead_entry = pop(ctx, &raw mut (*ctx).write_idx);
    new_dimensions = (width != (*buf).img.c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        || height != (*buf).img.c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        || uv_width != (*buf).img.c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        || uv_height != (*buf).img.c2rust_unnamed_2.c2rust_unnamed.uv_crop_height)
        as core::ffi::c_int;
    larger_dimensions = (width > (*buf).img.c2rust_unnamed_1.c2rust_unnamed.y_crop_width
        || height > (*buf).img.c2rust_unnamed_2.c2rust_unnamed.y_crop_height
        || uv_width > (*buf).img.c2rust_unnamed_1.c2rust_unnamed.uv_crop_width
        || uv_height > (*buf).img.c2rust_unnamed_2.c2rust_unnamed.uv_crop_height)
        as core::ffi::c_int;
    if larger_dimensions == 0 || new_dimensions != 0 {} else {
        __assert_fail(
            b"!larger_dimensions || new_dimensions\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            128 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 128],
                [core::ffi::c_char; 128],
            >(
                *b"int av1_lookahead_push(struct lookahead_ctx *, const YV12_BUFFER_CONFIG *, int64_t, int64_t, int, _Bool, aom_enc_frame_flags_t)\0",
            ))
                .as_ptr(),
        );
    };
    if larger_dimensions != 0 {
        let mut new_img: YV12_BUFFER_CONFIG = yv12_buffer_config {
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
        memset(
            &raw mut new_img as *mut YV12_BUFFER_CONFIG as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<YV12_BUFFER_CONFIG>() as size_t,
        );
        if aom_alloc_frame_buffer(
            &raw mut new_img,
            width,
            height,
            subsampling_x,
            subsampling_y,
            use_highbitdepth,
            AOM_BORDER_IN_PIXELS,
            0 as core::ffi::c_int,
            alloc_pyramid,
            0 as core::ffi::c_int,
        ) != 0
        {
            return 1 as core::ffi::c_int;
        }
        aom_free_frame_buffer(&raw mut (*buf).img);
        (*buf).img = new_img;
    } else if new_dimensions != 0 {
        (*buf).img.c2rust_unnamed.c2rust_unnamed.y_width = (*src)
            .c2rust_unnamed
            .c2rust_unnamed
            .y_width;
        (*buf).img.c2rust_unnamed_0.c2rust_unnamed.y_height = (*src)
            .c2rust_unnamed_0
            .c2rust_unnamed
            .y_height;
        (*buf).img.c2rust_unnamed.c2rust_unnamed.uv_width = (*src)
            .c2rust_unnamed
            .c2rust_unnamed
            .uv_width;
        (*buf).img.c2rust_unnamed_0.c2rust_unnamed.uv_height = (*src)
            .c2rust_unnamed_0
            .c2rust_unnamed
            .uv_height;
        (*buf).img.c2rust_unnamed_1.c2rust_unnamed.y_crop_width = (*src)
            .c2rust_unnamed_1
            .c2rust_unnamed
            .y_crop_width;
        (*buf).img.c2rust_unnamed_2.c2rust_unnamed.y_crop_height = (*src)
            .c2rust_unnamed_2
            .c2rust_unnamed
            .y_crop_height;
        (*buf).img.c2rust_unnamed_1.c2rust_unnamed.uv_crop_width = (*src)
            .c2rust_unnamed_1
            .c2rust_unnamed
            .uv_crop_width;
        (*buf).img.c2rust_unnamed_2.c2rust_unnamed.uv_crop_height = (*src)
            .c2rust_unnamed_2
            .c2rust_unnamed
            .uv_crop_height;
        (*buf).img.subsampling_x = (*src).subsampling_x;
        (*buf).img.subsampling_y = (*src).subsampling_y;
    }
    av1_copy_and_extend_frame(src, &raw mut (*buf).img);
    (*buf).ts_start = ts_start;
    (*buf).ts_end = ts_end;
    (*buf).display_idx = (*ctx).push_frame_count;
    (*buf).flags = flags;
    (*ctx).push_frame_count += 1;
    aom_remove_metadata_from_frame_buffer(&raw mut (*buf).img);
    if !((*src).metadata).is_null()
        && aom_copy_metadata_to_frame_buffer(&raw mut (*buf).img, (*src).metadata) != 0
    {
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_pop(
    mut ctx: *mut lookahead_ctx,
    mut drain: core::ffi::c_int,
    mut stage: COMPRESSOR_STAGE,
) -> *mut lookahead_entry {
    let mut buf: *mut lookahead_entry = 0 as *mut lookahead_entry;
    if !ctx.is_null() {
        let mut read_ctx: *mut read_ctx = &raw mut *((*ctx).read_ctxs)
            .as_mut_ptr()
            .offset(stage as isize) as *mut read_ctx;
        if (*read_ctx).valid == 1 as core::ffi::c_int {} else {
            __assert_fail(
                b"read_ctx->valid == 1\0" as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                    as *const core::ffi::c_char,
                171 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 89],
                    [core::ffi::c_char; 89],
                >(
                    *b"struct lookahead_entry *av1_lookahead_pop(struct lookahead_ctx *, int, COMPRESSOR_STAGE)\0",
                ))
                    .as_ptr(),
            );
        };
        if (*read_ctx).sz != 0 && (drain != 0 || (*read_ctx).sz == (*read_ctx).pop_sz) {
            buf = pop(ctx, &raw mut (*read_ctx).read_idx);
            (*read_ctx).sz -= 1;
        }
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_peek(
    mut ctx: *mut lookahead_ctx,
    mut index: core::ffi::c_int,
    mut stage: COMPRESSOR_STAGE,
) -> *mut lookahead_entry {
    let mut buf: *mut lookahead_entry = 0 as *mut lookahead_entry;
    if ctx.is_null() {
        return buf;
    }
    let mut read_ctx: *mut read_ctx = &raw mut *((*ctx).read_ctxs)
        .as_mut_ptr()
        .offset(stage as isize) as *mut read_ctx;
    if (*read_ctx).valid == 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"read_ctx->valid == 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            188 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 90],
                [core::ffi::c_char; 90],
            >(
                *b"struct lookahead_entry *av1_lookahead_peek(struct lookahead_ctx *, int, COMPRESSOR_STAGE)\0",
            ))
                .as_ptr(),
        );
    };
    if index >= 0 as core::ffi::c_int {
        if index < (*read_ctx).sz {
            index += (*read_ctx).read_idx;
            if index >= (*ctx).max_sz {
                index -= (*ctx).max_sz;
            }
            buf = ((*ctx).buf).offset(index as isize);
        }
    } else if index < 0 as core::ffi::c_int {
        if -index <= (*ctx).max_pre_frames as core::ffi::c_int {
            index += (*read_ctx).read_idx;
            if index < 0 as core::ffi::c_int {
                index += (*ctx).max_sz;
            }
            buf = ((*ctx).buf).offset(index as isize);
        }
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_depth(
    mut ctx: *mut lookahead_ctx,
    mut stage: COMPRESSOR_STAGE,
) -> core::ffi::c_int {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            209 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"int av1_lookahead_depth(struct lookahead_ctx *, COMPRESSOR_STAGE)\0"))
                .as_ptr(),
        );
    };
    let mut read_ctx: *mut read_ctx = &raw mut *((*ctx).read_ctxs)
        .as_mut_ptr()
        .offset(stage as isize) as *mut read_ctx;
    if (*read_ctx).valid == 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"read_ctx->valid == 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            212 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 66],
                [core::ffi::c_char; 66],
            >(*b"int av1_lookahead_depth(struct lookahead_ctx *, COMPRESSOR_STAGE)\0"))
                .as_ptr(),
        );
    };
    return (*read_ctx).sz;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lookahead_pop_sz(
    mut ctx: *mut lookahead_ctx,
    mut stage: COMPRESSOR_STAGE,
) -> core::ffi::c_int {
    if !ctx.is_null() {} else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            217 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"int av1_lookahead_pop_sz(struct lookahead_ctx *, COMPRESSOR_STAGE)\0"))
                .as_ptr(),
        );
    };
    let mut read_ctx: *mut read_ctx = &raw mut *((*ctx).read_ctxs)
        .as_mut_ptr()
        .offset(stage as isize) as *mut read_ctx;
    if (*read_ctx).valid == 1 as core::ffi::c_int {} else {
        __assert_fail(
            b"read_ctx->valid == 1\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/lookahead.c\0" as *const u8
                as *const core::ffi::c_char,
            220 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 67],
                [core::ffi::c_char; 67],
            >(*b"int av1_lookahead_pop_sz(struct lookahead_ctx *, COMPRESSOR_STAGE)\0"))
                .as_ptr(),
        );
    };
    return (*read_ctx).pop_sz;
}

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
