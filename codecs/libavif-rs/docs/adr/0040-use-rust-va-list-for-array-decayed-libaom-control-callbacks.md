# Use Rust VaList for array-decayed libaom control callbacks

Map the selected GNU x86-64 `aom_codec_control_fn_t` parameter to Rust's
`core::ffi::VaList`, matching the translated callback definitions and variadic
dispatcher. Clang decays glibc's one-element `va_list` array to a pointer in
this typedef; retaining that raw pointer creates incompatible nominal Rust
function types. Rust's VaList implements the platform's C parameter ABI.

Apply this narrow, inventoried correction only to the original callback typedef
with the verified array representation. Keep the raw record, callback map
layout, control dispatch, argument extraction, and original C inputs unchanged.
Do not coerce function pointers or reinterpret their arguments. Require native
Linux compilation and the original control/API tests before accepting it.

The [Rust VaList contract](https://doc.rust-lang.org/nightly/core/ffi/struct.VaList.html)
describes its C layout, parameter ABI, and `va_copy`/`va_end` behavior. The
project's compiler remains pinned to `nightly-2026-09-07`.
