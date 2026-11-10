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

// Mechanically translated from upstream/aom/av1/common/obu_util.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_uleb_decode(
        buffer: *const uint8_t,
        available: size_t,
        value: *mut uint64_t,
        length: *mut size_t,
    ) -> core::ffi::c_int;
    fn aom_rb_read_bit(rb: *mut aom_read_bit_buffer) -> core::ffi::c_int;
    fn aom_rb_read_literal(
        rb: *mut aom_read_bit_buffer,
        bits: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
unsafe extern "C-unwind" fn read_obu_size(
    mut data: *const uint8_t,
    mut bytes_available: size_t,
    obu_size: *mut size_t,
    length_field_size: *mut size_t,
) -> aom_codec_err_t {
    let mut u_obu_size: uint64_t = 0 as uint64_t;
    if aom_uleb_decode(data, bytes_available, &raw mut u_obu_size, length_field_size)
        != 0 as core::ffi::c_int
    {
        return AOM_CODEC_CORRUPT_FRAME;
    }
    if u_obu_size > UINT32_MAX as uint64_t {
        return AOM_CODEC_CORRUPT_FRAME;
    }
    *obu_size = u_obu_size as size_t;
    return AOM_CODEC_OK;
}
unsafe extern "C-unwind" fn read_obu_header(
    mut rb: *mut aom_read_bit_buffer,
    mut is_annexb: core::ffi::c_int,
    mut header: *mut ObuHeader,
) -> aom_codec_err_t {
    if rb.is_null() || header.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    let bit_buffer_byte_length: ptrdiff_t = ((*rb).bit_buffer_end)
        .offset_from((*rb).bit_buffer) as ptrdiff_t;
    if bit_buffer_byte_length < 1 as ptrdiff_t {
        return AOM_CODEC_CORRUPT_FRAME;
    }
    (*header).size = 1 as size_t;
    if aom_rb_read_bit(rb) != 0 as core::ffi::c_int {
        return AOM_CODEC_CORRUPT_FRAME;
    }
    (*header).type_0 = aom_rb_read_literal(rb, 4 as core::ffi::c_int) as OBU_TYPE;
    (*header).has_extension = aom_rb_read_bit(rb);
    (*header).has_size_field = aom_rb_read_bit(rb);
    if (*header).has_size_field == 0 && is_annexb == 0 {
        return AOM_CODEC_UNSUP_BITSTREAM;
    }
    aom_rb_read_bit(rb);
    if (*header).has_extension != 0 {
        if bit_buffer_byte_length == 1 as ptrdiff_t {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        (*header).size = ((*header).size).wrapping_add(1 as size_t);
        (*header).temporal_layer_id = aom_rb_read_literal(rb, 3 as core::ffi::c_int);
        (*header).spatial_layer_id = aom_rb_read_literal(rb, 2 as core::ffi::c_int);
        aom_rb_read_literal(rb, 3 as core::ffi::c_int);
    } else {
        (*header).temporal_layer_id = 0 as core::ffi::c_int;
        (*header).spatial_layer_id = 0 as core::ffi::c_int;
    }
    return AOM_CODEC_OK;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_read_obu_header(
    mut buffer: *mut uint8_t,
    mut buffer_length: size_t,
    mut consumed: *mut size_t,
    mut header: *mut ObuHeader,
    mut is_annexb: core::ffi::c_int,
) -> aom_codec_err_t {
    if buffer_length < 1 as size_t || consumed.is_null() || header.is_null() {
        return AOM_CODEC_INVALID_PARAM;
    }
    let mut rb: aom_read_bit_buffer = {
        let mut init = aom_read_bit_buffer {
            bit_buffer: buffer,
            bit_buffer_end: buffer.offset(buffer_length as isize),
            bit_offset: 0 as uint32_t,
            error_handler_data: NULL,
            error_handler: None,
        };
        init
    };
    let mut parse_result: aom_codec_err_t = read_obu_header(&raw mut rb, is_annexb, header);
    if parse_result as core::ffi::c_uint
        == AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        *consumed = (*header).size;
    }
    return parse_result;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_read_obu_header_and_size(
    mut data: *const uint8_t,
    mut bytes_available: size_t,
    mut is_annexb: core::ffi::c_int,
    mut obu_header: *mut ObuHeader,
    payload_size: *mut size_t,
    bytes_read: *mut size_t,
) -> aom_codec_err_t {
    let mut length_field_size_obu: size_t = 0 as size_t;
    let mut length_field_size_payload: size_t = 0 as size_t;
    let mut obu_size: size_t = 0 as size_t;
    let mut status: aom_codec_err_t = AOM_CODEC_OK;
    if is_annexb != 0 {
        status = read_obu_size(
            data,
            bytes_available,
            &raw mut obu_size,
            &raw mut length_field_size_obu,
        );
        if status as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return status;
        }
    }
    let mut rb: aom_read_bit_buffer = {
        let mut init = aom_read_bit_buffer {
            bit_buffer: data.offset(length_field_size_obu as isize),
            bit_buffer_end: data.offset(bytes_available as isize),
            bit_offset: 0 as uint32_t,
            error_handler_data: NULL,
            error_handler: None,
        };
        init
    };
    status = read_obu_header(&raw mut rb, is_annexb, obu_header);
    if status as core::ffi::c_uint
        != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return status;
    }
    if (*obu_header).has_size_field == 0 {
        if obu_size < (*obu_header).size {
            return AOM_CODEC_CORRUPT_FRAME;
        }
        *payload_size = obu_size.wrapping_sub((*obu_header).size);
    } else {
        status = read_obu_size(
            data
                .offset(length_field_size_obu as isize)
                .offset((*obu_header).size as isize),
            bytes_available
                .wrapping_sub(length_field_size_obu)
                .wrapping_sub((*obu_header).size),
            payload_size,
            &raw mut length_field_size_payload,
        );
        if status as core::ffi::c_uint
            != AOM_CODEC_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return status;
        }
    }
    *bytes_read = length_field_size_obu
        .wrapping_add((*obu_header).size)
        .wrapping_add(length_field_size_payload);
    return AOM_CODEC_OK;
}
