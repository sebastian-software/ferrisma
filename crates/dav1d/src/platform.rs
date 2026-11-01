//! Target runtime for the original source assertion macro.
#[cfg(target_os = "macos")]
use core::ffi::{c_char, c_int};

// Zero-length fields with these types retain C member alignment without
// changing the member's value, indexing, or size.
#[derive(Copy, Clone)]
#[repr(align(2))]
pub struct Align2;
#[derive(Copy, Clone)]
#[repr(align(4))]
pub struct Align4;
#[derive(Copy, Clone)]
#[repr(align(8))]
pub struct Align8;
#[derive(Copy, Clone)]
#[repr(align(16))]
pub struct Align16;
#[derive(Copy, Clone)]
#[repr(align(32))]
pub struct Align32;
#[derive(Copy, Clone)]
#[repr(align(64))]
pub struct Align64;

#[cfg(target_os = "macos")]
pub(crate) unsafe fn assertion(function: *const c_char, file: *const c_char,
                               line: c_int, expression: *const c_char) {
    #[cfg(debug_assertions)]
    {
        extern "C" {
            fn __assert_rtn(function: *const c_char, file: *const c_char,
                            line: c_int, expression: *const c_char) -> !;
        }
        // SAFETY: Translation supplies the original static NUL-terminated
        // strings. The C assertion runtime terminates without unwinding.
        __assert_rtn(function, file, line, expression);
    }
    #[cfg(not(debug_assertions))]
    let _ = (function, file, line, expression);
}
