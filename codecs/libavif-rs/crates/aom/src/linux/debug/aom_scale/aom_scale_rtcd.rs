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

// Mechanically translated from upstream/aom/aom_scale/aom_scale_rtcd.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use core::arch::asm;
extern "C-unwind" {
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type pthread_once_t = libc::pthread_once_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn xgetbv() -> uint64_t {
    let ecx: uint32_t = 0 as uint32_t;
    let mut eax: uint32_t = 0;
    let mut edx: uint32_t = 0;
    asm!(
        ".byte 0x0f, 0x01, 0xd0\n", "\n", lateout("ax") eax, lateout("dx") edx,
        inlateout("cx") ecx => _, options(preserves_flags, att_syntax)
    );
    return (edx as uint64_t) << 32 as core::ffi::c_int | eax as uint64_t;
}
pub const HAS_MMX: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const HAS_SSE: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const HAS_SSE2: core::ffi::c_int = 0x4 as core::ffi::c_int;
pub const HAS_SSE3: core::ffi::c_int = 0x8 as core::ffi::c_int;
pub const HAS_SSSE3: core::ffi::c_int = 0x10 as core::ffi::c_int;
pub const HAS_SSE4_1: core::ffi::c_int = 0x20 as core::ffi::c_int;
pub const HAS_AVX: core::ffi::c_int = 0x40 as core::ffi::c_int;
pub const HAS_AVX2: core::ffi::c_int = 0x80 as core::ffi::c_int;
pub const HAS_SSE4_2: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const HAS_AVX512: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const MMX_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 23 as core::ffi::c_int;
pub const SSE_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 25 as core::ffi::c_int;
pub const SSE2_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 26 as core::ffi::c_int;
pub const SSE3_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 0 as core::ffi::c_int;
pub const SSSE3_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 9 as core::ffi::c_int;
pub const SSE4_1_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 19 as core::ffi::c_int;
pub const SSE4_2_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 20 as core::ffi::c_int;
pub const AVX_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 27 as core::ffi::c_int | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int;
pub const AVX2_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 5 as core::ffi::c_int;
pub const AVX512_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 16 as core::ffi::c_int | (1 as core::ffi::c_uint) << 17 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 30 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 31 as core::ffi::c_int;
pub const AVX512_DL_BITS: core::ffi::c_uint = (1 as core::ffi::c_uint)
    << 1 as core::ffi::c_int | (1 as core::ffi::c_uint) << 6 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 8 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 10 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 11 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 12 as core::ffi::c_int
    | (1 as core::ffi::c_uint) << 14 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn x86_simd_caps() -> core::ffi::c_int {
    let mut flags: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut mask: core::ffi::c_uint = !(0 as core::ffi::c_uint);
    let mut max_cpuid_val: core::ffi::c_uint = 0;
    let mut reg_eax: core::ffi::c_uint = 0;
    let mut reg_ebx: core::ffi::c_uint = 0;
    let mut reg_ecx: core::ffi::c_uint = 0;
    let mut reg_edx: core::ffi::c_uint = 0;
    let mut env: *mut core::ffi::c_char = 0 as *mut core::ffi::c_char;
    env = getenv(b"AOM_SIMD_CAPS\0" as *const u8 as *const core::ffi::c_char);
    if !env.is_null() && *env as core::ffi::c_int != 0 {
        return strtol(env, 0 as *mut *mut core::ffi::c_char, 0 as core::ffi::c_int)
            as core::ffi::c_int;
    }
    env = getenv(b"AOM_SIMD_CAPS_MASK\0" as *const u8 as *const core::ffi::c_char);
    if !env.is_null() && *env as core::ffi::c_int != 0 {
        mask = strtoul(env, 0 as *mut *mut core::ffi::c_char, 0 as core::ffi::c_int)
            as core::ffi::c_uint;
    }
    let registers = core::arch::x86_64::__cpuid_count(0, 0); max_cpuid_val = registers.eax; reg_ebx = registers.ebx; reg_ecx = registers.ecx; reg_edx = registers.edx;
    if max_cpuid_val < 1 as core::ffi::c_uint {
        return 0 as core::ffi::c_int;
    }
    let registers = core::arch::x86_64::__cpuid_count(1, 0); reg_eax = registers.eax; reg_ebx = registers.ebx; reg_ecx = registers.ecx; reg_edx = registers.edx;
    flags
        |= (if reg_edx & (1 as core::ffi::c_uint) << 23 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 23 as core::ffi::c_int
        {
            HAS_MMX
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_edx & (1 as core::ffi::c_uint) << 25 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 25 as core::ffi::c_int
        {
            HAS_SSE
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_edx & (1 as core::ffi::c_uint) << 26 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 26 as core::ffi::c_int
        {
            HAS_SSE2
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 0 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 0 as core::ffi::c_int
        {
            HAS_SSE3
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
        {
            HAS_SSSE3
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 19 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 19 as core::ffi::c_int
        {
            HAS_SSE4_1
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    flags
        |= (if reg_ecx & (1 as core::ffi::c_uint) << 20 as core::ffi::c_int
            == (1 as core::ffi::c_uint) << 20 as core::ffi::c_int
        {
            HAS_SSE4_2
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    if reg_ecx
        & ((1 as core::ffi::c_uint) << 27 as core::ffi::c_int
            | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int)
        == (1 as core::ffi::c_uint) << 27 as core::ffi::c_int
            | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
    {
        if xgetbv() & 0x6 as uint64_t == 0x6 as uint64_t {
            flags |= HAS_AVX as core::ffi::c_uint;
            if max_cpuid_val >= 7 as core::ffi::c_uint {
                let registers = core::arch::x86_64::__cpuid_count(7, 0); reg_eax = registers.eax; reg_ebx = registers.ebx; reg_ecx = registers.ecx; reg_edx = registers.edx;
                flags
                    |= (if reg_ebx & (1 as core::ffi::c_uint) << 5 as core::ffi::c_int
                        == (1 as core::ffi::c_uint) << 5 as core::ffi::c_int
                    {
                        HAS_AVX2
                    } else {
                        0 as core::ffi::c_int
                    }) as core::ffi::c_uint;
                if xgetbv() & 0xe6 as uint64_t == 0xe6 as uint64_t {
                    flags
                        |= (if reg_ebx
                            & ((1 as core::ffi::c_uint) << 16 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 17 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 30 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 31 as core::ffi::c_int)
                            == (1 as core::ffi::c_uint) << 16 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 17 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 28 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 30 as core::ffi::c_int
                                | (1 as core::ffi::c_uint) << 31 as core::ffi::c_int
                            && reg_ecx
                                & ((1 as core::ffi::c_uint) << 1 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 6 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 8 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 10 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 11 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 12 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 14 as core::ffi::c_int)
                                == (1 as core::ffi::c_uint) << 1 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 6 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 8 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 9 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 10 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 11 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 12 as core::ffi::c_int
                                    | (1 as core::ffi::c_uint) << 14 as core::ffi::c_int
                        {
                            HAS_AVX512
                        } else {
                            0 as core::ffi::c_int
                        }) as core::ffi::c_uint;
                }
            }
        }
    }
    return (flags & mask) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn setup_rtcd_internal() {
    let mut flags: core::ffi::c_int = x86_simd_caps();
}
pub const PTHREAD_ONCE_INIT: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_scale_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}

extern "C" {
    fn strtol(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_long;
    fn strtoul(
        __nptr: *const core::ffi::c_char,
        __endptr: *mut *mut core::ffi::c_char,
        __base: core::ffi::c_int,
    ) -> core::ffi::c_ulong;
    fn getenv(__name: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
