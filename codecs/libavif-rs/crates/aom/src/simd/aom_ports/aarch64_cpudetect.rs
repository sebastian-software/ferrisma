/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_ports/aarch64_cpudetect.c and the
// environment helpers in aom_ports/arm_cpudetect.h (ADR 0056). Only the
// `__APPLE__` runtime-detection branch applies to the supported Arm target.
// The HAVE_* switches take the values of upstream's default arm64 Apple
// configuration (Neon, CRC32, DotProd, and I8MM enabled; SVE unavailable).
use core::ffi::{c_char, c_int, c_void, CStr};

// aom_ports/arm.h
pub const HAS_NEON: c_int = 1 << 0;
pub const HAS_ARM_CRC32: c_int = 1 << 1;
pub const HAS_NEON_DOTPROD: c_int = 1 << 2;
pub const HAS_NEON_I8MM: c_int = 1 << 3;
pub const HAS_SVE: c_int = 1 << 4;
pub const HAS_SVE2: c_int = 1 << 5;

extern "C" {
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strtol(text: *const c_char, end: *mut *mut c_char, base: c_int) -> core::ffi::c_long;
    fn sysctlbyname(name: *const c_char, old: *mut c_void, old_size: *mut usize,
                    new: *mut c_void, new_size: usize) -> c_int;
}

unsafe fn arm_cpu_env_flags(flags: *mut c_int) -> bool {
    let env = getenv(c"AOM_SIMD_CAPS".as_ptr());
    if !env.is_null() && *env != 0 {
        *flags = strtol(env, core::ptr::null_mut(), 0) as c_int;
        return true;
    }
    false
}

unsafe fn arm_cpu_env_mask() -> c_int {
    let env = getenv(c"AOM_SIMD_CAPS_MASK".as_ptr());
    if !env.is_null() && *env != 0 { strtol(env, core::ptr::null_mut(), 0) as c_int } else { !0 }
}

// sysctlbyname() parameter documentation for instruction set characteristics:
// https://developer.apple.com/documentation/kernel/1387446-sysctlbyname/determining_instruction_set_characteristics
#[inline]
unsafe fn have_feature(feature: &CStr) -> bool {
    let mut feature_present: i64 = 0;
    let mut size = core::mem::size_of::<i64>();
    if sysctlbyname(feature.as_ptr(), (&raw mut feature_present).cast(), &mut size,
                    core::ptr::null_mut(), 0) != 0 {
        return false;
    }
    feature_present != 0
}

unsafe fn arm_get_cpu_caps() -> c_int {
    let mut flags: c_int = 0;
    flags |= HAS_NEON;
    if have_feature(c"hw.optional.armv8_crc32") {
        flags |= HAS_ARM_CRC32;
    }
    if have_feature(c"hw.optional.arm.FEAT_DotProd") {
        flags |= HAS_NEON_DOTPROD;
    }
    if have_feature(c"hw.optional.arm.FEAT_I8MM") {
        flags |= HAS_NEON_I8MM;
    }
    flags
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_arm_cpu_caps() -> c_int {
    let mut flags: c_int = 0;
    if !arm_cpu_env_flags(&mut flags) {
        flags = arm_get_cpu_caps() & arm_cpu_env_mask();
    }

    // Restrict flags: FEAT_I8MM assumes that FEAT_DotProd is available.
    if flags & HAS_NEON_DOTPROD == 0 {
        flags &= !HAS_NEON_I8MM;
    }

    // Restrict flags: SVE assumes that FEAT_{DotProd,I8MM} are available.
    if flags & HAS_NEON_DOTPROD == 0 {
        flags &= !HAS_SVE;
    }
    if flags & HAS_NEON_I8MM == 0 {
        flags &= !HAS_SVE;
    }

    // Restrict flags: SVE2 assumes that FEAT_SVE is available.
    if flags & HAS_SVE == 0 {
        flags &= !HAS_SVE2;
    }

    flags
}
