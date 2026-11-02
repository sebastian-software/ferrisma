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

// Mechanically translated from upstream/aom/aom/src/aom_integer.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
static mut kMaximumLeb128Size: size_t = 8 as size_t;
static mut kLeb128ByteMask: uint8_t = 0x7f as uint8_t;
static mut kMaximumLeb128Value: uint64_t = UINT32_MAX as uint64_t;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_uleb_size_in_bytes(mut value: uint64_t) -> size_t {
    let mut size: size_t = 0 as size_t;
    loop {
        size = size.wrapping_add(1);
        value >>= 7 as core::ffi::c_int;
        if !(value != 0 as uint64_t) {
            break;
        }
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_uleb_decode(
    mut buffer: *const uint8_t,
    mut available: size_t,
    mut value: *mut uint64_t,
    mut length: *mut size_t,
) -> core::ffi::c_int {
    if !buffer.is_null() && !value.is_null() {
        *value = 0 as uint64_t;
        let mut i: size_t = 0 as size_t;
        while i < kMaximumLeb128Size && i < available {
            let decoded_byte: uint8_t = (*buffer.offset(i as isize) as core::ffi::c_int
                & kLeb128ByteMask as core::ffi::c_int) as uint8_t;
            *value |= (decoded_byte as uint64_t) << i.wrapping_mul(7 as size_t);
            if *buffer.offset(i as isize) as core::ffi::c_int >> 7 as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                if !length.is_null() {
                    *length = i.wrapping_add(1 as size_t);
                }
                if *value > UINT32_MAX as uint64_t {
                    return -(1 as core::ffi::c_int);
                }
                return 0 as core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
    }
    return -(1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_uleb_encode(
    mut value: uint64_t,
    mut available: size_t,
    mut coded_value: *mut uint8_t,
    mut coded_size: *mut size_t,
) -> core::ffi::c_int {
    let leb_size: size_t = aom_uleb_size_in_bytes(value) as size_t;
    if value > kMaximumLeb128Value || leb_size > kMaximumLeb128Size
        || leb_size > available || coded_value.is_null() || coded_size.is_null()
    {
        return -(1 as core::ffi::c_int);
    }
    let mut i: size_t = 0 as size_t;
    while i < leb_size {
        let mut byte: uint8_t = (value & 0x7f as uint64_t) as uint8_t;
        value >>= 7 as core::ffi::c_int;
        if value != 0 as uint64_t {
            byte = (byte as core::ffi::c_int | 0x80 as core::ffi::c_int) as uint8_t;
        }
        *coded_value.offset(i as isize) = byte;
        i = i.wrapping_add(1);
    }
    *coded_size = leb_size;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_uleb_encode_fixed_size(
    mut value: uint64_t,
    mut available: size_t,
    mut pad_to_size: size_t,
    mut coded_value: *mut uint8_t,
    mut coded_size: *mut size_t,
) -> core::ffi::c_int {
    if value > kMaximumLeb128Value || coded_value.is_null() || coded_size.is_null()
        || available < pad_to_size || pad_to_size > kMaximumLeb128Size
    {
        return -(1 as core::ffi::c_int);
    }
    let limit: uint64_t = ((1 as core::ffi::c_ulonglong)
        << (7 as size_t).wrapping_mul(pad_to_size)) as uint64_t;
    if value >= limit {
        return -(1 as core::ffi::c_int);
    }
    let mut i: size_t = 0 as size_t;
    while i < pad_to_size {
        let mut byte: uint8_t = (value & 0x7f as uint64_t) as uint8_t;
        value >>= 7 as core::ffi::c_int;
        if i < pad_to_size.wrapping_sub(1 as size_t) {
            byte = (byte as core::ffi::c_int | 0x80 as core::ffi::c_int) as uint8_t;
        }
        *coded_value.offset(i as isize) = byte;
        i = i.wrapping_add(1);
    }
    *coded_size = pad_to_size;
    return 0 as core::ffi::c_int;
}
