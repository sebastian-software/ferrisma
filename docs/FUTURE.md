# Deferred work

These are follow-up candidates after behavioral parity, not changes to mix into
the mechanical translation:

- A safe Rust-facing API with explicit allocation and callback ownership.
- Consolidate repeated translated header declarations after ABI verification.
- Replace raw pointer operations only with independent behavioral evidence.
- Profile before changing algorithms, allocations, SIMD, or codec selection.
- Expand sanitizer, fuzzing, and platform evidence for translated unsafe code.
- Extend the explicit noncontracting C reference contract (ADR 0024) to additional
  toolchains and targets with independent differential evidence.
- Generate and validate platform-specific libc/ABI declarations for Linux,
  Windows, and other architectures before enabling those Cargo targets.
- Exercise optional MP4Box golden tests, FuzzTest, and ComplianceWarden, and
  adapt the preserved upstream CI matrix to the Rust build.
- Stabilize translated opaque external types and the nightly toolchain choice
  without changing allocation or callback behavior.
- Extend dav1d's portable evidence with the optional external Argon corpus,
  continuous fuzzing, and additional sanitizer coverage before broader claims.
- Evaluate dav1d SIMD, safe wrappers, and existing Rust decoder crates as
  separate work; retain the mechanical scalar implementation as the reference.
- Evaluate SharpYUV SIMD, safe wrappers, source-declaration consolidation, and
  existing Rust conversion crates only as separately measured follow-up work.
- Extend the libaom SIMD layer of ADR 0056: remaining Neon hotspots (the
  8-bit transforms, quantizer, and intra predictors; `encodetxb_neon.c`),
  AVX2 kernels with the x86 dispatcher on GNU Linux, and an original-test
  build with an Arm configuration that runs libaom's own SIMD-versus-C tests
  against the Rust kernels. The floating-point CNN and neural-network kernels
  and the reordering Hadamard kernels need a separate exactness decision.
- Reduce translation overheads that profiling shows in the Rust AOM encoder:
  zero-initialized C locals (memset, about 5% of an all-intra encode) and
  C2Rust bitfield accessors (2–4%), without changing behavior.
