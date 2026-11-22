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

// Platform layer of the one-source translation (route B, tools/aom_one_source.py).
// The prepared C inputs reach the C library only through these shims where
// Darwin and glibc differ; everything else is the same translated code.

/// C `assert` failure. The translated check runs only with debug assertions,
/// as the original's `assert` only runs without NDEBUG.
pub(crate) unsafe fn ferrisma_assert_fail(expression: *const c_char, file: *const c_char,
                                          line: core::ffi::c_uint, function: *const c_char) -> ! {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn __assert_rtn(function: *const c_char, file: *const c_char,
                            line: c_int, expression: *const c_char) -> !;
        }
        // SAFETY: the translation passes the original static NUL-terminated strings.
        __assert_rtn(function, file, line as c_int, expression)
    }
    #[cfg(target_os = "linux")]
    {
        extern "C" {
            fn __assert_fail(expression: *const c_char, file: *const c_char,
                             line: core::ffi::c_uint, function: *const c_char) -> !;
        }
        // SAFETY: as above.
        __assert_fail(expression, file, line, function)
    }
}

/// The C library's `stderr`.
pub(crate) unsafe fn ferrisma_stderr() -> *mut libc::FILE {
    #[cfg(target_os = "macos")]
    extern "C" { static mut __stderrp: *mut libc::FILE; }
    #[cfg(target_os = "macos")]
    // SAFETY: the C library initializes its standard streams before main.
    return __stderrp;
    #[cfg(target_os = "linux")]
    extern "C" { static mut stderr: *mut libc::FILE; }
    #[cfg(target_os = "linux")]
    // SAFETY: as above.
    return stderr;
}

/// The C library's `stdout`.
pub(crate) unsafe fn ferrisma_stdout() -> *mut libc::FILE {
    #[cfg(target_os = "macos")]
    extern "C" { static mut __stdoutp: *mut libc::FILE; }
    #[cfg(target_os = "macos")]
    // SAFETY: the C library initializes its standard streams before main.
    return __stdoutp;
    #[cfg(target_os = "linux")]
    extern "C" { static mut stdout: *mut libc::FILE; }
    #[cfg(target_os = "linux")]
    // SAFETY: as above.
    return stdout;
}

/// The calling thread's `errno`.
pub(crate) unsafe fn ferrisma_errno() -> *mut c_int {
    #[cfg(target_os = "macos")]
    return libc::__error();
    #[cfg(target_os = "linux")]
    return libc::__errno_location();
}

/// `feenableexcept` exactly where the original calls it: its GNU Linux Debug
/// build (`HAVE_FEXCEPT && CONFIG_DEBUG` in aom_encoder.c). Elsewhere the
/// original's macros are empty; -1 makes the translated restore a no-op.
pub(crate) unsafe fn ferrisma_feenableexcept(excepts: c_int) -> c_int {
    #[cfg(all(target_os = "linux", debug_assertions))]
    {
        extern "C" { fn feenableexcept(excepts: c_int) -> c_int; }
        // SAFETY: changes only the calling thread's floating-point trap mask.
        return feenableexcept(excepts);
    }
    #[cfg(not(all(target_os = "linux", debug_assertions)))]
    {
        let _ = excepts;
        -1
    }
}

/// `fedisableexcept`, under the same condition as `ferrisma_feenableexcept`.
pub(crate) unsafe fn ferrisma_fedisableexcept(excepts: c_int) -> c_int {
    #[cfg(all(target_os = "linux", debug_assertions))]
    {
        extern "C" { fn fedisableexcept(excepts: c_int) -> c_int; }
        // SAFETY: changes only the calling thread's floating-point trap mask.
        return fedisableexcept(excepts);
    }
    #[cfg(not(all(target_os = "linux", debug_assertions)))]
    {
        let _ = excepts;
        -1
    }
}

/// `jmp_buf` with each C library's size and alignment. The port maps
/// setjmp/longjmp to unwinding (jump.rs); the storage only keeps the original
/// layout of the records that contain it.
#[cfg(target_os = "macos")]
pub type jmp_buf = [c_int; 48];
#[cfg(target_os = "linux")]
pub type jmp_buf = [JmpBufTag; 1];
#[cfg(target_os = "linux")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JmpBufTag {
    registers: [core::ffi::c_long; 8],
    mask_was_saved: c_int,
    mask: [core::ffi::c_ulong; 16],
}
