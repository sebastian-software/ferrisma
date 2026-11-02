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

// Mechanically translated from upstream/aom/av1/common/av1_rtcd.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type pthread_once_t = libc::pthread_once_t;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_rtcd() {
    aom_once(Some(setup_rtcd_internal as unsafe extern "C-unwind" fn() -> ()));
}
pub const PTHREAD_ONCE_INIT: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn setup_rtcd_internal() {}
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func);
}

extern "C" {
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
