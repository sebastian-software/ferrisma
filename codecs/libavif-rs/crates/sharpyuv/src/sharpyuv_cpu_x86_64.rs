// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// CPU detection
//
// Author: Christian Duvivier (cduvivier@google.com)

// Included x86 CPU-query source; SIMD conversion remains disabled.
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//

// Mechanically translated from upstream/libwebp/sharpyuv/sharpyuv_cpu.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use core::arch::asm;
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type CPUFeature = core::ffi::c_uint;
pub const kMSA: CPUFeature = 9;
pub const kMIPSdspR2: CPUFeature = 8;
pub const kMIPS32: CPUFeature = 7;
pub const kNEON: CPUFeature = 6;
pub const kAVX2: CPUFeature = 5;
pub const kAVX: CPUFeature = 4;
pub const kSSE4_1: CPUFeature = 3;
pub const kSlowSSSE3: CPUFeature = 2;
pub const kSSE3: CPUFeature = 1;
pub const kSSE2: CPUFeature = 0;
pub type VP8CPUInfo = Option<unsafe extern "C" fn(CPUFeature) -> core::ffi::c_int>;
#[inline]
unsafe extern "C" fn GetCPUInfo(
    mut cpu_info: *mut core::ffi::c_int,
    mut info_type: core::ffi::c_int,
) {
    let registers = core::arch::x86_64::__cpuid_count(info_type as u32, 0);
    *cpu_info.offset(0) = registers.eax as core::ffi::c_int;
    *cpu_info.offset(1) = registers.ebx as core::ffi::c_int;
    *cpu_info.offset(2) = registers.ecx as core::ffi::c_int;
    *cpu_info.offset(3) = registers.edx as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn xgetbv() -> uint64_t {
    let ecx: uint32_t = 0 as uint32_t;
    let mut eax: uint32_t = 0;
    let mut edx: uint32_t = 0;
    asm!(
        ".byte 0x0f, 0x01, 0xd0\n", "\n", lateout("ax") eax, lateout("dx") edx,
        inlateout("cx") ecx => _, options(preserves_flags, att_syntax)
    );
    return (edx as uint64_t) << 32 as core::ffi::c_int | eax as uint64_t;
}
unsafe extern "C" fn CheckSlowModel(mut info: core::ffi::c_int) -> core::ffi::c_int {
    static mut kSlowModels: [uint8_t; 6] = [
        0x37 as core::ffi::c_int as uint8_t,
        0x4a as core::ffi::c_int as uint8_t,
        0x4d as core::ffi::c_int as uint8_t,
        0x1c as core::ffi::c_int as uint8_t,
        0x26 as core::ffi::c_int as uint8_t,
        0x27 as core::ffi::c_int as uint8_t,
    ];
    let model: uint32_t = ((info & 0xf0000 as core::ffi::c_int) >> 12 as core::ffi::c_int
        | info >> 4 as core::ffi::c_int & 0xf as core::ffi::c_int) as uint32_t;
    let family: uint32_t = (info >> 8 as core::ffi::c_int & 0xf as core::ffi::c_int)
        as uint32_t;
    if family == 0x6 as uint32_t {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i
            < (::core::mem::size_of::<[uint8_t; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<uint8_t>() as usize)
        {
            if model == kSlowModels[i as usize] as uint32_t {
                return 1 as core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn x86CPUInfo(mut feature: CPUFeature) -> core::ffi::c_int {
    let mut max_cpuid_value: core::ffi::c_int = 0;
    let mut cpu_info: [core::ffi::c_int; 4] = [0; 4];
    let mut is_intel: core::ffi::c_int = 0 as core::ffi::c_int;
    GetCPUInfo(cpu_info.as_mut_ptr(), 0 as core::ffi::c_int);
    max_cpuid_value = cpu_info[0 as core::ffi::c_int as usize];
    if max_cpuid_value < 1 as core::ffi::c_int {
        return 0 as core::ffi::c_int
    } else {
        let VENDOR_ID_INTEL_EBX: core::ffi::c_int = 0x756e6547 as core::ffi::c_int;
        let VENDOR_ID_INTEL_EDX: core::ffi::c_int = 0x49656e69 as core::ffi::c_int;
        let VENDOR_ID_INTEL_ECX: core::ffi::c_int = 0x6c65746e as core::ffi::c_int;
        is_intel = (cpu_info[1 as core::ffi::c_int as usize] == VENDOR_ID_INTEL_EBX
            && cpu_info[2 as core::ffi::c_int as usize] == VENDOR_ID_INTEL_ECX
            && cpu_info[3 as core::ffi::c_int as usize] == VENDOR_ID_INTEL_EDX)
            as core::ffi::c_int;
    }
    GetCPUInfo(cpu_info.as_mut_ptr(), 1 as core::ffi::c_int);
    if feature as core::ffi::c_uint == kSSE2 as core::ffi::c_int as core::ffi::c_uint {
        return (cpu_info[3 as core::ffi::c_int as usize]
            & (1 as core::ffi::c_int) << 26 as core::ffi::c_int != 0)
            as core::ffi::c_int;
    }
    if feature as core::ffi::c_uint == kSSE3 as core::ffi::c_int as core::ffi::c_uint {
        return (cpu_info[2 as core::ffi::c_int as usize]
            & (1 as core::ffi::c_int) << 0 as core::ffi::c_int != 0) as core::ffi::c_int;
    }
    if feature as core::ffi::c_uint
        == kSlowSSSE3 as core::ffi::c_int as core::ffi::c_uint
    {
        if is_intel != 0
            && cpu_info[2 as core::ffi::c_int as usize]
                & (1 as core::ffi::c_int) << 9 as core::ffi::c_int != 0
        {
            return CheckSlowModel(cpu_info[0 as core::ffi::c_int as usize]);
        }
        return 0 as core::ffi::c_int;
    }
    if feature as core::ffi::c_uint == kSSE4_1 as core::ffi::c_int as core::ffi::c_uint {
        return (cpu_info[2 as core::ffi::c_int as usize]
            & (1 as core::ffi::c_int) << 19 as core::ffi::c_int != 0)
            as core::ffi::c_int;
    }
    if feature as core::ffi::c_uint == kAVX as core::ffi::c_int as core::ffi::c_uint {
        if cpu_info[2 as core::ffi::c_int as usize] & 0x18000000 as core::ffi::c_int
            == 0x18000000 as core::ffi::c_int
        {
            return (xgetbv() & 0x6 as uint64_t == 0x6 as uint64_t) as core::ffi::c_int;
        }
    }
    if feature as core::ffi::c_uint == kAVX2 as core::ffi::c_int as core::ffi::c_uint {
        if x86CPUInfo(kAVX) != 0 && max_cpuid_value >= 7 as core::ffi::c_int {
            GetCPUInfo(cpu_info.as_mut_ptr(), 7 as core::ffi::c_int);
            return (cpu_info[1 as core::ffi::c_int as usize]
                & (1 as core::ffi::c_int) << 5 as core::ffi::c_int != 0)
                as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub static mut SharpYuvGetCPUInfo: VP8CPUInfo = unsafe {
    Some(x86CPUInfo as unsafe extern "C" fn(CPUFeature) -> core::ffi::c_int)
};
