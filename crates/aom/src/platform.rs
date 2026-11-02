//! Target runtime for the original source assertion macro.
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

// The single field stays at offset zero. These storage wrappers preserve
// explicit alignment on original C local and static objects; all generated
// value operations continue to address the field itself.
#[repr(C, align(2))]
pub struct Aligned2<T>(pub T);
#[repr(C, align(4))]
pub struct Aligned4<T>(pub T);
#[repr(C, align(8))]
pub struct Aligned8<T>(pub T);
#[repr(C, align(16))]
pub struct Aligned16<T>(pub T);
#[repr(C, align(32))]
pub struct Aligned32<T>(pub T);
#[repr(C, align(64))]
pub struct Aligned64<T>(pub T);

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

#[derive(Copy, Clone)]
#[repr(align(128))]
pub struct Align128;
#[derive(Copy, Clone)]
#[repr(align(256))]
pub struct Align256;
#[repr(C, align(128))]
pub struct Aligned128<T>(pub T);
#[repr(C, align(256))]
pub struct Aligned256<T>(pub T);
