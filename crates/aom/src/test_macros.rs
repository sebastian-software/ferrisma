// Optional subjects generated from unchanged production macros for original tests.
#[cfg(all(target_os = "macos", debug_assertions))]
include!("macros-macos-debug.rs");
#[cfg(all(target_os = "macos", not(debug_assertions)))]
include!("macros-macos-release.rs");
#[cfg(all(target_os = "linux", debug_assertions))]
include!("macros-linux-debug.rs");
#[cfg(all(target_os = "linux", not(debug_assertions)))]
include!("macros-linux-release.rs");
