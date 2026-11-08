# Dispatch hand-translated x86 SIMD on GNU Linux

This extends [ADR 0056](0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md)
to x86-64 GNU Linux. Everything ADR 0056 says about dispatch, kernels and
exactness applies; this ADR records only what differs on x86.

## Context

ADR 0056 added Neon kernels on macOS ARM64 and left GNU Linux on the generic
headers "until AVX2 kernels exist". The translated Linux encoder is therefore
entirely scalar. Ferrixel's consumer measurements show the Neon build of the
same encoder 21 to 40 % faster than `rav1e`; on x86-64 that gap does not exist
yet, and x86 servers are where Sharp-style pipelines usually run.

C2Rust 0.21.0 cannot translate the x86 kernels mechanically. Translating
`aom_dsp/x86/sad_avx2.c` succeeds but silently drops every vector statement
(the SAD loop body is gone), and `highbd_inv_txfm_avx2.c` aborts on calls with
vector arguments. The x86 files are therefore translated by hand under the
same rule as the Neon files.

## Kernel set

The scalar code is the same on both architectures, so ADR 0056's profile still
selects the work: forward and inverse transforms, CDEF, quantization, block
error, subtraction and intra prediction. The x86 set is the x86 counterpart of
the Neon set, function by function:

- For each function, the highest upstream variant up to AVX2 is translated,
  plus every lower variant that it calls internally (the AVX2 transforms
  delegate several sizes to SSE4.1). Many functions have no AVX2 variant; their
  best one is SSE4.1 (`intra_edge_sse4.c`, `filterintra_sse4.c`, 13 forward
  transform sizes) or SSE2 (`highbd_intrapred_sse2.c`,
  `highbd_subtract_sse2.c`, `aom_highbd_quantize_b_64x64_adaptive`).
- Specializations implemented only in NASM files are not translated: hand
  translation keeps an intrinsic sequence, and assembly has none. This leaves
  the high-bitdepth DC and V predictors for 4x4, 8x8, 16x16 and 32x32
  (`highbd_intrapred_asm_sse2.asm`) on `_c`.
- AVX-512 is not included. libaom enables it only on CPUs with AVX512-DL, the
  standard runners do not guarantee such CPUs, and its only overlap with this
  set is the high-bitdepth forward transform. It needs its own measurement.
- High-bitdepth smooth and Paeth predictors have no x86 specialization and stay
  on `_c`.

## Dispatch

`tools/transpile_aom.py` generates the Linux dispatch headers with the original
`cmake/rtcd.pl --arch=x86_64 --require-c`, every family other than SSE2, SSE4.1
and AVX2 disabled, and the port overlay `crates/aom/rtcd/x86_64.pl`, which
withdraws every specialization not in the set above. With `--require-c` the
generator dispatches SSE2 through a pointer as well instead of assuming the
x86-64 baseline, which keeps every `_c` path selectable.

The generated `setup_rtcd_internal()` calls `x86_simd_caps()`, a static inline
function in `aom_ports/x86.h`, so it is translated into the RTCD units rather
than written by hand as `aom_arm_cpu_caps()` is. C2Rust's CPUID lowering has the
BX-register defect described in [ADR 0023](0023-preserve-the-x86-cpu-query-branch-with-simd-disabled.md);
the same narrow correction replaces only that lowering with
`core::arch::x86_64::__cpuid_count` for the same leaf and subleaf. The XGETBV
instruction, the AVX and AVX-512 guards, and the `AOM_SIMD_CAPS` and
`AOM_SIMD_CAPS_MASK` overrides keep their translated form.

## Kernels

Each original x86 file is translated into
`crates/aom/src/simd/<same path>.rs` (for example
`av1/encoder/x86/error_intrin_avx2.rs`), with the ADR 0056 rules for names,
helpers, comments, constants and intrinsic order, compiled only for
`target_arch = "x86_64"`.

Upstream compiles each x86 file with the flag its suffix names (`-msse2`,
`-msse4.1`, `-mavx2`). The Rust crate is compiled for the x86-64 baseline, so
every translated function carries `#[target_feature(enable = "...")]` for the
flag of the file it comes from. Helpers translated from shared headers carry
the smallest feature that their own intrinsics need; Rust inlines them into
callers with a superset of features, as the C compiler does. Functions are never
called without a feature check: only RTCD after `x86_simd_caps()` and tests
after `is_x86_feature_detected!` reach them.

## Evidence

The development machine is ARM64 and runs no x86 code, and no emulation is
used. x86 evidence therefore comes from native x86-64 CI runners, consistent
with [ADR 0055](0055-gate-pull-requests-on-linux-ci-and-local-macos-checks.md):

1. Locally, `cargo check --target x86_64-unknown-linux-gnu` for the crate and
   its SIMD tests.
2. On Linux CI, the supplementary transcriptions in `crates/aom/tests/simd` for
   each x86 instantiation of the original tests (SSE2, SSE4.1 and AVX2
   parameter lists), compared exactly against `_c` as in ADR 0056.
3. On Linux CI, byte-identical AVIF output for the identity corpus with
   detection enabled, with `AOM_SIMD_CAPS_MASK=0`, and with SSE4.1 as the
   highest family, so that the SSE4.1 and SSE2 paths that an AVX2 machine would
   not select are covered end to end.

## Consequences

The Linux profiles are regenerated by `aom-translate-linux` with dispatch
headers and imported with their `rtcd_dispatch` record; ABI, linkage and
ownership inventories are recorded again. Because a translation run takes about
three hours, the kernel set is completed before the Linux profiles are
regenerated, and later changes to the set regenerate them again. The macOS
profiles and the portable C reference are unchanged.
