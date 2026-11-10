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

// Mechanically translated from upstream/aom/aom_dsp/bitreader.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn od_ec_dec_init(
        dec: *mut od_ec_dec,
        buf: *const core::ffi::c_uchar,
        storage: uint32_t,
    );
    fn od_ec_dec_tell(dec: *const od_ec_dec) -> core::ffi::c_int;
    fn od_ec_dec_tell_frac(dec: *const od_ec_dec) -> uint32_t;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
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
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_reader_init(
    mut r: *mut aom_reader,
    mut buffer: *const uint8_t,
    mut size: size_t,
) -> core::ffi::c_int {
    if size != 0 && buffer.is_null() {
        return 1 as core::ffi::c_int;
    }
    (*r).buffer_end = buffer.offset(size as isize);
    (*r).buffer = buffer;
    od_ec_dec_init(&raw mut (*r).ec, buffer as *const core::ffi::c_uchar, size as uint32_t);
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_reader_find_begin(
    mut r: *mut aom_reader,
) -> *const uint8_t {
    return (*r).buffer;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_reader_find_end(mut r: *mut aom_reader) -> *const uint8_t {
    return (*r).buffer_end;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_reader_tell(mut r: *const aom_reader) -> uint32_t {
    return od_ec_dec_tell(&(*r).ec) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_reader_tell_frac(mut r: *const aom_reader) -> uint32_t {
    return od_ec_dec_tell_frac(&(*r).ec);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_reader_has_overflowed(
    mut r: *const aom_reader,
) -> core::ffi::c_int {
    let tell_bits: uint32_t = aom_reader_tell(r) as uint32_t;
    let tell_bytes: uint32_t = tell_bits.wrapping_add(7 as uint32_t)
        >> 3 as core::ffi::c_int;
    return (tell_bytes as ptrdiff_t
        > ((*r).buffer_end).offset_from((*r).buffer) as ptrdiff_t) as core::ffi::c_int;
}
