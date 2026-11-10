# Translate the SIMD variants in unsafe-allowed modules behind runtime selection

- Status: proposed
- Created: 2026-11-09
- Updated: 2026-11-10

## Context

[ADR 0005](0005-toolchain-and-unsafe-policy.md) denies `unsafe` outside the C
ABI module and requires a record that names the modules, the operations and
the safety arguments before the SIMD packages start. Wave 4's WP18 (x86
decoder), WP19 (x86 encoder) and WP20 (NEON) translate upstream's `*_sse2.c`,
`*_sse41.c`, `*_avx2.c` and `*_neon.c` files with `common_sse2.h`,
`common_sse41.h`, `neon.h`, `sharpyuv_sse2.c` and `sharpyuv_neon.c`: 15,015
lines of intrinsics. In Rust they are `core::arch` intrinsics. Since Rust 1.87
an intrinsic is safe to call inside a function marked
`#[target_feature(enable = "...")]` for its feature, or anywhere when the
target enables the feature; the loads and stores through pointers
(`_mm_loadu_si128`, `_mm_storeu_si128`, `vld1q_u8`, `vst1q_u8` and their
relatives) stay `unsafe`, and so does calling a `#[target_feature]` function
from a context that does not have the feature. The minimum Rust version 1.95
of ADR 0005 covers both rules.

Upstream selects the variants at run time: each `Init` function
(`VP8DspInit`, `VP8EncDspInit`, `VP8LDspInit`, `WebPInitUpsamplers` and the
others) assigns function pointers once per value of the global query pointer
`VP8GetCPUInfo` (`WEBP_DSP_INIT` re-runs it when the pointer changed), picking
SSE2, then SSE4.1, then AVX2 variants where the query answers `kSSE2`,
`kSSE4_1`, `kAVX2`; the tools set `VP8GetCPUInfo = NULL` for `-noasm`. On
AArch64 upstream compiles the NEON code in unconditionally and omits the C
code it replaces (`WEBP_NEON_OMIT_C_CODE`), so `-noasm` leaves NEON active
there. [ADR 0003](0003-rust-representation.md) turned every dispatch pointer
into a plain function, ready for selection inside, and WP4 translated
`VP8GetCPUInfo` as a query that the C ABI exposes as the pointer upstream's
tools assign ([ADR 0004](0004-c-abi-boundary.md)). The oracle of
[ADR 0002](0002-differential-oracle.md) is built with `WEBP_ENABLE_SIMD=OFF`;
upstream's own `test_cwebp.sh` compares its SIMD and `-noasm` outputs by
checksum, so the SIMD variants are bit-exact with the C code they replace, and
Ferrixel's port plan admits each variant only with exact parity and a reproducible
benchmark.

## Decision

**Modules.** The SIMD modules are the ones `tools/units.json` assigns to WP18,
WP19 and WP20: `src/dsp/common_sse2.rs`, `common_sse41.rs`, the `*_sse2.rs`,
`*_sse41.rs` and `*_avx2.rs` modules of `src/dsp`, `src/sharpyuv/sharpyuv_sse2.rs`
(x86 and x86-64), and `src/dsp/neon.rs`, the `*_neon.rs` modules of `src/dsp`
and `src/sharpyuv/sharpyuv_neon.rs` (AArch64). Each is compiled only with the
Cargo feature `simd` on its architecture (`all(feature = "simd", <arch>)` in its
`cfg`, written into `tools/units.json` and generated into the `mod.rs` files by
`tools/units.py`), and each carries `#[allow(unsafe_code, reason = "ADR 0008:
...")]` on its `pub mod` line, generated from the `unsafe` field of its
manifest entry; the lines after it deny `clippy::undocumented_unsafe_blocks`
and `clippy::multiple_unsafe_ops_per_block`, as `src/abi/mod.rs` does for the
ABI.
`src/lib.rs` keeps `#![deny(unsafe_code)]`; no other module is relaxed.

**Operations.** Inside a SIMD module exactly two kinds of operation may be
`unsafe`, each in its own `unsafe` block with a `// SAFETY:` comment:

1. A vector load or store through a pointer (`_mm_loadu_si128`,
   `_mm_loadl_epi64`, `_mm_storeu_si128`, `_mm_storel_epi64`, `vld1q_u8`,
   `vst1q_u8` and the like) on a slice whose length the code has checked in
   the lines before, with the comment naming that check; the slice, or the
   `chunks_exact` iterator it comes from, is the bound. Unaligned forms only:
   a Rust slice promises no alignment, so upstream's aligned loads become
   unaligned loads, with no effect on the result (a deviation class each
   package records once).
2. The call of a `#[target_feature(enable = "...")]` function from the
   module's entry point, after the selection described below has established
   the feature, with the comment naming the selection. The entry points are
   the only functions a dispatch function calls.

Everything else stays safe: the intrinsics inside `#[target_feature]`
functions, the arithmetic, the shuffles and the casts between vector types
through the cast intrinsics. No raw pointer arithmetic (offsets are slice
indices), no `transmute`, no `static mut`, no `asm!`, no `unsafe impl`. Where
upstream reads or writes a few bytes through a vector load for convenience,
`from_le_bytes`/`to_le_bytes` on a sub-slice is preferred to an `unsafe`
load. The functions keep upstream's names and `_SSE2`, `_SSE41`, `_AVX2`,
`_NEON` suffixes under the naming rule of ADR 0001, and the module translates
the whole upstream file including its `Init` function's body, which becomes
the list of the dispatch functions it serves (documentation, not code).

**Feature flag.** The Cargo feature `simd` is in `default`. With
`--no-default-features` the library is the scalar translation that the
reference oracle corresponds to; every test must pass in both builds
(`tools/check.py --no-simd` runs the test stage that way). The `oracle`
feature of ADR 0002 is unchanged.

**Runtime selection.** `webp::dsp::cpu::features()` returns the `Features`
record (`sse2`, `sse3`, `slow_ssse3`, `sse4_1`, `avx`, `avx2`, `neon`) that
the dispatch functions select by. It is computed from the query behind the C
ABI's `VP8GetCPUInfo` pointer, as the `Init` functions compute their choices:
a `NULL` pointer answers no x86 feature (`-noasm`), the translation's own
query answers from `cpuid`, and a query a caller installed answers for
itself; `neon` is true on AArch64 whatever the pointer says, as upstream
compiles NEON in there. With `simd` on AArch64, `vp8_get_cpu_info` answers
`kNEON` as upstream's `armCPUInfo` does in a NEON build, and the C ABI's
pointer is that query rather than `NULL` (WP20 makes both changes; the
reference-configuration answers stay in the scalar build). The record is
cached in one atomic word together with
a token of the pointer it was computed for and recomputed when the pointer
changed, which is `WEBP_DSP_INIT`'s rule: a change of `VP8GetCPUInfo` takes
effect at the next call, not in the middle of one. The dispatch functions of
the portable modules (`src/dsp/dec.rs`, `enc.rs`, `lossless.rs`,
`lossless_enc.rs`, `upsampling.rs`, `yuv.rs`, `alpha_processing.rs`,
`filters.rs`, `rescaler.rs`, `cost.rs`, `ssim.rs`, `quant.rs`,
`src/sharpyuv/sharpyuv_dsp.rs`) gain, under `cfg(feature = "simd")` and the
architecture, the selection that upstream's `Init` functions establish for
that pointer: the variant of the last `Init` branch that assigns it (AVX2
over SSE4.1 over SSE2 over the C function on x86, in the order
`VP8DspInit` and its siblings run them; NEON over the C function on AArch64),
each consulting `features()` and calling the SIMD module's entry point. A
SIMD package makes these additions in the portable modules without a further
integrator decision; they change no signature and no scalar result. Function
tables (`VP8PredLuma4` and the like) select per entry the same way.

**Parity.** A SIMD function is admitted only with all of:

1. a Rust-against-Rust parity test of the function against the `_C`
   translation it replaces, over seeded inputs that cover every length and
   shape the function accepts, unaligned offsets into the buffers, and the
   saturation and sign edges of its arithmetic, compiled with `simd` so that
   both are present (`tests/wp18_*.rs`, `wp19_*.rs`, `wp20_*.rs`);
2. the whole differential suite against the reference oracle with `simd` on,
   which CI already runs: every output stays byte-identical to C;
3. the whole suite in the scalar build (`--no-simd`), which CI runs without
   the oracle on both systems and against the release oracle on Linux;
4. the diagnostic oracle variant `release-simd` (`tools/build_oracle.py
   --variant release-simd`, upstream with `WEBP_ENABLE_SIMD=ON`), against
   which the package runs the DSP probes of WP4, WP6, WP7, WP10 and WP11,
   whose C side then calls upstream's SIMD functions through the dispatch
   pointers, and its own tests; it is run locally and recorded on the status
   page, not in CI, and it attributes any difference to upstream's SIMD or to
   the port;
5. upstream's test scripts through the Rust-linked tools, which compare the
   default and `-noasm` runs by checksum (the existing baseline), and
   `tools/unit_dependencies.py --check --variant release-simd`, which sees
   the SIMD units' references.

**Benchmark.** `examples/dsp_bench` is the harness: no dependency, seeded
inputs, a fixed number of calls per measurement and the median of repeated
measurements, printing nanoseconds per call for the C translation and each
SIMD level the host has. Each package registers its module's cases there and
records the table (host, CPU, `rustc` version) in its pull request. A variant
slower than the C translation is recorded, not removed or tuned: this is the
mechanical port; optimization is Ferrixel's later phase.

## Consequences

The library keeps `#![deny(unsafe_code)]` and a plain `cargo build`; the SIMD
modules add `unsafe` of two kinds only, each argued locally, and
`--no-default-features` builds the proven scalar translation. Callers see no
new API besides `features()` and the feature flag; the C ABI's `-noasm`
contract holds through the pointer. CI grows by the scalar test passes. The
SIMD packages touch the portable modules' dispatch functions, so WP18 and WP20
can conflict there; the second to merge rebases, and the integrator checks
the merged tree as for every pair.

## Review triggers

Revisit when a SIMD function cannot be expressed with the two operations
above, when Rust stabilizes a portable SIMD type that could replace the
per-architecture modules, when the minimum Rust version is raised, or when
Ferrixel's measurements ask for tuning beyond the mechanical port.
