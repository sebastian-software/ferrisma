# Port SIMD after the portable decoder, as intrinsics on both architectures

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-09

## Context

libjpeg-turbo's speed comes partly from SIMD for the IDCT, upsampling and color
conversion (`simd/`). On AArch64 that code is C with NEON intrinsics (about 7k
lines in `simd/arm`); upstream replaced its former GAS assembly with it and
reports it as faster in several cases. On x86-64 it is NASM assembly (about 14k
lines in `simd/x86_64`) for SSE2 and AVX2. The serial Huffman decoder has no
SIMD and dominates decode time on photographs (Ferrixel's measurement
estimates about 70 % on a 12 Mpx photo).

## Decision

1. The portable decoder comes first and is verified against the `WITH_SIMD=0`
   oracle ([ADR 0002](0002-differential-oracle.md)).
2. NEON kernels are translated from `simd/arm` C intrinsics to `std::arch::aarch64`
   intrinsics, one Rust function per C function, dispatched where upstream's
   `jsimd.c` dispatches.
3. x86-64 kernels are rewritten from the NASM sources as `std::arch::x86_64`
   intrinsics (SSE2 and AVX2), with runtime feature detection replacing
   upstream's CPUID code. This is the one place where the port cannot be
   line-by-line, so each kernel is proven bit-identical to the assembly through
   probes against a `WITH_SIMD=1` oracle, and its speed is measured against it.
4. Every kernel's output equals the portable path's output wherever upstream's
   SIMD and C paths agree; where upstream's SIMD differs from its C code (for
   example rounding in the fast IDCT), the SIMD oracle is the reference and the
   difference is recorded.
5. All SIMD modules need the unsafe ADR required by
   [ADR 0003](0003-toolchain-and-unsafe-policy.md) before work starts.
6. Speed is measured against libjpeg-turbo, never estimated. `tools/bench.py`
   decodes the same inputs through the port's safe API (`examples/bench.rs`)
   and through the oracle's libjpeg API (the probe `wp9/bench`), requires the
   same output from both, and reports the median time per decode and the
   ratio of the port's time to C's. It compares each SIMD configuration with
   the `simd` oracle and the portable decoder with the reference oracle, and
   has cases that select the kernels one by one. It measures on native hosts
   only, never under an emulator such as Rosetta 2. CI's bench job runs it on
   Linux x86-64 and ARM64 and keeps the record as an artifact; on shared
   runners the ratios are comparable and the absolute times only roughly.

## Consequences

The first usable decoder is portable and slower than libjpeg-turbo with SIMD.
The x86-64 rewrite carries a performance risk against hand-written assembly; it
is bounded by the share of decode time in SIMD code and is measured per kernel.

## Review triggers

Revisit when the portable decoder passes its gates, when an intrinsics kernel
cannot reach the assembly's speed
([ferrixel#74](https://github.com/sebastian-software/ferrixel/issues/74), formerly libjpeg-turbo-rs#17), or
when upstream changes its SIMD layout.
