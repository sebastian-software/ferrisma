// Copyright 2021 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/diag.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn vsnprintf(
        __s: *mut core::ffi::c_char,
        __maxlen: size_t,
        __format: *const core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
}
pub type va_list<'a> = core::ffi::VaList<'a>;
pub const AVIF_DIAGNOSTICS_ERROR_BUFFER_SIZE: core::ffi::c_int = 256 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn avifDiagnosticsClearError(mut diag: *mut avifDiagnostics) {
    *((*diag).error).as_mut_ptr() = '\0' as i32 as core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn avifDiagnosticsPrintf(
    mut diag: *mut avifDiagnostics,
    mut format: *const core::ffi::c_char,
    mut args: ...
) {
    if diag.is_null() {
        return;
    }
    if *((*diag).error).as_mut_ptr() != 0 {
        return;
    }
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    vsnprintf(
        ((*diag).error).as_mut_ptr(),
        AVIF_DIAGNOSTICS_ERROR_BUFFER_SIZE as size_t,
        format,
        args_0,
    );
    (*diag)
        .error[(AVIF_DIAGNOSTICS_ERROR_BUFFER_SIZE - 1 as core::ffi::c_int) as usize] = '\0'
        as i32 as core::ffi::c_char;
}
