# Deferred work

These are follow-up candidates after behavioral parity, not changes to mix into
the mechanical translation:

- A safe Rust-facing API with explicit allocation and callback ownership.
- Consolidate repeated translated header declarations after ABI verification.
- Replace raw pointer operations only with independent behavioral evidence.
- Profile before changing algorithms, allocations, SIMD, or codec selection.
- Expand sanitizer, fuzzing, and platform evidence for translated unsafe code.
