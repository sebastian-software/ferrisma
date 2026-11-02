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

// Mechanically translated from upstream/aom/av1/encoder/hash.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CRC32C {
    pub table: [[uint32_t; 256]; 8],
}
pub type CRC32C = _CRC32C;
pub const POLY: core::ffi::c_uint = 0x82f63b78 as core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_crc32c_calculator_init(mut p_crc32c: *mut CRC32C) {
    let mut crc: uint32_t = 0;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < 256 as core::ffi::c_int {
        crc = n as uint32_t;
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        crc = if crc & 1 as uint32_t != 0 {
            crc >> 1 as core::ffi::c_int ^ POLY as uint32_t
        } else {
            crc >> 1 as core::ffi::c_int
        };
        (*p_crc32c).table[0 as core::ffi::c_int as usize][n as usize] = crc;
        n += 1;
    }
    let mut n_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while n_0 < 256 as core::ffi::c_int {
        crc = (*p_crc32c).table[0 as core::ffi::c_int as usize][n_0 as usize];
        let mut k: core::ffi::c_int = 1 as core::ffi::c_int;
        while k < 8 as core::ffi::c_int {
            crc = (*p_crc32c)
                .table[0 as core::ffi::c_int as usize][(crc & 0xff as uint32_t) as usize]
                ^ crc >> 8 as core::ffi::c_int;
            (*p_crc32c).table[k as usize][n_0 as usize] = crc;
            k += 1;
        }
        n_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_crc32c_value_c(
    mut c: *mut core::ffi::c_void,
    mut buf: *const uint8_t,
    mut len: size_t,
) -> uint32_t {
    let mut next: *const uint8_t = buf;
    let mut crc: uint64_t = 0;
    let mut p: *mut CRC32C = c as *mut CRC32C;
    crc = (0 as core::ffi::c_uint ^ 0xffffffff as core::ffi::c_uint) as uint64_t;
    while len != 0 && next as uintptr_t & 7 as uintptr_t != 0 as uintptr_t {
        let fresh0 = next;
        next = next.offset(1);
        crc = (*p)
            .table[0 as core::ffi::c_int
            as usize][((crc ^ *fresh0 as uint64_t) & 0xff as uint64_t) as usize]
            as uint64_t ^ crc >> 8 as core::ffi::c_int;
        len = len.wrapping_sub(1);
    }
    while len >= 8 as size_t {
        crc ^= *(next as *mut uint64_t);
        crc = ((*p)
            .table[7 as core::ffi::c_int as usize][(crc & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[6 as core::ffi::c_int
                as usize][(crc >> 8 as core::ffi::c_int & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[5 as core::ffi::c_int
                as usize][(crc >> 16 as core::ffi::c_int & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[4 as core::ffi::c_int
                as usize][(crc >> 24 as core::ffi::c_int & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[3 as core::ffi::c_int
                as usize][(crc >> 32 as core::ffi::c_int & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[2 as core::ffi::c_int
                as usize][(crc >> 40 as core::ffi::c_int & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[1 as core::ffi::c_int
                as usize][(crc >> 48 as core::ffi::c_int & 0xff as uint64_t) as usize]
            ^ (*p)
                .table[0 as core::ffi::c_int
                as usize][(crc >> 56 as core::ffi::c_int) as usize]) as uint64_t;
        next = next.offset(8 as core::ffi::c_int as isize);
        len = len.wrapping_sub(8 as size_t);
    }
    while len != 0 {
        let fresh1 = next;
        next = next.offset(1);
        crc = (*p)
            .table[0 as core::ffi::c_int
            as usize][((crc ^ *fresh1 as uint64_t) & 0xff as uint64_t) as usize]
            as uint64_t ^ crc >> 8 as core::ffi::c_int;
        len = len.wrapping_sub(1);
    }
    return crc as uint32_t ^ 0xffffffff as uint32_t;
}
