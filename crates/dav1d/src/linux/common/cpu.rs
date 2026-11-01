/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Mechanically translated from upstream/dav1d/src/cpu.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type Dav1dContext;
    fn __errno_location() -> *mut core::ffi::c_int;
    fn dav1d_log(c: *mut Dav1dContext, format: *const core::ffi::c_char, ...);
    fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> core::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_getaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type __cpu_mask = core::ffi::c_ulong;
pub type pthread_t = libc::pthread_t;
pub const ENOSYS: core::ffi::c_int = 38 as core::ffi::c_int;
#[no_mangle]
pub static mut dav1d_cpu_flags: core::ffi::c_uint = 0 as core::ffi::c_uint;
#[no_mangle]
pub static mut dav1d_cpu_flags_mask: core::ffi::c_uint = !(0 as core::ffi::c_uint);
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_cpu() {}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_set_cpu_flags_mask(mask: core::ffi::c_uint) {
    dav1d_cpu_flags_mask = mask;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_num_logical_processors(
    c: *mut Dav1dContext,
) -> core::ffi::c_int {
    let mut affinity: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    if pthread_getaffinity_np(
        pthread_self(),
        ::core::mem::size_of::<cpu_set_t>() as size_t,
        &raw mut affinity,
    ) == 0
    {
        return __sched_cpucount(
            ::core::mem::size_of::<cpu_set_t>() as size_t,
            &raw mut affinity,
        );
    }
    if !c.is_null() {
        dav1d_log(
            c,
            b"Unable to detect thread count, defaulting to single-threaded mode\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    return 1 as core::ffi::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_getauxval(
    mut type_0: core::ffi::c_ulong,
) -> core::ffi::c_ulong {
    *__errno_location() = ENOSYS;
    return 0 as core::ffi::c_ulong;
}
