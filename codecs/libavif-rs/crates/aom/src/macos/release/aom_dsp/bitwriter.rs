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

// Mechanically translated from upstream/aom/aom_dsp/bitwriter.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn od_ec_enc_init(enc: *mut od_ec_enc, size: uint32_t);
    fn od_ec_enc_clear(enc: *mut od_ec_enc);
    fn od_ec_enc_done(
        enc: *mut od_ec_enc,
        nbytes: *mut uint32_t,
    ) -> *mut core::ffi::c_uchar;
    fn od_ec_enc_tell(enc: *const od_ec_enc) -> core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type od_ec_enc_window = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct od_ec_enc {
    pub buf: *mut core::ffi::c_uchar,
    pub storage: uint32_t,
    pub offs: uint32_t,
    pub low: od_ec_enc_window,
    pub rng: uint16_t,
    pub cnt: int16_t,
    pub error: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_writer {
    pub pos: core::ffi::c_uint,
    pub buffer: *mut uint8_t,
    pub size: size_t,
    pub ec: od_ec_enc,
    pub allow_update_cdf: uint8_t,
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_start_encode(
    mut w: *mut aom_writer,
    mut source: *mut uint8_t,
    mut size: size_t,
) {
    (*w).buffer = source;
    (*w).size = size;
    (*w).pos = 0 as core::ffi::c_uint;
    od_ec_enc_init(&raw mut (*w).ec, 62025 as uint32_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_stop_encode(mut w: *mut aom_writer) -> core::ffi::c_int {
    let mut nb_bits: core::ffi::c_int = 0;
    let mut bytes: uint32_t = 0;
    let mut data: *mut core::ffi::c_uchar = 0 as *mut core::ffi::c_uchar;
    data = od_ec_enc_done(&raw mut (*w).ec, &raw mut bytes);
    if data.is_null() || bytes as size_t > (*w).size {
        od_ec_enc_clear(&raw mut (*w).ec);
        return -(1 as core::ffi::c_int);
    }
    nb_bits = od_ec_enc_tell(&raw mut (*w).ec);
    memcpy(
        (*w).buffer as *mut core::ffi::c_void,
        data as *const core::ffi::c_void,
        bytes as size_t,
    );
    (*w).pos = bytes as core::ffi::c_uint;
    od_ec_enc_clear(&raw mut (*w).ec);
    return nb_bits;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_tell_size(mut w: *mut aom_writer) -> core::ffi::c_int {
    let nb_bits: core::ffi::c_int = od_ec_enc_tell(&raw mut (*w).ec) as core::ffi::c_int;
    return nb_bits;
}

extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
