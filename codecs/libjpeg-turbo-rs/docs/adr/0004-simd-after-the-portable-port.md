# Port SIMD after the portable decoder, as intrinsics on both architectures

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-12

## Context

libjpeg-turbo's speed comes partly from SIMD for the IDCT, upsampling and color
conversion (`simd/`). On AArch64 that code is C with NEON intrinsics (about 7k
lines in `simd/arm`); upstream replaced its former GAS assembly with it and
reports it as faster in several cases. On x86-64 it is NASM assembly (about 14k
lines in `simd/x86_64`) for SSE2 and AVX2. The serial Huffman decoder has no
SIMD and dominates decode time on photographs (Ferrisma's measurement
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

## Amendment 2026-11-12: inline the Huffman fast path

The serial Huffman decoder, not the SIMD kernels, held most of the remaining
gap to libjpeg-turbo ([engine performance
analysis](https://github.com/sebastian-software/ferrisma/blob/main/docs/benchmarks/v1.0-engine-performance-analysis-2026-11-12.md),
issues [ferrisma#76](https://github.com/sebastian-software/ferrisma/issues/76)
and [ferrisma#221](https://github.com/sebastian-software/ferrisma/issues/221)).
Upstream's `HUFF_DECODE_FAST` is a macro; the port's function of that name
takes the bit-buffer state by `&mut` and carried `#[inline]`, which LLVM did
not follow, so the state lived in memory on every symbol. It is now
`#[inline(always)]` (with a scoped `clippy::inline_always` expectation). No
other code changed, and no `unsafe` was added.

Correctness: `tools/bench.py`'s cases produce the same checksums in C, the
previous port and the inlined port; upstream's in-scope bit tests pass against
the `release` and `simd` oracles (79 of 79 each), and the differential tests
pass with `LIBJPEG_ORACLE_DIR` set for the `release` oracle without default
features and for the `simd` oracle in the `neon` and `none` modes. The T.83
conformance tests did not run: their data is downloaded from ITU and was not
fetched for this change.

Speed, measured with `tools/bench.py`'s inputs (3632×2384 tiled photo, 5
decodes per run) and the `simd` oracle's `wp9/bench`, 7 interleaved runs
alternating the order of C, the previous port and the inlined port, medians.
Apple M1 Ultra, macOS 27.0.1. **Indicative; loaded host** (1-minute load
average 13.9–19.4, other builds running):

| case | before ms | after ms | C ms | after / before | before / C | after / C |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 40.82 | 29.16 | 25.87 | 0.71 | 1.58 | 1.13 |
| baseline 4:2:2, RGB | 47.52 | 34.42 | 30.18 | 0.72 | 1.57 | 1.14 |
| baseline 4:4:4, RGB | 58.90 | 42.82 | 36.34 | 0.73 | 1.62 | 1.18 |
| progressive 4:2:0, RGB | 96.20 | 95.96 | 64.95 | 1.00 | 1.48 | 1.48 |
| baseline 4:2:0, RGBX | 40.69 | 29.40 | 26.19 | 0.72 | 1.55 | 1.12 |
| baseline 4:2:0, fast IDCT | 38.39 | 27.01 | 24.77 | 0.70 | 1.55 | 1.09 |
| baseline 4:2:0, float IDCT | 50.48 | 39.02 | 37.12 | 0.77 | 1.36 | 1.05 |
| baseline 4:2:0, merged upsampling | 38.71 | 27.19 | 24.44 | 0.70 | 1.58 | 1.11 |
| baseline 4:2:2, merged upsampling | 46.53 | 33.27 | 29.27 | 0.71 | 1.59 | 1.14 |
| baseline 4:2:0, scale 1/2 | 35.02 | 23.50 | 21.64 | 0.67 | 1.62 | 1.09 |
| baseline 4:2:0, scale 1/4 | 32.44 | 21.04 | 19.79 | 0.65 | 1.64 | 1.06 |
| baseline 4:2:0, scale 1/8 | 31.91 | 19.20 | 18.01 | 0.60 | 1.77 | 1.07 |
| testorig.jpg (227×149) | 0.12 | 0.11 | 0.08 | 0.91 | 1.51 | 1.37 |

Baseline (sequential Huffman) decodes are 23–40 % faster and 1.05–1.18× C.
Progressive decoding uses the separate progressive entropy decoder and is
unchanged at 1.48× C; it is the next Huffman target. The remaining baseline
gap includes the IDCT pass-2 helpers, which `#[target_feature]` keeps out of
line. The script and raw runs are in the analysis's evidence directory
(`jpeg_ab.py`, `jpeg-ab-inline.json`). An idle-host and CI `bench` run should
confirm the ratios.

## Review triggers

Revisit when the portable decoder passes its gates, when an intrinsics kernel
cannot reach the assembly's speed
([ferrisma#74](https://github.com/sebastian-software/ferrisma/issues/74), formerly libjpeg-turbo-rs#17), or
when upstream changes its SIMD layout.
