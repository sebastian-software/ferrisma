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
- Extend the libaom SIMD layers of ADR 0056 and ADR 0058: remaining Neon
  hotspots (the 8-bit transforms, quantizer, and intra predictors;
  `encodetxb_neon.c`), the x86 counterparts of the same functions, the
  NASM-only high-bitdepth DC and V predictors, an AVX-512 measurement, the
  speed 7 case where SSE4.1 beats AVX2 on AMD EPYC 7763, and an original-test
  build with an Arm or x86 configuration that runs libaom's own SIMD-versus-C
  tests against the Rust kernels. The floating-point CNN and neural-network kernels
  and the reordering Hadamard kernels need a separate exactness decision.
- Reduce translation overheads that profiling shows in the Rust AOM encoder:
  zero-initialized C locals (memset, about 5% of an all-intra encode) and
  C2Rust bitfield accessors (2–4%), without changing behavior.
