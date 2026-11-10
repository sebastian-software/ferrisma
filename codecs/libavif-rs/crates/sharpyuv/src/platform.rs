//! Target-runtime equivalents of macros expanded by the translation host SDK.
use core::ffi::{c_char, c_int};

#[cfg(target_os = "linux")]
#[link(name = "m")]
extern "C" {}

pub(crate) unsafe fn assertion(function: *const c_char, file: *const c_char,
                               line: c_int, expression: *const c_char) {
    #[cfg(debug_assertions)]
    {
        // SAFETY: C2Rust supplies the original static NUL-terminated strings.
        // The platform routine terminates; it cannot unwind into Rust or C.
        #[cfg(target_os = "macos")]
        {
            extern "C" { fn __assert_rtn(f: *const c_char, file: *const c_char, line: c_int, e: *const c_char) -> !; }
            __assert_rtn(function, file, line, expression);
        }
        #[cfg(target_os = "linux")]
        {
            extern "C" { fn __assert_fail(e: *const c_char, file: *const c_char, line: u32, f: *const c_char) -> !; }
            __assert_fail(expression, file, line as u32, function);
        }
    }
    #[cfg(not(debug_assertions))]
    let _ = (function, file, line, expression);
}
