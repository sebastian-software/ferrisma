// libjpeg-turbo 3.2.1, src/jconfig.h.in:
//
// /* Version ID for the JPEG library.
//  * Might be useful for tests like "#if JPEG_LIB_VERSION >= 60".
//  */
//! Translation of `src/jconfig.h.in` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! The values are those that the reference oracle's generated `jconfig.h`
//! defines (`oracle.json`, `jconfig_h`); `WITH_SIMD` and
//! `RIGHT_SHIFT_IS_UNSIGNED` are undefined there and have no constant (ADR
//! 0005). The `_WIN32` branch does not apply to the CI hosts.

// upstream: src/jconfig.h.in JPEG_LIB_VERSION
pub const JPEG_LIB_VERSION: i32 = 62;

// upstream: src/jconfig.h.in LIBJPEG_TURBO_VERSION
/// A bare token (`3.2.1`) in C, usable only through stringification.
pub const LIBJPEG_TURBO_VERSION: &str = "3.2.1";

// upstream: src/jconfig.h.in LIBJPEG_TURBO_VERSION_NUMBER
pub const LIBJPEG_TURBO_VERSION_NUMBER: i32 = 3_002_001;

// upstream: src/jconfig.h.in C_ARITH_CODING_SUPPORTED
pub const C_ARITH_CODING_SUPPORTED: i32 = 1;

// upstream: src/jconfig.h.in D_ARITH_CODING_SUPPORTED
pub const D_ARITH_CODING_SUPPORTED: i32 = 1;

// upstream: src/jconfig.h.in MEM_SRCDST_SUPPORTED
pub const MEM_SRCDST_SUPPORTED: i32 = 1;

// upstream: src/jconfig.h.in BITS_IN_JSAMPLE
pub const BITS_IN_JSAMPLE: i32 = 8;
