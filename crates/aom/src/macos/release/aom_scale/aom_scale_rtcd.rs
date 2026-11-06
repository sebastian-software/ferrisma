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
extern "C-unwind" {
    fn aom_arm_cpu_caps() -> core::ffi::c_int;
}
pub type pthread_once_t = libc::pthread_once_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
unsafe extern "C-unwind" fn setup_rtcd_internal() {
    let mut flags: core::ffi::c_int = aom_arm_cpu_caps();
}
pub const _PTHREAD_ONCE_SIG_init: core::ffi::c_int = 0x30b1bcba as core::ffi::c_int;
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = libc::PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func as Option<unsafe extern "C-unwind" fn() -> ()>);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_scale_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}

extern "C" {
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
