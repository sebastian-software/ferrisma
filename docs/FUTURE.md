# Deferred work

These are follow-up candidates after behavioral parity, not changes to mix into
the mechanical translation:

- A safe Rust-facing API with explicit allocation and callback ownership.
- Consolidate repeated translated header declarations after ABI verification.
- Replace raw pointer operations only with independent behavioral evidence.
- Profile before changing algorithms, allocations, SIMD, or codec selection.
- Expand sanitizer, fuzzing, and platform evidence for translated unsafe code.
- Choose an explicit cross-compiler floating-point contraction contract before
  claiming exact decoded RGB byte parity (ADR 0005).
- Generate and validate platform-specific libc/ABI declarations for Linux,
  Windows, and other architectures before enabling those Cargo targets.
- Exercise optional MP4Box golden tests, FuzzTest, and ComplianceWarden, and
  adapt the preserved upstream CI matrix to the Rust build.
- Stabilize translated opaque external types and the nightly toolchain choice
  without changing allocation or callback behavior.
